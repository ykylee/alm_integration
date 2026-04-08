const currentScreen = document.body.dataset.screen || "overview";
const navItems = document.querySelectorAll(".nav-item");
const roleChips = document.querySelectorAll(".role-chip");

const roleContent = {
  developer: {
    name: "개발자",
    summary: "내 리뷰, 빌드, 테스트 보완이 먼저 보이는 작업 중심 홈",
    heroKicker: "Developer Home",
    heroTitle: "내 리뷰, 빌드 이상, 테스트 보완이 가장 먼저 보여야 한다.",
    heroDescription:
      "개발자는 시스템 설정보다 지금 손대야 할 PR, 실패 빌드, 누락 테스트를 가장 빠르게 확인해야 한다.",
    metrics: [
      ["내가 볼 PR", "7", "오늘 기준"],
      ["실패 빌드", "2", "조치 필요"],
      ["테스트 보완", "4", "AI 초안 가능"],
    ],
    priorities: [
      ["리뷰 대기 PR-392", "과제 워크스페이스 상세 페이지에서 diff와 리뷰 코멘트를 먼저 확인", "과제 워크스페이스"],
      ["빌드 재실행 확인", "품질 검증 상세 페이지에서 환경 차이와 로그 비교", "품질 검증"],
      ["단위테스트 보완", "변경 범위 기준 테스트 초안 생성 요청", "품질 검증"],
    ],
    actions: [
      ["리뷰 시작", "과제 워크스페이스에서 AI 리뷰 초안과 diff를 함께 열기"],
      ["실패 빌드 비교", "품질 검증 페이지에서 환경 차이와 로그 비교"],
      ["테스트 초안 생성", "QA 작업면에서 변경 범위 기준 테스트 초안 요청"],
    ],
    risks: [
      ["리뷰 지연", "핵심 리뷰어 2명 부재로 승인 지연 가능"],
      ["코딩룰 위반 의심", "null 처리 규칙 1건 재검토 필요"],
      ["테스트 누락", "결제 취소 경로 검증 케이스 없음"],
    ],
    access: [
      ["주요 권한", "과제 수행, 코드리뷰, 단위테스트 보조"],
      ["기본 필터", "내 과제, 내가 리뷰할 PR, 실패 빌드"],
      ["제한 영역", "조직 변경 반영, 권한 관리, 연계 재실행"],
    ],
  },
  "project-lead": {
    name: "프로젝트리더",
    summary: "일정, 위험, 승인 병목, 품질 게이트를 먼저 보는 운영 홈",
    heroKicker: "Project Lead Home",
    heroTitle: "진행률보다 병목과 승인 대기 항목이 먼저 보여야 한다.",
    heroDescription:
      "프로젝트리더는 현재 과제가 어디서 막혔는지와 누가 결정을 내려야 하는지를 빠르게 확인해야 한다.",
    metrics: [
      ["위험 과제", "3", "마일스톤 영향"],
      ["승인 대기", "5", "조직장/관리자 확인 필요"],
      ["릴리스 게이트", "2", "미충족"],
    ],
    priorities: [
      ["릴리스 일정 조정", "프로젝트 운영 상세 페이지에서 대체 일정안과 병목을 즉시 확인", "프로젝트 운영"],
      ["품질 게이트 해소", "품질 검증 상세 페이지에서 결함과 테스트 상태를 확인", "품질 검증"],
      ["조직 승인 요청", "조직 운영 상세 페이지에서 승인 대기와 가용성 영향 확인", "조직 운영"],
    ],
    actions: [
      ["승인 요청 전달", "조직장과 시스템 관리자에게 병목 전달"],
      ["일정 조정안 작성", "프로젝트 운영 페이지에서 배포 대체 시나리오 생성"],
      ["보고용 보드 정렬", "위험 과제와 승인 상태를 보고용 요약으로 재배치"],
    ],
    risks: [
      ["코드 동결 지연", "오픈 PR 2건이 마감 임박"],
      ["테스트 미완료", "회귀 시나리오 6건 남음"],
      ["리소스 부족", "주간 부재로 리뷰 모수 감소"],
    ],
    access: [
      ["주요 권한", "과제 조정, 상태 보고, 승인 요청 허브"],
      ["기본 필터", "위험 과제, 승인 대기, 지연 가능 일정"],
      ["제한 영역", "연계 설정, 권한 관리, 조직 마스터 반영"],
    ],
  },
  "org-head": {
    name: "조직장",
    summary: "조직 승인, 가용성, 과제 분포를 먼저 보는 조직 운영 홈",
    heroKicker: "Organization Head Home",
    heroTitle: "조직이 감당할 수 있는지와 무엇을 승인해야 하는지가 먼저 보여야 한다.",
    heroDescription:
      "조직장은 개별 구현 정보보다 조직 자원, 승인 책임, 부재 영향, 전환 요청을 먼저 봐야 한다.",
    metrics: [
      ["승인 대기", "5", "오늘 처리 필요"],
      ["핵심 인력 부재", "3", "다음 주"],
      ["조정 필요 과제", "4", "조직 영향 있음"],
    ],
    priorities: [
      ["관리 대상 전환", "조직 운영 상세 페이지에서 수용 가능성과 리소스 영향을 확인", "조직 운영"],
      ["승인 루트 갱신", "관리자 콘솔과 조직 운영 페이지의 영향 범위를 함께 확인", "관리자 콘솔"],
      ["리뷰 가용성 감소", "공유 캘린더 영향과 승인 일정 조정을 상세 페이지에서 확인", "조직 운영"],
    ],
    actions: [
      ["전환 승인 검토", "조직 수용 가능성과 리소스 영향 확인"],
      ["캘린더 충돌 조정", "부재와 배포 일정 충돌 재배치"],
      ["우선순위 재정렬", "조직 차원의 과제 중요도 재조정"],
    ],
    risks: [
      ["핵심 인력 공백", "리뷰 리드 1명과 QA 1명 동시 부재"],
      ["조직 변경 영향", "프로젝트 조직 정보 18건 갱신 필요"],
      ["승인 지연", "전환 요청 2건이 일정 병목 유발"],
    ],
    access: [
      ["주요 권한", "조직 단위 승인, 가용성 판단, 조직 현황 조회"],
      ["기본 필터", "조직별 활성 과제, 승인 요청, 부재 영향"],
      ["제한 영역", "연계 재실행, 권한 정책, 시스템 감사"],
    ],
  },
  "sys-admin": {
    name: "시스템 관리자",
    summary: "연계 상태, 조직 변경 반영, 감사 가능 변경 실행이 먼저 보이는 콘솔 홈",
    heroKicker: "Admin Home",
    heroTitle: "동기화 실패, 조직 변경 반영, 권한 정책이 먼저 보여야 한다.",
    heroDescription:
      "시스템 관리자는 일반 업무보다 연계 안정성, 기준정보 반영, 감사 가능한 변경 실행을 우선 본다.",
    metrics: [
      ["연계 경고", "2", "조치 필요"],
      ["조직 변경 검토", "6", "영향 분석 대기"],
      ["권한 예외", "3", "검토 필요"],
    ],
    priorities: [
      ["HR Sync 검토", "관리자 콘솔 상세 페이지에서 변경 반영 큐와 계정계 연계를 확인", "관리자 콘솔"],
      ["정책 예외 점검", "권한 예외와 AI 실행 정책을 감사 로그와 함께 확인", "관리자 콘솔"],
      ["조직 영향 확인", "조직 운영 페이지와 연결해 승인 영향 범위를 확인", "조직 운영"],
    ],
    actions: [
      ["영향 분석 실행", "조직 변경으로 영향 받는 과제와 계정 확인"],
      ["동기화 재처리", "Bamboo 지연 큐 재실행"],
      ["감사 로그 확인", "역할 정책 변경 이력 추적"],
    ],
    risks: [
      ["동기화 지연", "Bamboo 응답 지연이 릴리스 판단에 영향"],
      ["계정계 불일치", "Crowd 그룹 2건 재계산 필요"],
      ["권한 과부여", "테스트 실행 권한 예외 설정 증가"],
    ],
    access: [
      ["주요 권한", "연계 관리, 조직 변경 반영, 권한/감사 관리"],
      ["기본 필터", "실패 동기화, 변경 검토 대기, 예외 승인"],
      ["제한 영역", "일반 사용자 홈의 일상 작업 흐름에는 기본 진입하지 않음"],
    ],
  },
  se: {
    name: "SE 담당자",
    summary: "환경 차이, CI 플랜, 산출물, 릴리스 준비도를 먼저 보는 품질 홈",
    heroKicker: "SE Home",
    heroTitle: "환경 구성, 배포 준비도, CI 초안 검토가 먼저 보여야 한다.",
    heroDescription:
      "SE 담당자는 개발 자체보다 반영 가능성, 환경 차이, 산출물, CI 설정의 일관성을 우선 본다.",
    metrics: [
      ["환경 경고", "2", "운영 반영 영향"],
      ["CI 초안", "1", "변수 확인 필요"],
      ["릴리스 체크", "8/10", "완료"],
    ],
    priorities: [
      ["환경 차이 검토", "품질 검증 상세 페이지에서 staging/production 차이를 먼저 확인", "품질 검증"],
      ["배포 후보 점검", "과제 워크스페이스와 품질 검증 페이지에서 산출물 메타데이터 확인", "과제 워크스페이스"],
      ["정책 예외 확인", "관리자 콘솔의 예외 승인 상태와 연결해 확인", "관리자 콘솔"],
    ],
    actions: [
      ["환경 차이 비교", "staging/production 설정 비교"],
      ["CI 초안 검토", "템플릿 변수와 예외 규칙 확인"],
      ["산출물 검증", "패키지 서명과 배포 메타데이터 점검"],
    ],
    risks: [
      ["환경 불일치", "운영 환경 라이브러리 버전 차이"],
      ["배포 누락", "모니터링 태그 자동 주입 미확인"],
      ["초안 과신", "AI CI 초안 검토 없이 확정 금지"],
    ],
    access: [
      ["주요 권한", "환경 검토, 산출물 확인, 릴리스 준비도 관리"],
      ["기본 필터", "릴리스 후보, 환경 경고, CI 초안"],
      ["제한 영역", "조직 승인, 권한 정책, 조직 마스터 변경"],
    ],
  },
  qa: {
    name: "테스트 담당자",
    summary: "테스트 실행률, 결함, 릴리스 게이트를 먼저 보는 품질 검증 홈",
    heroKicker: "QA Home",
    heroTitle: "테스트 실행률, 결함, 재검증 대기가 먼저 보여야 한다.",
    heroDescription:
      "테스트 담당자는 요구사항 진척보다 검증 범위, 실패 추세, 결함 차단 여부를 먼저 확인해야 한다.",
    metrics: [
      ["회귀 실행률", "42/48", "진행 중"],
      ["차단 결함", "1", "릴리스 영향"],
      ["재검증 대기", "2", "오늘 처리"],
    ],
    priorities: [
      ["차단 결함 확인", "품질 검증 상세 페이지에서 결함 상태와 재검증 대기를 먼저 확인", "품질 검증"],
      ["회귀 테스트 완료", "남은 6건을 실행하고 릴리스 게이트 재판단", "품질 검증"],
      ["과제 영향 확인", "과제 워크스페이스에서 관련 요구사항과 PR 맥락 확인", "과제 워크스페이스"],
    ],
    actions: [
      ["결함 재검증", "수정본 배포 후 재실행"],
      ["누락 시나리오 등록", "테스트 케이스 추가"],
      ["릴리스 차단 해제 판단", "품질 게이트 충족 여부 검토"],
    ],
    risks: [
      ["차단 결함", "결제 취소 경로 불안정"],
      ["커버리지 부족", "목표 80% 대비 현재 78%"],
      ["재검증 지연", "수정 배포가 늦어질 가능성"],
    ],
    access: [
      ["주요 권한", "테스트 계획/실행, 결함 관리, 품질 게이트 판단"],
      ["기본 필터", "실패 시나리오, 재검증 대기, 차단 결함"],
      ["제한 영역", "조직 승인, 연계 설정, 권한 정책"],
    ],
  },
};

