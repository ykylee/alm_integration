# 조직 관리 UI/UX 레퍼런스 조사 및 재설계 기준

- 문서 목적: 시스템 관리자용 조직 관리 UI 를 재설계하기 위해, 실제 제품 레퍼런스에서 공통 패턴을 추출하고 우리 프로토타입에 적용할 기준을 정리한다.
- 범위: 웹 레퍼런스 조사 결과, 공통 UI 패턴, 권장 사용자 시나리오, 프로토타입 재설계 기준
- 대상 독자: 기획자, 디자이너, 프론트엔드 개발자, 백엔드 개발자, 시스템 관리자
- 상태: draft
- 최종 수정일: 2026-04-08
- 관련 문서: `docs/overview/integrated_data_management_ui_direction.md`, `docs/overview/ui_ux_prototype.md`

## 문서 위치

- 위키 홈: [../README.md](../README.md)
- 개요 위키: [./README.md](./README.md)
- 관련 문서: [./integrated_data_management_ui_direction.md](./integrated_data_management_ui_direction.md)

## 1. 조사 목적

현재 조직 관리 프로토타입은 기능 연결은 빠르게 확인할 수 있지만, 실제 시스템 관리자용 작업면처럼 느껴지기에는 정보 계층과 작업 흐름이 아직 평평하다. 따라서 “조직 구조 관리 + 구성원 조작 + 감사 추적”이 함께 필요한 관리형 UI 를 참고해 공통 패턴을 추출하고, 우리 프로토타입을 그 기준으로 재구성한다.

## 2. 참고한 레퍼런스

### 2.1 Atlassian Administration

