use sqlx::PgPool;

use crate::services::normalization::{NormalizationPipeline, NormalizationPipelineError};
use crate::services::organization_write::{OrganizationWriteError, OrganizationWriteService};
use crate::services::project_write::{ProjectWriteError, ProjectWriteService};
use crate::services::reference_resolution::{ReferenceResolutionError, ReferenceResolutionService};
use crate::services::sync_runs::{SyncRunRecord, SyncRunRepository, SyncRunRepositoryError};
use crate::services::work_item_write::{WorkItemWriteError, WorkItemWriteService};
use crate::services::workforce_write::{WorkforceWriteError, WorkforceWriteService};

#[derive(Debug)]
pub enum PushIngestionProcessorError {
    Normalization(NormalizationPipelineError),
    ReferenceResolution(ReferenceResolutionError),
    OrganizationWrite(OrganizationWriteError),
    WorkforceWrite(WorkforceWriteError),
    ProjectWrite(ProjectWriteError),
    WorkItemWrite(WorkItemWriteError),
    SyncRun(SyncRunRepositoryError),
}

impl std::fmt::Display for PushIngestionProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normalization(error) => write!(f, "normalization error: {error}"),
            Self::ReferenceResolution(error) => write!(f, "reference resolution error: {error}"),
            Self::OrganizationWrite(error) => write!(f, "organization write error: {error}"),
            Self::WorkforceWrite(error) => write!(f, "workforce write error: {error}"),
            Self::ProjectWrite(error) => write!(f, "project write error: {error}"),
            Self::WorkItemWrite(error) => write!(f, "work item write error: {error}"),
            Self::SyncRun(error) => write!(f, "sync run error: {error}"),
        }
    }
}

impl std::error::Error for PushIngestionProcessorError {}

impl From<NormalizationPipelineError> for PushIngestionProcessorError {
    fn from(error: NormalizationPipelineError) -> Self {
        Self::Normalization(error)
    }
}

impl From<ReferenceResolutionError> for PushIngestionProcessorError {
    fn from(error: ReferenceResolutionError) -> Self {
        Self::ReferenceResolution(error)
    }
}

impl From<OrganizationWriteError> for PushIngestionProcessorError {
    fn from(error: OrganizationWriteError) -> Self {
        Self::OrganizationWrite(error)
    }
}

impl From<WorkforceWriteError> for PushIngestionProcessorError {
    fn from(error: WorkforceWriteError) -> Self {
        Self::WorkforceWrite(error)
    }
}

impl From<ProjectWriteError> for PushIngestionProcessorError {
    fn from(error: ProjectWriteError) -> Self {
        Self::ProjectWrite(error)
    }
}

impl From<WorkItemWriteError> for PushIngestionProcessorError {
    fn from(error: WorkItemWriteError) -> Self {
        Self::WorkItemWrite(error)
    }
}

impl From<SyncRunRepositoryError> for PushIngestionProcessorError {
    fn from(error: SyncRunRepositoryError) -> Self {
        Self::SyncRun(error)
    }
}

pub struct PushIngestionProcessor {
    pool: PgPool,
    normalization_pipeline: NormalizationPipeline,
    reference_resolution_service: ReferenceResolutionService,
    organization_write_service: OrganizationWriteService,
    workforce_write_service: WorkforceWriteService,
    project_write_service: ProjectWriteService,
    work_item_write_service: WorkItemWriteService,
    sync_run_repository: SyncRunRepository,
}

impl PushIngestionProcessor {
    pub fn new(pool: PgPool) -> Self {
        Self {
            normalization_pipeline: NormalizationPipeline::new(pool.clone()),
            reference_resolution_service: ReferenceResolutionService::new(pool.clone()),
            organization_write_service: OrganizationWriteService::new(pool.clone()),
            workforce_write_service: WorkforceWriteService::new(pool.clone()),
            project_write_service: ProjectWriteService::new(pool.clone()),
            work_item_write_service: WorkItemWriteService::new(pool.clone()),
            sync_run_repository: SyncRunRepository::new(pool.clone()),
            pool,
        }
    }

    pub async fn process_run(
        &self,
        run_id: &str,
        accepted_count: i32,
    ) -> Result<SyncRunRecord, PushIngestionProcessorError> {
        if accepted_count <= 0 {
            return self
                .sync_run_repository
                .get(run_id)
                .await?
                .ok_or(SyncRunRepositoryError::NotFound)
                .map_err(Into::into);
        }

        for _ in 0..accepted_count {
            self.normalization_pipeline
                .normalize_pending_for_run(run_id, i64::from(accepted_count))
                .await?;

            let resolution_result = self
                .reference_resolution_service
                .resolve_pending_references_for_run(run_id, i64::from(accepted_count))
                .await?;

            if resolution_result.resolved_count == 0 {
                break;
            }
        }

        let run_internal_id = sqlx::query_scalar::<_, uuid::Uuid>(
            "select integration_run_id from integration_run where external_run_id = $1",
        )
        .bind(run_id)
        .fetch_one(&self.pool)
        .await
        .map_err(SyncRunRepositoryError::Database)?;

        self.organization_write_service
            .apply_for_run(run_internal_id)
            .await?;
        self.workforce_write_service
            .apply_for_run(run_internal_id)
            .await?;
        self.project_write_service
            .apply_for_run(run_internal_id)
            .await?;
        self.work_item_write_service
            .apply_for_run(run_internal_id)
            .await?;

        self.sync_run_repository
            .complete_push_run(run_id, accepted_count, accepted_count, 0)
            .await
            .map_err(Into::into)
    }
}