const pageContent = {
  tasks: {
    developer: {
      chip: "개발자 집중: Code Review",
      kicker: "Developer Focus",
      title: "Code Review",
      summary: "리뷰 대기 PR, 코딩룰 위반 의심, 테스트 부족 범위를 먼저 본다.",
      points: [
        ["PR-392", "변경 14개 파일, 위험 3개 파일 우선 확인"],
        ["코딩룰", "예외 승인 없는 위반 의심 2건"],
        ["테스트", "변경 범위 대비 단위테스트 미존재 1건"],
      ],
    },
    "project-lead": {
      chip: "프로젝트리더 집중: Release Readiness",
      kicker: "Project Lead Focus",
      title: "Release Readiness",
      summary: "릴리스 조건, 승인 병목, 품질 게이트 미충족 항목을 먼저 본다.",
      points: [
        ["승인 상태", "조직장 승인 1건, 관리자 검토 1건"],
        ["품질 게이트", "정적분석 1건, 테스트 1건 미충족"],
        ["일정 영향", "3일 지연 가능성, 대체안 필요"],
      ],
    },
    "org-head": {
      chip: "조직장 집중: People Calendar",
      kicker: "Organization Focus",
      title: "People Calendar",
      summary: "승인과 일정에 영향을 주는 조직 부재와 과제 배치를 먼저 본다.",
      points: [
        ["가용성", "주간 리뷰 모수 감소, 배포 일정 충돌"],
        ["조직 영향", "변경 후 승인 루트 재배치 필요"],
        ["과제 분포", "중요 과제 3건이 동일 조직에 집중"],
      ],
    },
    "sys-admin": {
      chip: "시스템 관리자 집중: Governance Snapshot",
      kicker: "Admin Focus",
      title: "Governance Snapshot",
      summary: "과제 자체보다 승인 이력, 연계 상태, 권한 예외를 함께 본다.",
      points: [
        ["연계 상태", "외부 시스템 동기화와 데이터 최신성 확인"],
        ["권한 정책", "역할별 접근 정책 위반 의심 1건"],
        ["감사 이력", "상태 변경 37건 추적 가능"],
      ],
    },
    se: {
      chip: "SE 담당자 집중: Environment",
      kicker: "SE Focus",
      title: "Environment",
      summary: "환경 차이, CI 초안, 산출물 메타데이터를 먼저 본다.",
      points: [
        ["환경 차이", "staging/production 변수 2건 차이"],
        ["산출물", "배포 패키지 2개 검토 필요"],
        ["릴리스 준비", "운영 반영 체크리스트 2건 미완료"],
      ],
    },
    qa: {
      chip: "테스트 담당자 집중: Defects",
      kicker: "QA Focus",
      title: "Defects",
      summary: "결함 심각도, 재검증 대기, 릴리스 차단 여부를 먼저 본다.",
      points: [
        ["차단 결함", "1건, 릴리스 전 필수 해소"],
        ["재검증", "2건 대기, 테스트 환경 준비 완료"],
        ["품질 게이트", "통합테스트 6건 남음"],
      ],
    },
  },
  delivery: {
    developer: {
      chip: "개발자 관점: Delivery Impact",
      kicker: "Developer View",
      title: "Delivery Impact",
      summary: "내 작업이 일정에 미치는 직접 영향을 먼저 확인한다.",
      points: [
        ["내 PR 영향", "코드 동결 지연 가능성 1건"],
        ["빌드 영향", "재실행 대기 build 1건"],
        ["테스트 영향", "누락 시나리오가 릴리스 조건에 반영됨"],
      ],
    },
    "project-lead": {
      chip: "프로젝트리더 집중: Release Plan",
      kicker: "Project Lead Focus",
      title: "Release Plan",
      summary: "마일스톤, 승인 병목, 품질 게이트 미충족 항목을 먼저 본다.",
      points: [
        ["마일스톤", "코드 동결과 릴리스 승인이 핵심 병목"],
        ["병목", "조직장/관리자 승인 2건 대기"],
        ["보고", "경영 보고용 상태 요약 즉시 작성 가능"],
      ],
    },
    "org-head": {
      chip: "조직장 관점: Approval Impact",
      kicker: "Organization Focus",
      title: "Approval Impact",
      summary: "승인 지연이 일정에 미치는 영향을 먼저 본다.",
      points: [
        ["조직 승인", "배포 일정 조정 1건 처리 필요"],
        ["가용성", "핵심 인력 부재가 일정에 영향"],
        ["조정안", "우선순위 재배치 필요"],
      ],
    },
    "sys-admin": {
      chip: "시스템 관리자 관점: Integration Risk",
      kicker: "Admin Focus",
      title: "Integration Risk",
      summary: "외부 연계 지연이 일정과 승인 병목에 주는 영향을 먼저 본다.",
      points: [
        ["Bamboo 지연", "릴리스 판단 타이밍에 직접 영향"],
        ["HR Sync", "승인 루트 갱신 지연 가능성"],
        ["예외 정책", "CI 재실행 예외 승인 필요"],
      ],
    },
    se: {
      chip: "SE 담당자 관점: Release Readiness",
      kicker: "SE Focus",
      title: "Release Readiness",
      summary: "환경 조건과 산출물 준비 상태가 일정 충족 조건인지 먼저 본다.",
      points: [
        ["배포 패키지", "검증 2건 남음"],
        ["환경 차이", "운영 변수 점검 필요"],
        ["릴리스 체크", "모니터링 태그 누락 1건"],
      ],
    },
    qa: {
      chip: "테스트 담당자 관점: Quality Gate Impact",
      kicker: "QA Focus",
      title: "Quality Gate Impact",
      summary: "테스트와 결함 상태가 일정 차단 조건인지 먼저 본다.",
      points: [
        ["회귀 잔여", "6건 완료 전 릴리스 불가"],
        ["차단 결함", "1건 해소 필요"],
        ["재검증", "2건 오늘 처리 필요"],
      ],
    },
  },
  organization: {
    developer: {
      chip: "개발자 관점: Availability Impact",
      kicker: "Developer View",
      title: "Availability Impact",
      summary: "리뷰어 부재와 조직 변경이 내 작업 대기시간에 미치는 영향을 본다.",
      points: [
        ["리뷰 모수", "8명에서 6명으로 감소"],
        ["승인 루트", "조직 변경으로 대기 증가 가능"],
        ["지원 요청", "타 팀 리뷰어 필요 가능성"],
      ],
    },
    "project-lead": {
      chip: "프로젝트리더 관점: Resource Alignment",
      kicker: "Project Lead Focus",
      title: "Resource Alignment",
      summary: "조직 가용성과 승인 지연이 프로젝트 일정에 주는 영향을 본다.",
      points: [
        ["자원 가용성", "핵심 인력 부재 3건"],
        ["승인 대기", "전환/일정 조정 3건"],
        ["대응", "우선순위 재배치 필요"],
      ],
    },
    "org-head": {
      chip: "조직장 집중: Capacity & Approval",
      kicker: "Organization Focus",
      title: "Capacity & Approval",
      summary: "조직 수용 가능성, 승인 대기, 가용성 저하를 먼저 본다.",
      points: [
        ["승인 큐", "전환, 조직 변경, 일정 조정 우선 처리"],
        ["가용성", "다음 주 리뷰 가능 인원 감소"],
        ["영향도", "프로젝트 조직 정보 18건 갱신 후보"],
      ],
    },
    "sys-admin": {
      chip: "시스템 관리자 관점: Master Data Impact",
      kicker: "Admin Focus",
      title: "Master Data Impact",
      summary: "조직/인력 기준정보와 조직 변경이 내부 마스터와 승인 경로에 주는 영향을 본다.",
      points: [
        ["조직 마스터", "활성 28개 조직 중 1건 개편 예정"],
        ["인력 기준정보", "주 소속 변경 3건 반영 대기"],
        ["계정계", "Crowd 그룹 2건 재계산 필요"],
      ],
    },
    se: {
      chip: "SE 담당자 관점: Support Allocation",
      kicker: "SE Focus",
      title: "Support Allocation",
      summary: "환경 지원 리소스와 배포 지원 가능 시간을 먼저 본다.",
      points: [
        ["출장 영향", "SE 1명 출장"],
        ["배포 창구", "운영 지원 슬롯 제한"],
        ["우선순위", "중요 과제 3건 집중"],
      ],
    },
    qa: {
      chip: "테스트 담당자 관점: QA Capacity",
      kicker: "QA Focus",
      title: "QA Capacity",
      summary: "재검증 슬롯과 테스트 셀 가용성을 먼저 본다.",
      points: [
        ["QA 셀", "재검증 슬롯 확보"],
        ["테스트 병목", "동시 회귀 과제 증가"],
        ["조정안", "중요 과제 우선 재배치 필요"],
      ],
    },
  },
  quality: {
    developer: {
      chip: "개발자 관점: Test Feedback",
      kicker: "Developer View",
      title: "Test Feedback",
      summary: "내 변경이 테스트와 품질 게이트에 어떻게 걸리는지 먼저 본다.",
      points: [
        ["커버리지", "목표 80% 대비 78%"],
        ["차단 결함", "내 변경과 연관된 결함 1건"],
        ["AI 테스트", "초안 4건 중 2건 채택"],
      ],
    },
    "project-lead": {
      chip: "프로젝트리더 관점: Gate Progress",
      kicker: "Project Lead Focus",
      title: "Gate Progress",
      summary: "릴리스 차단 조건이 무엇이고 언제 해소되는지 먼저 본다.",
      points: [
        ["정적분석", "Major 1건"],
        ["통합테스트", "6건 잔여"],
        ["판단", "재검증 완료 후 재승인"],
      ],
    },
    "org-head": {
      chip: "조직장 관점: Quality Capacity",
      kicker: "Organization Focus",
      title: "Quality Capacity",
      summary: "품질 검증과 재검증에 필요한 조직 가용성을 먼저 본다.",
      points: [
        ["QA 셀", "재검증 대기 2건"],
        ["SE 지원", "환경 검토 2건"],
        ["조정", "승인 일정과 품질 일정 동기화 필요"],
      ],
    },
    "sys-admin": {
      chip: "시스템 관리자 관점: Policy Exceptions",
      kicker: "Admin Focus",
      title: "Policy Exceptions",
      summary: "AI 실행 권한과 품질 예외 정책이 어떻게 적용되는지 먼저 본다.",
      points: [
        ["예외 승인", "AI 테스트 실행 권한 2건"],
        ["감사 로그", "품질 상태 변경 모두 기록"],
        ["정책", "규칙 예외는 관리자 승인 필요"],
      ],
    },
    se: {
      chip: "SE 담당자 집중: Environment Review",
      kicker: "SE Focus",
      title: "Environment Review",
      summary: "환경 차이, CI 초안, 릴리스 체크리스트를 먼저 본다.",
      points: [
        ["환경 차이", "staging/production 변수 2건 차이"],
        ["CI 초안", "Bamboo Specs 변수 3건 확인 필요"],
        ["산출물", "배포 패키지 2개 검증 필요"],
      ],
    },
    qa: {
      chip: "테스트 담당자 집중: Defect & Regression",
      kicker: "QA Focus",
      title: "Defect & Regression",
      summary: "결함 상태, 회귀 진행, 재검증 대기를 먼저 본다.",
      points: [
        ["차단 결함", "결제 취소 시나리오 1건"],
        ["회귀 잔여", "6건 남음"],
        ["재검증", "2건 오늘 처리 필요"],
      ],
    },
  },
  admin: {
    developer: {
      chip: "개발자 관점: Read-only Governance",
      kicker: "Developer View",
      title: "Read-only Governance",
      summary: "관리자 콘솔은 읽기 중심으로 연계 상태와 승인 근거를 확인하는 용도다.",
      points: [
        ["연계 상태", "빌드와 리뷰 지연 원인 파악"],
        ["변경 반영", "조직 변경 영향 범위 확인"],
        ["제한", "실행 권한은 없음"],
      ],
    },
    "project-lead": {
      chip: "프로젝트리더 관점: Delivery Dependencies",
      kicker: "Project Lead Focus",
      title: "Delivery Dependencies",
      summary: "연계 지연과 정책 예외가 프로젝트 일정에 주는 영향을 확인한다.",
      points: [
        ["Bamboo 지연", "릴리스 승인 판단 지연"],
        ["HR Sync", "승인 루트 갱신 일정 영향"],
        ["예외 승인", "AI 테스트 정책 확인"],
      ],
    },
    "org-head": {
      chip: "조직장 관점: Approval Governance",
      kicker: "Organization Focus",
      title: "Approval Governance",
      summary: "조직장 승인 정책과 조직/인사 기준정보 반영 이력이 어떻게 적용되는지 확인한다.",
      points: [
        ["승인 정책", "프로젝트리더/조직장 분리 적용"],
        ["기준정보 반영", "조직명 변경과 주 소속 이동 이력 확인"],
        ["감사", "주요 상태 변경 이력 조회"],
      ],
    },
    "sys-admin": {
      chip: "시스템 관리자 집중: Governance Console",
      kicker: "Admin Focus",
      title: "Governance Console",
      summary: "연계 연결 설정, 동기화 실행, 조직/인력 마스터 반영, 권한 예외와 감사 로그를 함께 본다.",
      points: [
        ["연계 설정", "Jira/Bamboo/HR Sync endpoint 검증 상태 확인"],
        ["기준정보 반영", "조직 변경 영향 6건과 인력 보정 3건 처리"],
        ["감사", "주요 상태 변경 37건 보관"],
      ],
    },
    se: {
      chip: "SE 담당자 관점: Operational Constraints",
      kicker: "SE Focus",
      title: "Operational Constraints",
      summary: "환경 정책과 운영 예외가 배포 준비도에 주는 영향을 확인한다.",
      points: [
        ["배포 예외", "CI 재실행 정책 확인"],
        ["계정계", "지원 그룹 매핑 영향 확인"],
        ["제한", "실행 권한은 관리자 승인 필요"],
      ],
    },
    qa: {
      chip: "테스트 담당자 관점: Audit & Exceptions",
      kicker: "QA Focus",
      title: "Audit & Exceptions",
      summary: "테스트 실행 권한과 품질 예외가 어떻게 기록되는지 확인한다.",
      points: [
        ["AI 실행 예외", "테스트 실행 권한 2건 검토"],
        ["감사 로그", "상태 변경 이력 조회"],
        ["변경 영향", "조직 변경이 테스트 루트에 미치는 영향 확인"],
      ],
    },
  },
};