- 그룹 관리 기준: [Manage groups | Atlassian Support](https://support.atlassian.com/user-management/docs/manage-groups/)
- 감사 로그 기준: [View audit log activities | Atlassian Support](https://support.atlassian.com/security-and-access-policies/docs/view-audit-log-activities/)

관찰 포인트:

- 그룹과 권한 관리를 목록 중심으로 다룬다.
- 관리 액션을 `create`, `edit`, `delete` 로 분리하고 멤버와 역할을 같은 문맥에서 다룬다.
- 감사 로그는 별도 메뉴이지만 실제 운영에서는 같은 관리 콘솔 문맥에서 필수 보조면으로 작동한다.

### 2.2 Microsoft Entra / Microsoft 365 Admin

- 사용자·그룹 관리 학습 모듈: [Manage users and groups in Microsoft Entra ID | Microsoft Learn](https://learn.microsoft.com/en-us/training/modules/manage-users-and-groups-in-aad/)
- 중앙 관리자 허브 관점: [Manage Microsoft 365 with the Admin app in Microsoft Teams | Microsoft Learn](https://learn.microsoft.com/en-us/microsoftteams/manage-admin-app)
- 관리자 센터 기능 범위: [Use the Microsoft 365 admin center to manage your subscription | Microsoft Learn](https://learn.microsoft.com/en-us/power-platform/admin/use-office-365-admin-center-manage-subscription)

관찰 포인트:

- 좌측 내비게이션과 상단 명령 바가 분리돼 있다.
- 관리 대상의 목록, 상세, 역할/권한 조작이 명확히 분리된다.
- 중앙 허브는 있더라도 실제 실무 흐름은 특정 관리 대상 화면 안에서 끝난다.

### 2.3 Okta Lifecycle Management

- 제품 개요: [Lifecycle Management and App Provisioning Software | Okta](https://www.okta.com/products/lifecycle-management/)

관찰 포인트:

- `joiner / mover / leaver` 처럼 운영 시나리오 중심 용어가 강하다.
- `single plane` 또는 한 작업면에서 사용자, 정책, 감사, 앱 접근까지 이어지는 경험을 강조한다.
- 조직/인력 관리에서 “이동”과 “비활성화”는 일급 액션으로 다뤄진다.

### 2.4 IBM Governance and Catalog

- 제품 개요: [Governance and Catalog | IBM](https://www.ibm.com/products/watsonx-data-intelligence/governance-catalog)

관찰 포인트:

- 데이터 거버넌스 화면은 단순 목록보다 `정책`, `품질`, `감사`, `관계`를 한 화면군 안에서 설명한다.
- 카탈로그형 UI 라도 실제 작업은 선택 상세와 관계/영향 분석 패널이 함께 있어야 한다.
- 관리 화면은 “무엇을 바꿀 수 있는가” 못지않게 “이 변경이 어디에 영향을 주는가”를 강조한다.

## 3. 공통 패턴 정리

조사 결과, 우리 컨셉에 맞는 공통 패턴은 다음과 같다.

- 좌측에는 안정적인 관리 대상 탐색을 둔다.
- 상단에는 현재 화면의 성격, 권한 수준, API 연결 상태, 주요 실행 버튼을 둔다.
- 본문은 `목록/트리`, `선택 상세`, `액션` 으로 최소 3분할하는 편이 실제 운영에 가깝다.
- 감사 로그와 변경 이력은 별도 페이지보다 현재 작업면의 보조 패널로 배치하는 편이 효율적이다.
- 조직 관리에서는 `조직 구조`, `구성원`, `영향 범위`가 동시에 보여야 삭제/이동 판단이 쉽다.
- 멤버 관리 액션은 인력 화면에만 두기보다 조직 상세 작업면에서도 바로 실행 가능해야 한다.
- 조직 선택 후 상위 경로, 직속 하위 조직, 하위 포함 인원 규모가 즉시 보여야 계층 변경 판단이 쉬워진다.

## 4. 우리 시스템에 맞는 권장 사용자 시나리오

### 4.1 조직 생성 또는 수정

1. 좌측 조직 디렉터리에서 현재 구조를 확인한다.
2. 중앙 선택 상세에서 상위 경로와 직속 하위 조직을 확인한다.
3. 우측 액션 레일에서 조직 코드, 이름, 상위 조직, 상태를 수정한다.
4. 하단 이력 패널에서 변경 결과와 최근 이동 내역을 확인한다.

### 4.2 조직 이동 영향 확인

1. 트리에서 대상 조직을 선택한다.
2. 구조 스냅샷에서 상위 경로와 하위 포함 조직 수를 확인한다.
3. 직속 구성원/하위 포함 구성원 규모를 확인한다.
4. 변경 액션 전에 영향 범위 패널과 최근 이력을 검토한다.

### 4.3 구성원 이동 또는 비활성화

1. 조직을 선택하고 직속 구성원 테이블에서 구성원을 고른다.
2. 우측 `joiner / mover / leaver` 성격의 액션 영역에서 이동 대상 조직 또는 비활성화를 실행한다.
3. 하단 구성원 이동 이력에서 결과를 확인한다.

## 5. 재설계 기준

이번 프로토타입 재설계에서는 아래 원칙을 따른다.

- 조직 관리 화면을 `Directory + Inspector + Action Rail` 구조로 재배치한다.
- 트리와 목록은 좌측 디렉터리 영역으로 묶는다.
- 중앙에는 선택 조직의 구조 스냅샷, 상세, 직속 구성원, 영향 요약을 둔다.
- 우측에는 조직 액션과 구성원 액션을 분리한 작업 레일을 둔다.
- 하단에는 조직 변경 이력과 구성원 이동 이력을 감사 패널처럼 배치한다.
- 색과 스타일은 기존 데모 감성보다 차분한 `enterprise control room` 방향으로 조정한다.
- 카드형 통계, 상태 배지, 모노스페이스 코드, 얕은 유리 효과는 유지하되 장식성을 줄이고 정보 밀도를 높인다.

## 6. 이번 재설계의 적용 범위

이번 단계의 직접 적용 범위는 다음과 같다.

- `src/ui_prototype/data_organizations.html`
- `src/ui_prototype/styles.css`

향후 같은 패턴을 아래 화면으로 확장한다.

- `src/ui_prototype/data_workforce.html`
- 외부 시스템별 데이터 관리 화면
- 자체 `ALM` 데이터 관리 화면
