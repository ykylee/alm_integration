use crate::adapters::{AdapterError, AdapterRegistry, PullAdapterRequest};
use sqlx::PgPool;
use uuid::Uuid;

use crate::services::normalization::{NormalizationPipeline, NormalizationPipelineError};
use crate::services::project_write::{ProjectWriteError, ProjectWriteService};
use crate::services::raw_ingestion::{
    CreateRawIngestionEventInput, RawIngestionRepository, RawIngestionRepositoryError,
};
use crate::services::reference_resolution::{ReferenceResolutionError, ReferenceResolutionService};
use crate::services::sync_runs::{
    CreateSyncRunInput, SyncRunRecord, SyncRunRepository, SyncRunRepositoryError,
};
use crate::services::work_item_write::{WorkItemWriteError, WorkItemWriteService};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PullRecordInput {
    pub source_object_type: String,
    pub source_object_id: String,
    pub source_event_key: String,
    pub source_version: Option<String>,
    pub source_updated_at: Option<String>,
    pub payload: serde_json::Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PullSyncRunInput {
    pub source_system: String,
    pub mode: String,
    pub scope: serde_json::Value,
    pub reason: Option<String>,
    pub records: Vec<PullRecordInput>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum PullSyncOrchestratorError {
    SyncRun(SyncRunRepositoryError),
    RawIngestion(RawIngestionRepositoryError),
    Normalization(NormalizationPipelineError),
    ReferenceResolution(ReferenceResolutionError),
    ProjectWrite(ProjectWriteError),
    WorkItemWrite(WorkItemWriteError),
    Adapter(AdapterError),
}

impl std::fmt::Display for PullSyncOrchestratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyncRun(error) => write!(f, "sync run error: {error}"),
            Self::RawIngestion(error) => write!(f, "raw ingestion error: {error}"),
            Self::Normalization(error) => write!(f, "normalization error: {error}"),
            Self::ReferenceResolution(error) => write!(f, "reference resolution error: {error}"),
            Self::ProjectWrite(error) => write!(f, "project write error: {error}"),
            Self::WorkItemWrite(error) => write!(f, "work item write error: {error}"),
            Self::Adapter(error) => write!(f, "adapter error: {error}"),
        }
    }
}

impl std::error::Error for PullSyncOrchestratorError {}

impl From<SyncRunRepositoryError> for PullSyncOrchestratorError {
    fn from(error: SyncRunRepositoryError) -> Self {
        Self::SyncRun(error)
    }
}

impl From<RawIngestionRepositoryError> for PullSyncOrchestratorError {
    fn from(error: RawIngestionRepositoryError) -> Self {
        Self::RawIngestion(error)
    }
}

impl From<NormalizationPipelineError> for PullSyncOrchestratorError {
    fn from(error: NormalizationPipelineError) -> Self {
        Self::Normalization(error)
    }
}

impl From<ReferenceResolutionError> for PullSyncOrchestratorError {
    fn from(error: ReferenceResolutionError) -> Self {
        Self::ReferenceResolution(error)
    }
}

impl From<ProjectWriteError> for PullSyncOrchestratorError {
    fn from(error: ProjectWriteError) -> Self {
        Self::ProjectWrite(error)
    }
}

impl From<WorkItemWriteError> for PullSyncOrchestratorError {
    fn from(error: WorkItemWriteError) -> Self {
        Self::WorkItemWrite(error)
    }
}

impl From<AdapterError> for PullSyncOrchestratorError {
    fn from(error: AdapterError) -> Self {
        Self::Adapter(error)
    }
}

#[allow(dead_code)]
pub struct PullSyncOrchestrator {
    pool: PgPool,
    sync_run_repository: SyncRunRepository,
    raw_ingestion_repository: RawIngestionRepository,
    normalization_pipeline: NormalizationPipeline,
    reference_resolution_service: ReferenceResolutionService,
    project_write_service: ProjectWriteService,
    work_item_write_service: WorkItemWriteService,
}

impl PullSyncOrchestrator {
    #[allow(dead_code)]
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool: pool.clone(),
            sync_run_repository: SyncRunRepository::new(pool.clone()),
            normalization_pipeline: NormalizationPipeline::new(pool.clone()),
            reference_resolution_service: ReferenceResolutionService::new(pool.clone()),
            project_write_service: ProjectWriteService::new(pool.clone()),
            work_item_write_service: WorkItemWriteService::new(pool.clone()),
            raw_ingestion_repository: RawIngestionRepository::new(pool),
        }
    }

    #[allow(dead_code)]
    pub async fn run(
        &self,
        input: PullSyncRunInput,
    ) -> Result<SyncRunRecord, PullSyncOrchestratorError> {
        let run = self
            .sync_run_repository
            .create(CreateSyncRunInput {
                source_system: input.source_system.clone(),
                mode: input.mode,
                scope: input.scope,
                reason: input.reason,
            })
            .await?;

        let mut success_count = 0;
        let mut failure_count = 0;

        for record in input.records {
            let result = self
                .raw_ingestion_repository
                .create_for_run(
                    &run.run_id,
                    CreateRawIngestionEventInput {
                        source_system: input.source_system.clone(),
                        source_object_type: record.source_object_type,
                        source_object_id: record.source_object_id,
                        source_event_key: record.source_event_key,
                        source_version: record.source_version,
                        source_updated_at: record.source_updated_at,
                        payload: record.payload,
                    },
                )
                .await;

            match result {
                Ok(accepted) if accepted.accepted => success_count += 1,
                Ok(_) => failure_count += 1,
                Err(_) => failure_count += 1,
            }
        }

        if success_count > 0 {
            for _ in 0..success_count {
                self.normalization_pipeline
                    .normalize_pending_for_run(&run.run_id, i64::from(success_count))
                    .await?;

                let resolution_result = self
                    .reference_resolution_service
                    .resolve_pending_references_for_run(&run.run_id, i64::from(success_count))
                    .await?;

                if resolution_result.resolved_count == 0 {
                    break;
                }
            }
        }

        let run_internal_id = sqlx::query_scalar::<_, Uuid>(
            "select integration_run_id from integration_run where external_run_id = $1",
        )
        .bind(&run.run_id)
        .fetch_one(&self.pool)
        .await
        .map_err(SyncRunRepositoryError::Database)?;

        self.project_write_service
            .apply_for_run(run_internal_id)
            .await?;
        self.work_item_write_service
            .apply_for_run(run_internal_id)
            .await?;

        self.sync_run_repository
            .complete_pull_run(
                &run.run_id,
                success_count + failure_count,
                success_count,
                failure_count,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(dead_code)]
    pub async fn run_from_adapter(
        &self,
        adapter_registry: &AdapterRegistry,
        input: PullSyncRequestInput,
    ) -> Result<SyncRunRecord, PullSyncOrchestratorError> {
        let adapter = adapter_registry
            .get_pull_adapter(&input.source_system)
            .ok_or_else(|| AdapterError::UnsupportedSourceSystem(input.source_system.clone()))?;
        let records = adapter
            .pull(PullAdapterRequest {
                mode: input.mode.clone(),
                scope: input.scope.clone(),
            })
            .await?;

        self.run(PullSyncRunInput {
            source_system: input.source_system,
            mode: input.mode,
            scope: input.scope,
            reason: input.reason,
            records,
        })
        .await
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PullSyncRequestInput {
    pub source_system: String,
    pub mode: String,
    pub scope: serde_json::Value,
    pub reason: Option<String>,
}