function renderCards(target, items, mapper) {
  if (!target || !items) return;
  target.innerHTML = items.map((item) => mapper(item)).join("");
}

function renderOverview(role) {
  renderCards(document.getElementById("hero-metrics"), role.metrics, ([label, value, meta]) => {
    return `<article><span>${label}</span><strong>${value}</strong><small>${meta}</small></article>`;
  });

  renderCards(document.getElementById("priority-strip"), role.priorities, ([title, copy, meta]) => {
    return `<div class="priority-card"><strong>${title}</strong><p>${copy}</p><div class="priority-meta">${meta}</div></div>`;
  });

  renderCards(document.getElementById("quick-actions"), role.actions, ([title, copy]) => {
    return `<div><strong>${title}</strong><p>${copy}</p></div>`;
  });

  renderCards(document.getElementById("risk-signals"), role.risks, ([title, copy]) => {
    return `<div class="signal-card neutral"><strong>${title}</strong><p>${copy}</p></div>`;
  });

  renderCards(document.getElementById("access-summary"), role.access, ([title, copy]) => {
    return `<div><strong>${title}</strong><p>${copy}</p></div>`;
  });
}

function renderPageFocus(roleKey) {
  const content = pageContent[currentScreen]?.[roleKey];
  if (!content) return;

  const chip = document.getElementById("page-role-chip");
  const kicker = document.getElementById("page-role-kicker");
  const title = document.getElementById("page-role-title");
  const summary = document.getElementById("page-role-summary");
  const points = document.getElementById("page-role-points");

  if (chip) chip.textContent = content.chip;
  if (kicker) kicker.textContent = content.kicker;
  if (title) title.textContent = content.title;
  if (summary) summary.textContent = content.summary;

  renderCards(points, content.points, ([head, body]) => {
    return `<div><strong>${head}</strong><p>${body}</p></div>`;
  });
}

function renderRole(roleKey) {
  const role = roleContent[roleKey] || roleContent.developer;
  localStorage.setItem("prototypeRole", roleKey);

  const roleName = document.getElementById("role-name");
  const roleSummary = document.getElementById("role-summary");
  const heroKicker = document.getElementById("hero-kicker");
  const heroTitle = document.getElementById("hero-title");
  const heroDescription = document.getElementById("hero-description");

  if (roleName) roleName.textContent = role.name;
  if (roleSummary) roleSummary.textContent = role.summary;
  if (heroKicker) heroKicker.textContent = role.heroKicker;
  if (heroTitle) heroTitle.textContent = role.heroTitle;
  if (heroDescription) heroDescription.textContent = role.heroDescription;

  renderOverview(role);
  renderPageFocus(roleKey);

  roleChips.forEach((chip) => {
    chip.classList.toggle("active", chip.dataset.role === roleKey);
  });
}

navItems.forEach((item) => {
  item.classList.toggle("active", item.dataset.screen === currentScreen);
});

roleChips.forEach((chip) => {
  chip.addEventListener("click", () => {
    renderRole(chip.dataset.role);
  });
});

renderRole(localStorage.getItem("prototypeRole") || "developer");
