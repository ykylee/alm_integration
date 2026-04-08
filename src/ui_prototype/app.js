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
  "data-alm": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Internal ALM",
      kicker: "System Admin Focus",
      title: "Internal ALM Data Surface",
      summary: "내부 표준 모델 기준의 프로젝트와 업무 항목을 직접 확인하고, 조직/인력 참조 반영 상태를 함께 본다.",
      points: [
        ["프로젝트", "내부 표준 `project` 기준 현황 확인"],
        ["업무 항목", "표준 `work_item` 상태와 담당자 반영 확인"],
        ["참조 상태", "조직/인력 연결 결과를 함께 검토"],
      ],
    },
  },
  "data-external-jira": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Jira Source",
      kicker: "System Admin Focus",
      title: "Jira Data Surface",
      summary: "`Jira` 원천의 수집 실행 이력과 현재 노출 범위를 시스템 단위로 분리해 관리한다.",
      points: [
        ["실행 이력", "최근 `pull`/`push` 실행 결과 확인"],
        ["현재 범위", "현재는 `sync-runs` 중심으로 노출"],
        ["후속 확장", "원시 이벤트, 표준화, 오류 큐를 같은 화면군으로 확장"],
      ],
    },
  },
  "data-external-bitbucket": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Bitbucket Source",
      kicker: "System Admin Focus",
      title: "Bitbucket Data Surface",
      summary: "`Bitbucket` 원천의 수집 실행 이력과 현재 노출 범위를 시스템 단위로 분리해 관리한다.",
      points: [
        ["실행 이력", "최근 `pull`/`push` 실행 결과 확인"],
        ["현재 범위", "현재는 `sync-runs` 중심으로 노출"],
        ["후속 확장", "원시 이벤트, 표준화, 오류 큐를 같은 화면군으로 확장"],
      ],
    },
  },
  "data-external-bamboo": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Bamboo Source",
      kicker: "System Admin Focus",
      title: "Bamboo Data Surface",
      summary: "`Bamboo` 원천의 수집 실행 이력과 현재 노출 범위를 시스템 단위로 분리해 관리한다.",
      points: [
        ["실행 이력", "최근 `pull`/`push` 실행 결과 확인"],
        ["현재 범위", "현재는 `sync-runs` 중심으로 노출"],
        ["후속 확장", "원시 이벤트, 표준화, 오류 큐를 같은 화면군으로 확장"],
      ],
    },
  },
  "data-external-confluence": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Confluence Source",
      kicker: "System Admin Focus",
      title: "Confluence Data Surface",
      summary: "`Confluence` 원천의 수집 실행 이력과 현재 노출 범위를 시스템 단위로 분리해 관리한다.",
      points: [
        ["실행 이력", "최근 `pull`/`push` 실행 결과 확인"],
        ["현재 범위", "현재는 `sync-runs` 중심으로 노출"],
        ["후속 확장", "원시 이벤트, 표준화, 오류 큐를 같은 화면군으로 확장"],
      ],
    },
  },
  "data-organizations": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Organization Master",
      kicker: "System Admin Focus",
      title: "Organization Master Surface",
      summary: "조직 코드를 기준으로 상태, 상위 조직, 유효기간, 영향 범위를 조직 전용 작업면에서 관리한다.",
      points: [
        ["조직 목록", "활성 조직 기준 현황 확인"],
        ["구조 정보", "상위 조직과 유효기간 검토"],
        ["영향 범위", "후속 인력과 도메인 참조 영향 확인"],
      ],
    },
  },
  "data-workforce": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Workforce Master",
      kicker: "System Admin Focus",
      title: "Workforce by Organization",
      summary: "조직 기준으로 인력 목록을 조회하고 재직 상태, 직군, 소속 반영 결과를 함께 본다.",
      points: [
        ["조직 필터", "조직별 인력 리스트 조회"],
        ["재직 상태", "활성 인력 기준 상태 확인"],
        ["연결 상태", "조직 기준 인력 반영 결과 검토"],
      ],
    },
  },
  "data-settings": {
    "sys-admin": {
      chip: "시스템 관리자 전용: Integration Settings",
      kicker: "System Settings Focus",
      title: "Connection Settings",
      summary: "운영 화면에서 공통으로 사용하는 관리자 API 연결 정보를 한 곳에서 저장하고 검증합니다.",
      points: [
        ["설정 집중", "운영 화면에서 분리된 연결 설정 작업면"],
        ["공통 적용", "저장한 API URL을 데이터 관리 화면 전체에서 재사용"],
        ["연결 검증", "헬스 체크로 현재 연결 상태를 바로 확인"],
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
  const content = pageContent[currentScreen]?.[roleKey] || pageContent[currentScreen]?.["sys-admin"];
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

const PROTOTYPE_API_BASE_KEY = "prototypeApiBaseUrl";
const PROTOTYPE_SELECTED_ORGANIZATION_KEY = "prototypeSelectedOrganizationCode";
const DEFAULT_API_BASE_URL = "http://127.0.0.1:8080/api/v1";
const organizationAdminState = {
  organizations: [],
  workforce: [],
};
const workforceAdminState = {
  items: [],
  organizationCode: "",
  selectedEmployeeNumber: "",
};

function escapeHtml(value) {
  return String(value ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;")
    .replaceAll("'", "&#39;");
}

function getQueryApiBaseUrl() {
  const params = new URLSearchParams(window.location.search);
  return params.get("apiBase");
}

function getStoredApiBaseUrl() {
  return localStorage.getItem(PROTOTYPE_API_BASE_KEY) || "";
}

function resolveApiBaseUrl() {
  return getQueryApiBaseUrl() || getStoredApiBaseUrl() || DEFAULT_API_BASE_URL;
}

function getActiveApiBaseUrl() {
  const input = document.getElementById("api-base-url-input");
  return (input?.value || resolveApiBaseUrl()).trim();
}

function updateCurrentApiBaseUrlDisplays() {
  const value = resolveApiBaseUrl();
  document.querySelectorAll("[data-api-base-url-display]").forEach((element) => {
    element.textContent = value;
  });
}

function getSelectedOrganizationCode() {
  return localStorage.getItem(PROTOTYPE_SELECTED_ORGANIZATION_KEY) || "";
}

function setSelectedOrganizationCode(value) {
  if (!value) {
    localStorage.removeItem(PROTOTYPE_SELECTED_ORGANIZATION_KEY);
    return;
  }
  localStorage.setItem(PROTOTYPE_SELECTED_ORGANIZATION_KEY, value);
}

function setApiStatus(kind, message) {
  const chip = document.getElementById("api-status-chip");
  const copy = document.getElementById("api-status-copy");
  if (!chip || !copy) return;

  chip.classList.remove("ok", "warn", "danger");
  if (kind === "ok") chip.classList.add("ok");
  if (kind === "warn") chip.classList.add("warn");
  if (kind === "danger") chip.classList.add("danger");

  chip.textContent =
    kind === "ok" ? "연결 정상" : kind === "warn" ? "부분 연결" : kind === "danger" ? "연결 실패" : "연결 대기";
  copy.textContent = message;
}

function setMetricBlock(targetId, metrics) {
  const target = document.getElementById(targetId);
  if (!target) return;

  target.innerHTML = metrics
    .map(
      ({ label, value }) =>
        `<div><span>${escapeHtml(label)}</span><strong>${escapeHtml(value)}</strong></div>`,
    )
    .join("");
}

function renderTableRows(targetId, rows, emptyMessage) {
  const target = document.getElementById(targetId);
  if (!target) return;

  if (!rows.length) {
    target.innerHTML = `<tr><td colspan="3">${escapeHtml(emptyMessage)}</td></tr>`;
    return;
  }

  target.innerHTML = rows.join("");
}

function renderBulletSummary(targetId, items, emptyMessage) {
  const target = document.getElementById(targetId);
  if (!target) return;

  if (!items.length) {
    target.innerHTML = `<div><strong>비어 있음</strong><p>${escapeHtml(emptyMessage)}</p></div>`;
    return;
  }

  target.innerHTML = items
    .map(
      ({ title, body }) => `<div><strong>${escapeHtml(title)}</strong><p>${escapeHtml(body)}</p></div>`,
    )
    .join("");
}

function formatRelativeTime(value) {
  if (!value) return "-";
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return String(value);
  }

  return date.toLocaleString("ko-KR", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

async function fetchJson(baseUrl, path, params = {}) {
  const url = new URL(path, `${baseUrl.replace(/\/$/, "")}/`);

  Object.entries(params).forEach(([key, value]) => {
    if (value !== undefined && value !== null && value !== "") {
      url.searchParams.set(key, value);
    }
  });

  const response = await fetch(url, {
    headers: {
      accept: "application/json",
    },
  });

  let payload = null;
  const text = await response.text();
  if (text) {
    try {
      payload = JSON.parse(text);
    } catch (error) {
      throw new Error(`JSON 파싱 실패: ${error.message}`);
    }
  }

  if (!response.ok) {
    const reason = typeof payload === "string" ? payload : text || response.statusText;
    throw new Error(`${response.status} ${reason}`.trim());
  }

  return payload;
}

async function sendJson(baseUrl, path, method, payload) {
  const url = new URL(path, `${baseUrl.replace(/\/$/, "")}/`);
  const response = await fetch(url, {
    method,
    headers: {
      accept: "application/json",
      "content-type": "application/json",
    },
    body: payload ? JSON.stringify(payload) : undefined,
  });

  const text = await response.text();
  let parsed = null;
  if (text) {
    try {
      parsed = JSON.parse(text);
    } catch (_error) {
      parsed = text;
    }
  }

  if (!response.ok) {
    const reason = typeof parsed === "string" ? parsed : text || response.statusText;
    throw new Error(`${response.status} ${reason}`.trim());
  }

  return parsed;
}

function formatRunStatus(run) {
  if (run.run_status === "queued") return "대기";
  if (run.run_status === "completed") return "완료";
  if (run.run_status === "partially_completed") return "부분 완료";
  if (run.run_status === "failed") return "실패";
  return run.run_status;
}

function summarizeSyncRuns(items) {
  const queued = items.filter((item) => item.run_status === "queued").length;
  const partial = items.filter((item) => item.run_status === "partially_completed").length;
  const failed = items.filter((item) => item.run_status === "failed").length;

  return [
    { label: "동기화 실행", value: `${items.length}건` },
    { label: "대기/부분 완료", value: `${queued + partial}건` },
    { label: "실패 실행", value: `${failed}건` },
  ];
}

async function loadAdminLiveData() {
  const baseUrl = getActiveApiBaseUrl();
  localStorage.setItem(PROTOTYPE_API_BASE_KEY, baseUrl);
  setApiStatus("loading", `관리자 API를 확인 중입니다. ${baseUrl}`);

  try {
    const [syncRuns, organizations, workforce, projects, workItems] = await Promise.all([
      fetchJson(baseUrl, "admin/sync-runs"),
      fetchJson(baseUrl, "admin/master-data/organizations", { organization_status: "active" }),
      fetchJson(baseUrl, "admin/master-data/workforce", { employment_status: "active" }),
      fetchJson(baseUrl, "admin/projects"),
      fetchJson(baseUrl, "admin/work-items"),
    ]);

    const syncRunItems = syncRuns?.items || [];
    const organizationItems = organizations?.items || [];
    const workforceItems = workforce?.items || [];
    const projectItems = projects?.items || [];
    const workItemItems = workItems?.items || [];

    const latestSyncRows = syncRunItems.slice(0, 5).map((item) => {
      const processed = `${item.success_count}/${item.processed_count}`;
      const statusClass =
        item.run_status === "completed" ? "ok" : item.run_status === "queued" ? "warn" : "danger-text";
      return `<tr><td class="mono">${escapeHtml(item.source_system)}</td><td>${escapeHtml(processed)} 처리 · ${escapeHtml(
        item.mode,
      )}</td><td class="${statusClass}">${escapeHtml(formatRunStatus(item))}</td></tr>`;
    });

    const organizationSummary = organizationItems.slice(0, 4).map((item) => ({
      title: `${item.organization_name} (${item.organization_code})`,
      body: `상태 ${item.organization_status}, 상위 조직 ${item.parent_organization_code || "없음"}`,
    }));
    const workforceSummary = workforceItems.slice(0, 4).map((item) => ({
      title: `${item.display_name} (${item.employee_number})`,
      body: `${item.primary_organization_name} 소속 · ${item.employment_status}`,
    }));
    const projectRows = projectItems.slice(0, 5).map((item) => {
      return `<tr><td class="mono">${escapeHtml(item.project_code)}</td><td>${escapeHtml(
        item.owning_organization_name || "미지정",
      )}</td><td>${escapeHtml(item.project_owner_display_name || "미지정")}</td></tr>`;
    });
    const workItemRows = workItemItems.slice(0, 5).map((item) => {
      return `<tr><td class="mono">${escapeHtml(item.work_item_key)}</td><td>${escapeHtml(
        item.assignee_display_name || "미지정",
      )}</td><td>${escapeHtml(item.current_common_status)}</td></tr>`;
    });

    setMetricBlock("admin-live-metrics", [
      { label: "동기화 실행", value: `${syncRunItems.length}건` },
      { label: "활성 조직", value: `${organizationItems.length}개` },
      { label: "활성 인력", value: `${workforceItems.length}명` },
    ]);
    renderTableRows("admin-sync-runs-body", latestSyncRows, "표시할 동기화 실행 이력이 없습니다.");
    renderBulletSummary(
      "admin-organization-summary",
      organizationSummary,
      "조직 기준정보가 아직 적재되지 않았습니다.",
    );
    renderBulletSummary(
      "admin-workforce-summary",
      workforceSummary,
      "인력 기준정보가 아직 적재되지 않았습니다.",
    );
    renderTableRows("admin-projects-body", projectRows, "표시할 프로젝트가 없습니다.");
    renderTableRows("admin-work-items-body", workItemRows, "표시할 업무 항목이 없습니다.");
    renderBulletSummary("admin-api-coverage", [
      { title: "sync-runs", body: `총 ${syncRunItems.length}건, 대기/부분 완료 ${summarizeSyncRuns(syncRunItems)[1].value}` },
      { title: "master-data", body: `조직 ${organizationItems.length}개, 인력 ${workforceItems.length}명` },
      { title: "domain-data", body: `프로젝트 ${projectItems.length}건, 업무 항목 ${workItemItems.length}건` },
    ]);

    setApiStatus("ok", `관리자 API 연결 완료. 기준 URL ${baseUrl}`);
  } catch (error) {
    setMetricBlock("admin-live-metrics", [
      { label: "동기화 실행", value: "-" },
      { label: "활성 조직", value: "-" },
      { label: "활성 인력", value: "-" },
    ]);
    renderTableRows("admin-sync-runs-body", [], "동기화 실행 이력을 불러오지 못했습니다.");
    renderBulletSummary("admin-organization-summary", [], "조직 기준정보를 불러오지 못했습니다.");
    renderBulletSummary("admin-workforce-summary", [], "인력 기준정보를 불러오지 못했습니다.");
    renderTableRows("admin-projects-body", [], "프로젝트 운영 뷰를 불러오지 못했습니다.");
    renderTableRows("admin-work-items-body", [], "업무 항목 운영 뷰를 불러오지 못했습니다.");
    renderBulletSummary("admin-api-coverage", [], "운영 API 범위를 확인하지 못했습니다.");
    setApiStatus("danger", `관리자 API 연결 실패: ${error.message}`);
  }
}

async function loadOrganizationLiveData() {
  const filterInput = document.getElementById("organization-filter-input");
  if (!filterInput) return;

  const baseUrl = getActiveApiBaseUrl();
  const organizationCode = filterInput.value.trim();
  localStorage.setItem(PROTOTYPE_API_BASE_KEY, baseUrl);

  setApiStatus(
    "loading",
    `조직 운영 API를 확인 중입니다. ${organizationCode || "전체"} 조직 기준으로 조회합니다.`,
  );

  try {
    const [organizations, workforce, projects, workItems] = await Promise.all([
      fetchJson(baseUrl, "admin/master-data/organizations", {
        organization_status: "active",
        organization_code: organizationCode,
      }),
      fetchJson(baseUrl, "admin/master-data/workforce", {
        employment_status: "active",
        primary_organization_code: organizationCode,
      }),
      fetchJson(baseUrl, "admin/projects", {
        owning_organization_code: organizationCode,
      }),
      fetchJson(baseUrl, "admin/work-items", {
        owning_organization_code: organizationCode,
      }),
    ]);

    const organizationItems = organizations?.items || [];
    const workforceItems = workforce?.items || [];
    const projectItems = projects?.items || [];
    const workItemItems = workItems?.items || [];
    const targetName = organizationItems[0]?.organization_name || organizationCode || "전체 조직";

    const organizationRows = organizationItems.slice(0, 6).map((item) => {
      return `<tr><td class="mono">${escapeHtml(item.organization_code)}</td><td>${escapeHtml(
        item.organization_name,
      )}</td><td class="${item.organization_status === "active" ? "ok" : "warn"}">${escapeHtml(
        item.organization_status,
      )}</td></tr>`;
    });
    const workforceSummary = workforceItems.slice(0, 5).map((item) => ({
      title: `${item.display_name} (${item.employee_number})`,
      body: `${item.primary_organization_name} · ${item.employment_status}${item.job_family ? ` · ${item.job_family}` : ""}`,
    }));
    const projectRows = projectItems.slice(0, 6).map((item) => {
      return `<tr><td class="mono">${escapeHtml(item.project_code)}</td><td>${escapeHtml(
        item.project_name,
      )}</td><td>${escapeHtml(item.project_owner_display_name || "미지정")}</td></tr>`;
    });
    const workItemRows = workItemItems.slice(0, 6).map((item) => {
      return `<tr><td class="mono">${escapeHtml(item.work_item_key)}</td><td>${escapeHtml(
        item.assignee_display_name || "미지정",
      )}</td><td>${escapeHtml(item.current_common_status)}</td></tr>`;
    });

    setMetricBlock("organization-live-metrics", [
      { label: "대상 조직", value: targetName },
      { label: "소속 인력", value: `${workforceItems.length}명` },
      { label: "연결 과제", value: `${projectItems.length}/${workItemItems.length}` },
    ]);
    renderTableRows("organization-master-body", organizationRows, "조건에 맞는 조직이 없습니다.");
    renderBulletSummary(
      "organization-workforce-summary",
      workforceSummary,
      "조건에 맞는 인력 기준정보가 없습니다.",
    );
    renderTableRows("organization-projects-body", projectRows, "조건에 맞는 프로젝트가 없습니다.");
    renderTableRows("organization-work-items-body", workItemRows, "조건에 맞는 업무 항목이 없습니다.");

    setApiStatus("ok", `조직 운영 API 연결 완료. ${targetName} 기준 데이터를 반영했습니다.`);
  } catch (error) {
    setMetricBlock("organization-live-metrics", [
      { label: "대상 조직", value: "-" },
      { label: "소속 인력", value: "-" },
      { label: "연결 과제", value: "-" },
    ]);
    renderTableRows("organization-master-body", [], "조직 마스터를 불러오지 못했습니다.");
    renderBulletSummary("organization-workforce-summary", [], "인력 기준정보를 불러오지 못했습니다.");
    renderTableRows("organization-projects-body", [], "프로젝트 뷰를 불러오지 못했습니다.");
    renderTableRows("organization-work-items-body", [], "업무 항목 뷰를 불러오지 못했습니다.");
    setApiStatus("danger", `조직 운영 API 연결 실패: ${error.message}`);
  }
}

function summarizeExternalRuns(items, sourceSystem) {
  const systemItems = items.filter((item) => item.source_system === sourceSystem);
  if (!systemItems.length) {
    return [{ title: "실행 이력 없음", body: "아직 수집 또는 수동 실행 이력이 없습니다." }];
  }

  return systemItems.slice(0, 3).map((item) => ({
    title: `${item.mode} · ${formatRunStatus(item)}`,
    body: `${item.reason || "사유 없음"} · queued ${item.queued_at || "-"}`,
  }));
}

function summarizeAlmDetail(projectItems, workItemItems) {
  const project = projectItems[0];
  const workItem = workItemItems[0];

  if (!project && !workItem) {
    return [{ title: "대표 데이터 없음", body: "표시할 내부 표준 데이터가 없습니다." }];
  }

  return [
    project
      ? {
          title: `프로젝트 ${project.project_code}`,
          body: `${project.project_name} · 상태 ${project.project_status || "미정"} · 책임자 ${project.project_owner_display_name || "미지정"}`,
        }
      : {
          title: "프로젝트 없음",
          body: "프로젝트 데이터가 아직 적재되지 않았습니다.",
        },
    workItem
      ? {
          title: `업무 항목 ${workItem.work_item_key}`,
          body: `${workItem.current_common_status || "미정"} · 담당 ${workItem.assignee_display_name || "미지정"} · 소속 ${workItem.owning_organization_name || "미지정"}`,
        }
      : {
          title: "업무 항목 없음",
          body: "업무 항목 데이터가 아직 적재되지 않았습니다.",
        },
  ];
}

function summarizeAlmActions(projectItems, workItemItems) {
  return [
    {
      title: "대표 레코드 점검",
      body: `프로젝트 ${projectItems.length}건, 업무 항목 ${workItemItems.length}건 기준으로 이상치 여부를 검토합니다.`,
    },
    {
      title: "조직/인력 참조 확인",
      body: "담당 조직과 담당자 연결이 비어 있는 표준 레코드를 우선 확인합니다.",
    },
    {
      title: "후속 관리",
      body: "계획 단위, 상태 이력, 계층 연결 누락이 있으면 도메인 반영 파이프라인을 재점검합니다.",
    },
  ];
}

function summarizeAlmImpact(projectItems, workItemItems) {
  const plannedItems = workItemItems.filter((item) => item.iteration_name || item.current_common_status).length;
  const referencedOrganizations = new Set(
    workItemItems.map((item) => item.owning_organization_name).filter((value) => Boolean(value)),
  ).size;

  return [
    {
      title: "조직 영향",
      body: `업무 항목 기준 ${referencedOrganizations}개 조직이 내부 표준 데이터에 연결돼 있습니다.`,
    },
    {
      title: "업무 흐름 영향",
      body: `현재 프로젝트 ${projectItems.length}건, 계획/상태가 보이는 업무 항목 ${plannedItems}건을 운영 뷰에서 추적할 수 있습니다.`,
    },
    {
      title: "운영 기준",
      body: "내부 ALM 화면은 외부 원천과 분리된 최종 표준 상태를 판단하는 기준 화면으로 사용합니다.",
    },
  ];
}

function summarizeExternalDetail(items, sourceSystem) {
  const item = items.find((run) => run.source_system === sourceSystem);
  if (!item) {
    return [{ title: "대표 실행 없음", body: "최근 실행 건이 아직 없습니다." }];
  }

  return [
    {
      title: `${getExternalSystemName(sourceSystem)} 최근 실행`,
      body: `${item.mode} · 상태 ${formatRunStatus(item)} · 처리 ${item.success_count || 0}/${item.processed_count || 0}`,
    },
    {
      title: "실행 사유",
      body: `${item.reason || "사유 없음"} · queued ${item.queued_at || "-"} · completed ${item.completed_at || "-"}`,
    },
  ];
}

function summarizeExternalActions(items, sourceSystem) {
  const item = items.find((run) => run.source_system === sourceSystem);
  const status = item?.run_status || "unknown";

  return [
    {
      title: "실행 상태 검토",
      body: status === "failed" ? "실패 원인과 원천 시스템 상태를 우선 확인합니다." : "최근 실행 건의 처리 결과와 지연 여부를 먼저 확인합니다.",
    },
    {
      title: "재실행 판단",
      body: status === "queued" ? "대기 상태가 길어지면 실행 큐와 스케줄을 점검합니다." : "필요 시 수동 재실행 또는 후속 적재 경로를 검토합니다.",
    },
    {
      title: "후속 확장",
      body: "원시 이벤트, 표준화, 오류 큐가 붙으면 이 화면에서 바로 이어서 확인할 수 있게 설계합니다.",
    },
  ];
}

function summarizeOrganizationActions(items) {
  const item = items[0];
  return [
    {
      title: "변경 후보 점검",
      body: item ? `${item.organization_name} 기준 상위 조직과 상태를 우선 검토합니다.` : "조직 변경 후보가 아직 없습니다.",
    },
    {
      title: "영향 확인",
      body: "조직 변경 시 인력 주 소속과 프로젝트/업무 항목 참조 영향 범위를 함께 봅니다.",
    },
    {
      title: "후속 반영",
      body: "변경이 확정되면 인력 관리 화면과 도메인 조회 화면에서 후속 반영 여부를 재확인합니다.",
    },
  ];
}

function summarizeOrganizationImpact(items) {
  const topLevelCount = items.filter((item) => !item.parent_organization_code).length;
  return [
    {
      title: "구조 영향",
      body: `현재 활성 조직 ${items.length}개 중 최상위 조직 ${topLevelCount}개를 기준으로 트리 확장이 가능합니다.`,
    },
    {
      title: "기준정보 영향",
      body: "조직 마스터는 인력 주 소속과 프로젝트/업무 항목 담당 조직의 기준 축으로 사용됩니다.",
    },
    {
      title: "운영 기준",
      body: "조직 관리 화면은 구조 변경과 유효기간 점검을 먼저 수행하는 기준 작업면입니다.",
    },
  ];
}

function summarizeWorkforceDetail(items, organizationCode) {
  const item = items[0];
  if (!item) {
    return [{ title: "대표 인력 없음", body: `${organizationCode || "전체"} 기준 인력 데이터가 없습니다.` }];
  }

  return [
    {
      title: `${item.display_name} (${item.employee_number})`,
      body: `${item.primary_organization_name || "미지정"} · ${item.employment_status} · ${item.job_family || "직군 미정"}`,
    },
    {
      title: "참조 기준",
      body: `이 인력은 프로젝트 책임자, 업무 항목 담당자/등록자 참조 후보로 사용됩니다.`,
    },
  ];
}

function summarizeWorkforceActions(items, organizationCode) {
  return [
    {
      title: "조직별 인력 검토",
      body: `${organizationCode || "전체"} 기준 ${items.length}명 중 재직 상태와 주 소속을 먼저 확인합니다.`,
    },
    {
      title: "참조 보정 준비",
      body: "프로젝트 책임자와 업무 항목 담당자 매핑에 필요한 사번 누락 여부를 확인합니다.",
    },
    {
      title: "후속 연계 확인",
      body: "조직 이동이나 비활성 전환 후 도메인 참조가 정상 반영됐는지 운영 조회 화면과 함께 점검합니다.",
    },
  ];
}

function setSelectedWorkforceEmployeeNumber(employeeNumber) {
  workforceAdminState.selectedEmployeeNumber = employeeNumber || "";
}

function getSelectedWorkforceItem(items) {
  if (!items.length) return null;
  const selected =
    items.find((item) => item.employee_number === workforceAdminState.selectedEmployeeNumber) || null;
  return selected || items[0] || null;
}

function summarizeWorkforceDirectory(items, organizationCode) {
  if (!items.length) {
    return [{ title: "표시 대상 없음", body: `${organizationCode || "전체"} 기준 인력 데이터가 없습니다.` }];
  }

  const activeCount = items.filter((item) => item.employment_status !== "inactive").length;
  const orgCount = new Set(items.map((item) => item.primary_organization_code).filter(Boolean)).size;
  const jobFamilyCount = new Set(items.map((item) => item.job_family).filter(Boolean)).size;

  return [
    {
      title: `${organizationCode || "전체"} 범위 ${items.length}명`,
      body: `활성 ${activeCount}명 · 조직 ${orgCount}개 · 직군 ${jobFamilyCount}개 기준으로 보고 있습니다.`,
    },
    {
      title: "이 페이지에서 얻는 정보",
      body: "누가 어디 소속인지, 어떤 인력이 기준 참조 축인지, 어느 조직으로 이동시킬지 한 번에 판단합니다.",
    },
  ];
}

function summarizeSelectedWorkforce(selected, organizationCode, items) {
  if (!selected) {
    return [{ title: "선택 인력 없음", body: `${organizationCode || "현재 필터"} 범위에서 인력을 선택해 상세와 조치 흐름을 확인하세요.` }];
  }

  const peers = items.filter(
    (item) =>
      item.primary_organization_code === selected.primary_organization_code &&
      item.employment_status !== "inactive",
  ).length;

  return [
    {
      title: `${selected.display_name} (${selected.employee_number})`,
      body: `${selected.primary_organization_name || "미지정"} · ${selected.employment_status} · ${selected.job_family || "직군 미정"}`,
    },
    {
      title: "현재 조직 문맥",
      body: `${selected.primary_organization_name || "현재 조직"} 에서 함께 보이는 활성 인력 ${peers}명 중 한 명입니다.`,
    },
    {
      title: "참조 사용 위치",
      body: "프로젝트 책임자와 업무 항목 담당자/등록자 매핑 기준으로 연결될 수 있는 인력입니다.",
    },
  ];
}

function summarizeWorkforceOrganizationContext(selected, items, organizationCode) {
  if (!selected) {
    return [{ title: "조직 맥락 없음", body: `${organizationCode || "현재 필터"} 범위에서 인력을 선택하면 같은 조직 구성과 인접 문맥이 표시됩니다.` }];
  }

  const peers = items.filter((item) => item.primary_organization_code === selected.primary_organization_code);
  const sameFamily = peers.filter((item) => item.job_family && item.job_family === selected.job_family).length;

  return [
    {
      title: `${selected.primary_organization_name || "미지정"} 조직 맥락`,
      body: `현재 필터 안에서 같은 조직 소속 ${peers.length}명, 같은 직군 ${sameFamily}명이 확인됩니다.`,
    },
    {
      title: "운영자가 이 페이지에서 하는 판단",
      body: "조직별 인력 과밀/공백, 기준 참조 대상 누락, 이동 후 조직 수용 상태를 함께 확인합니다.",
    },
  ];
}

function summarizeWorkforceImpact(selected) {
  if (!selected) {
    return [{ title: "후속 점검 없음", body: "선택 인력이 없으면 이동 또는 비활성화 후속 점검 항목을 계산할 수 없습니다." }];
  }

  return [
    {
      title: "도메인 참조 확인",
      body: `${selected.display_name} 변경 후에는 프로젝트 책임자, 업무 항목 담당자/등록자 참조가 정상인지 다시 봐야 합니다.`,
    },
    {
      title: "조직 정원 관점",
      body: "조직 이동 시 현재 조직과 대상 조직의 활성 인력 수가 함께 변하므로 두 조직을 같이 확인해야 합니다.",
    },
    {
      title: "감사 추적",
      body: "이동 또는 비활성화 후에는 조직 관리 화면의 구성원 이동 이력과 함께 변경 사실을 재확인합니다.",
    },
  ];
}

function summarizeWorkforceActionPreview(items) {
  const organizationCode = document.getElementById("member-organization-code-input")?.value.trim();
  const targetOrganizationCode = document.getElementById("member-target-organization-code-input")?.value.trim();
  const employeeNumber = document.getElementById("member-employee-number-input")?.value.trim();
  const employmentStatus = document.getElementById("member-employment-status-input")?.value.trim();

  if (!organizationCode || !employeeNumber) {
    return [{ title: "입력 필요", body: "현재 소속 조직과 사번을 입력하면 이동 또는 비활성화 영향을 미리 볼 수 있습니다." }];
  }

  const member = items.find((item) => item.employee_number === employeeNumber) || null;
  const currentCount = items.filter(
    (item) => item.primary_organization_code === organizationCode && item.employment_status !== "inactive",
  ).length;

  if (!member) {
    return [
      {
        title: "신규 등록 모드",
        body: `${organizationCode} 에 새 구성원을 등록할 예정입니다. 현재 이 범위에서 같은 조직 활성 인력은 ${currentCount}명입니다.`,
      },
      {
        title: "입력 점검",
        body: "신규 등록 시 이동 대상 조직은 비워두고, 재직 상태와 직군을 먼저 확정하는 편이 안전합니다.",
      },
    ];
  }

  if (employmentStatus === "inactive" && !targetOrganizationCode) {
    return [
      {
        title: "비활성화 영향",
        body: `${member.display_name} 을 비활성화하면 현재 조직 활성 인력은 ${Math.max(currentCount - 1, 0)}명으로 줄어듭니다.`,
      },
      {
        title: "후속 확인",
        body: "비활성화 뒤에는 이 인력을 참조하는 프로젝트/업무 항목 누락이 없는지 함께 점검해야 합니다.",
      },
    ];
  }

  if (targetOrganizationCode) {
    const targetCount = items.filter(
      (item) =>
        item.primary_organization_code === targetOrganizationCode && item.employment_status !== "inactive",
    ).length;

    return [
      {
        title: "이동 영향",
        body:
          targetOrganizationCode === organizationCode
            ? "같은 조직으로 이동할 수는 없습니다. 이동이 아니라 현재 정보 수정으로 처리됩니다."
            : `${member.display_name} 이동 시 현재 조직은 ${Math.max(currentCount - 1, 0)}명, 대상 조직은 ${targetCount + 1}명으로 바뀝니다.`,
      },
      {
        title: "운영 체크",
        body: "이동 후에는 대상 조직 화면과 조직 변경 이력을 함께 열어 후속 반영을 확인하는 편이 좋습니다.",
      },
    ];
  }

  return [
    {
      title: "현재 정보 수정 모드",
      body: `${member.display_name} 의 인적 정보 또는 상태를 현재 조직 ${organizationCode} 문맥에서 수정합니다.`,
    },
    {
      title: "후속 확인",
      body: "이름, 직군, 이메일 같은 참조 정보가 바뀌면 이후 운영 조회 화면에서도 반영 여부를 확인해야 합니다.",
    },
  ];
}

function getOrganizationDepthMap(organizations) {
  const byCode = new Map(organizations.map((item) => [item.organization_code, item]));
  const memo = new Map();

  function depthFor(code) {
    if (memo.has(code)) return memo.get(code);
    const current = byCode.get(code);
    if (!current || !current.parent_organization_code) {
      memo.set(code, 0);
      return 0;
    }
    const depth = depthFor(current.parent_organization_code) + 1;
    memo.set(code, depth);
    return depth;
  }

  organizations.forEach((item) => depthFor(item.organization_code));
  return memo;
}

function getOrganizationLevelLabel(depth) {
  if (depth === 0) return "사업부";
  if (depth === 1) return "팀";
  if (depth === 2) return "그룹";
  if (depth === 3) return "파트";
  return "세부조직";
}

function getOrganizationDescendantCodes(organizations, organizationCode) {
  const childrenMap = new Map();
  organizations.forEach((item) => {
    const parent = item.parent_organization_code || "__root__";
    const current = childrenMap.get(parent) || [];
    current.push(item.organization_code);
    childrenMap.set(parent, current);
  });

  const result = [];
  const queue = [organizationCode];
  while (queue.length) {
    const currentCode = queue.shift();
    result.push(currentCode);
    const children = childrenMap.get(currentCode) || [];
    children.forEach((childCode) => queue.push(childCode));
  }
  return result;
}

function getOrganizationByCode(organizations, organizationCode) {
  return organizations.find((item) => item.organization_code === organizationCode) || null;
}

function getOrganizationDirectoryFilterState() {
  return {
    query: document.getElementById("organization-directory-search-input")?.value.trim().toLowerCase() || "",
    scope: document.getElementById("organization-directory-scope-select")?.value || "all",
  };
}

function getWorkforceFilterState() {
  return {
    organizationCode: document.getElementById("organization-filter-input")?.value.trim() || "",
    query: document.getElementById("workforce-search-input")?.value.trim().toLowerCase() || "",
  };
}

function getAncestorPath(organizations, organizationCode) {
  const byCode = new Map(organizations.map((item) => [item.organization_code, item]));
  const path = [];
  let current = byCode.get(organizationCode) || null;
  let guard = 0;
  while (current && guard < organizations.length + 1) {
    path.unshift(current.organization_name);
    current = current.parent_organization_code
      ? byCode.get(current.parent_organization_code) || null
      : null;
    guard += 1;
  }
  return path;
}

function getOrganizationAncestorCodes(organizations, organizationCode) {
  const byCode = new Map(organizations.map((item) => [item.organization_code, item]));
  const codes = [];
  let current = byCode.get(organizationCode) || null;
  let guard = 0;
  while (current && guard < organizations.length + 1) {
    codes.push(current.organization_code);
    current = current.parent_organization_code
      ? byCode.get(current.parent_organization_code) || null
      : null;
    guard += 1;
  }
  return codes;
}

function getFilteredOrganizations(organizations) {
  const { query, scope } = getOrganizationDirectoryFilterState();
  const selectedCode = getSelectedOrganizationCode();

  let scopedItems = organizations;
  if (scope === "top_level") {
    scopedItems = organizations.filter((item) => !item.parent_organization_code);
  } else if (scope === "selected_subtree" && selectedCode) {
    const descendantCodes = new Set(getOrganizationDescendantCodes(organizations, selectedCode));
    scopedItems = organizations.filter((item) => descendantCodes.has(item.organization_code));
  }

  if (!query) {
    return scopedItems;
  }

  const scopedCodeSet = new Set(scopedItems.map((item) => item.organization_code));
  const matchedCodes = new Set(
    scopedItems
      .filter((item) => {
        const haystack = `${item.organization_code} ${item.organization_name}`.toLowerCase();
        return haystack.includes(query);
      })
      .map((item) => item.organization_code),
  );

  if (!matchedCodes.size) {
    return [];
  }

  const visibleCodes = new Set();
  matchedCodes.forEach((code) => {
    getOrganizationAncestorCodes(organizations, code).forEach((ancestorCode) => {
      if (scopedCodeSet.has(ancestorCode)) {
        visibleCodes.add(ancestorCode);
      }
    });
    getOrganizationDescendantCodes(organizations, code).forEach((descendantCode) => {
      if (scopedCodeSet.has(descendantCode)) {
        visibleCodes.add(descendantCode);
      }
    });
  });

  return scopedItems.filter((item) => visibleCodes.has(item.organization_code));
}

function renderOrganizationSelectionState() {
  const organizations = organizationAdminState.organizations;
  const workforceItems = organizationAdminState.workforce;
  const selectedCode = getSelectedOrganizationCode();
  const selected = getOrganizationByCode(organizations, selectedCode);

  const banner = document.getElementById("organization-selected-banner");
  if (banner) {
    if (!selected) {
      banner.innerHTML =
        '<div class="status-chip">선택 대기</div><p>트리나 디렉터리에서 조직을 선택하면 현재 작업 대상이 여기에 표시됩니다.</p>';
    } else {
      const path = getAncestorPath(organizations, selected.organization_code).join(" > ");
      const directMembers = workforceItems.filter(
        (item) =>
          item.primary_organization_code === selected.organization_code &&
          item.employment_status !== "inactive",
      ).length;
      banner.innerHTML = `<div class="status-chip ok">현재 선택: ${escapeHtml(
        selected.organization_name,
      )}</div><p>${escapeHtml(path)} · 코드 ${escapeHtml(
        selected.organization_code,
      )} · 직속 활성 구성원 ${escapeHtml(String(directMembers))}명</p>`;
    }
  }

  const organizationRows = document.querySelectorAll("#data-admin-organizations-body tr[data-organization-code]");
  organizationRows.forEach((row) => {
    row.classList.toggle("active-row", row.dataset.organizationCode === selectedCode);
  });
}

function renderOrganizationDirectoryAndTree() {
  const organizations = organizationAdminState.organizations;
  const workforceItems = organizationAdminState.workforce;
  const filteredOrganizations = getFilteredOrganizations(organizations);
  const { scope, query } = getOrganizationDirectoryFilterState();
  const topLevelCount = filteredOrganizations.filter((item) => !item.parent_organization_code).length;

  setMetricBlock("organization-admin-metrics", [
    { label: "활성 조직", value: `${filteredOrganizations.length}/${organizations.length}개` },
    { label: "최상위 조직", value: `${topLevelCount}개` },
    {
      label: "현재 필터",
      value:
        scope === "top_level"
          ? "최상위"
          : scope === "selected_subtree"
            ? "선택 하위"
            : query
              ? "검색 결과"
              : "전체",
    },
  ]);

  renderTableRows(
    "data-admin-organizations-body",
    filteredOrganizations.slice(0, 24).map(
      (item) =>
        `<tr data-organization-code="${escapeHtml(item.organization_code)}" data-organization-name="${escapeHtml(
          item.organization_name,
        )}" data-parent-organization-code="${escapeHtml(
          item.parent_organization_code || "",
        )}" data-organization-status="${escapeHtml(
          item.organization_status,
        )}" data-effective-from="${escapeHtml(item.effective_from || "")}" data-effective-to="${escapeHtml(
          item.effective_to || "",
        )}"><td class="mono">${escapeHtml(item.organization_code)}</td><td>${escapeHtml(
          item.organization_name,
        )}</td><td>${escapeHtml(item.organization_status)}</td></tr>`,
    ),
    query ? "검색 조건에 맞는 조직이 없습니다." : "활성 조직이 없습니다.",
  );

  renderOrganizationTree("organization-tree-panel", filteredOrganizations, workforceItems);
  renderOrganizationSelectionState();
  renderOrganizationFilterSummary(filteredOrganizations.length, organizations.length, scope, query);
}

function renderOrganizationFilterSummary(filteredCount, totalCount, scope, query) {
  const target = document.getElementById("organization-filter-summary");
  if (!target) return;

  const scopeLabel =
    scope === "top_level" ? "최상위 조직만" : scope === "selected_subtree" ? "선택 조직 하위만" : "전체 조직";
  const queryLabel = query ? `검색어 "${query}" 적용` : "검색어 없음";

  target.innerHTML = `<span class="status-chip ok">표시 ${escapeHtml(String(filteredCount))}/${escapeHtml(
    String(totalCount),
  )}</span><p>${escapeHtml(scopeLabel)} · ${escapeHtml(queryLabel)}</p>`;
}

function getFilteredWorkforceItems(items) {
  const { query } = getWorkforceFilterState();
  if (!query) return items;

  return items.filter((item) => {
    const haystack = `${item.employee_number} ${item.display_name} ${item.primary_organization_name || ""}`.toLowerCase();
    return haystack.includes(query);
  });
}

function renderWorkforceAdminView() {
  const organizationCode = workforceAdminState.organizationCode;
  const items = workforceAdminState.items;
  const filteredItems = getFilteredWorkforceItems(items);
  const selected = getSelectedWorkforceItem(filteredItems);
  const jobFamilies = new Set(
    filteredItems.map((item) => item.job_family).filter((item) => Boolean(item)),
  ).size;

  setMetricBlock("workforce-admin-metrics", [
    { label: "조직 필터", value: organizationCode || "전체" },
    { label: "활성 인력", value: `${filteredItems.length}/${items.length}명` },
    { label: "직군 정보", value: `${jobFamilies}개` },
  ]);
  if (!selected && filteredItems[0]) {
    setSelectedWorkforceEmployeeNumber(filteredItems[0].employee_number);
  } else if (selected) {
    setSelectedWorkforceEmployeeNumber(selected.employee_number);
  }
  const activeSelected = getSelectedWorkforceItem(filteredItems);

  renderBulletSummary("workforce-directory-summary", summarizeWorkforceDirectory(filteredItems, organizationCode), "조건에 맞는 인력이 없습니다.");
  renderTableRows(
    "data-admin-workforce-body",
    filteredItems.slice(0, 12).map(
      (item) =>
        `<tr data-employee-number="${escapeHtml(item.employee_number)}" data-display-name="${escapeHtml(
          item.display_name,
        )}" data-primary-organization-code="${escapeHtml(item.primary_organization_code || "")}" data-primary-organization-name="${escapeHtml(
          item.primary_organization_name || "",
        )}" data-employment-status="${escapeHtml(item.employment_status)}" data-job-family="${escapeHtml(
          item.job_family || "",
        )}" data-email="${escapeHtml(item.email || "")}"><td class="mono">${escapeHtml(item.employee_number)}</td><td>${escapeHtml(
          item.display_name,
        )}</td><td>${escapeHtml(item.primary_organization_name || "미지정")}</td></tr>`,
    ),
    "조건에 맞는 인력이 없습니다.",
  );
  renderBulletSummary(
    "workforce-selected-summary",
    summarizeSelectedWorkforce(activeSelected, organizationCode, filteredItems),
    "선택 인력 상세가 없습니다.",
  );
  renderBulletSummary(
    "workforce-organization-context-summary",
    summarizeWorkforceOrganizationContext(activeSelected, filteredItems, organizationCode),
    "조직 맥락을 계산하지 못했습니다.",
  );
  renderBulletSummary(
    "workforce-admin-action-summary",
    summarizeWorkforceActions(activeSelected ? [activeSelected] : filteredItems, organizationCode),
    "운영 액션이 없습니다.",
  );
  renderBulletSummary(
    "workforce-impact-summary",
    summarizeWorkforceImpact(activeSelected),
    "후속 영향이 없습니다.",
  );
  renderBulletSummary(
    "workforce-action-preview",
    summarizeWorkforceActionPreview(filteredItems),
    "액션 미리보기를 계산하지 못했습니다.",
  );
  renderWorkforceFilterSummary(filteredItems.length, items.length, organizationCode, getWorkforceFilterState().query);
  renderWorkforceSelectionState(activeSelected);
}

function renderWorkforceFilterSummary(filteredCount, totalCount, organizationCode, query) {
  const target = document.getElementById("workforce-filter-summary");
  if (!target) return;

  const orgLabel = organizationCode || "전체 조직";
  const queryLabel = query ? `검색어 "${query}" 적용` : "검색어 없음";
  target.innerHTML = `<span class="status-chip ok">표시 ${escapeHtml(String(filteredCount))}/${escapeHtml(
    String(totalCount),
  )}</span><p>${escapeHtml(orgLabel)} · ${escapeHtml(queryLabel)}</p>`;
}

function renderWorkforceSelectionState(selected) {
  const banner = document.getElementById("workforce-selected-banner");
  if (banner) {
    if (!selected) {
      banner.innerHTML =
        '<div class="status-chip">선택 대기</div><p>목록에서 인력을 선택하면 현재 작업 대상과 조직 문맥이 여기에 표시됩니다.</p>';
    } else {
      banner.innerHTML = `<div class="status-chip ok">현재 선택: ${escapeHtml(
        selected.display_name,
      )}</div><p>${escapeHtml(selected.primary_organization_name || "미지정")} · 사번 ${escapeHtml(
        selected.employee_number,
      )} · 재직 상태 ${escapeHtml(selected.employment_status)}</p>`;
    }
  }

  const workforceRows = document.querySelectorAll("#data-admin-workforce-body tr[data-employee-number]");
  workforceRows.forEach((row) => {
    row.classList.toggle("active-row", row.dataset.employeeNumber === workforceAdminState.selectedEmployeeNumber);
  });
}

function populateWorkforceForm(record, organizationCodeOverride) {
  const organizationInput = document.getElementById("member-organization-code-input");
  const employeeInput = document.getElementById("member-employee-number-input");
  const displayNameInput = document.getElementById("member-display-name-input");
  const employmentStatusInput = document.getElementById("member-employment-status-input");
  const targetOrganizationInput = document.getElementById("member-target-organization-code-input");
  const jobFamilyInput = document.getElementById("member-job-family-input");
  const emailInput = document.getElementById("member-email-input");

  if (!record) {
    if (organizationInput) organizationInput.value = organizationCodeOverride || workforceAdminState.organizationCode || "";
    if (employeeInput) employeeInput.value = "E1001";
    if (displayNameInput) displayNameInput.value = "홍관리";
    if (employmentStatusInput) employmentStatusInput.value = "active";
    if (targetOrganizationInput) targetOrganizationInput.value = "";
    if (jobFamilyInput) jobFamilyInput.value = "operations";
    if (emailInput) emailInput.value = "ops@example.com";
    return;
  }

  if (organizationInput) {
    organizationInput.value = organizationCodeOverride || record.primary_organization_code || workforceAdminState.organizationCode || "";
  }
  if (employeeInput) employeeInput.value = record.employee_number || "";
  if (displayNameInput) displayNameInput.value = record.display_name || "";
  if (employmentStatusInput) employmentStatusInput.value = record.employment_status || "active";
  if (targetOrganizationInput) targetOrganizationInput.value = "";
  if (jobFamilyInput) jobFamilyInput.value = record.job_family || "";
  if (emailInput) emailInput.value = record.email || "";
}

function summarizeOrganizationActionPreview(organizations, workforceItems) {
  const organizationCode = document.getElementById("organization-code-input")?.value.trim();
  const parentOrganizationCode = document.getElementById("organization-parent-input")?.value.trim();

  if (!organizationCode) {
    return [{ title: "조직 코드 필요", body: "삭제 또는 계층 이동 영향을 보려면 조직 코드를 먼저 입력하세요." }];
  }

  const selected = getOrganizationByCode(organizations, organizationCode);
  if (!selected) {
    return [
      {
        title: "신규 조직 생성 모드",
        body: `${organizationCode} 는 아직 존재하지 않습니다. 상위 조직과 상태를 확인한 뒤 신규 생성할 수 있습니다.`,
      },
      {
        title: "상위 조직 확인",
        body: parentOrganizationCode
          ? `${parentOrganizationCode} 아래로 신규 조직이 생성될 예정입니다. 상위 조직 코드가 실제로 존재하는지 확인하세요.`
          : "최상위 조직으로 신규 생성될 예정입니다.",
      },
    ];
  }

  const descendantCodes = getOrganizationDescendantCodes(organizations, organizationCode);
  const activeDirectMembers = workforceItems.filter(
    (item) =>
      item.primary_organization_code === organizationCode && item.employment_status !== "inactive",
  ).length;
  const activeSubtreeMembers = workforceItems.filter(
    (item) =>
      descendantCodes.includes(item.primary_organization_code) && item.employment_status !== "inactive",
  ).length;
  const activeChildren = organizations.filter(
    (item) =>
      item.parent_organization_code === organizationCode && item.organization_status !== "deleted",
  ).length;

  const nextParent = parentOrganizationCode || null;
  let hierarchyBody = "상위 조직 변경 없음";
  if (nextParent && nextParent === organizationCode) {
    hierarchyBody = "자기 자신을 상위 조직으로 지정할 수 없습니다.";
  } else if (nextParent && descendantCodes.includes(nextParent)) {
    hierarchyBody = "하위 조직 아래로 이동하면 순환 계층이 생겨 허용되지 않습니다.";
  } else if (nextParent && !getOrganizationByCode(organizations, nextParent)) {
    hierarchyBody = "입력한 상위 조직 코드가 존재하지 않습니다.";
  } else if (nextParent && nextParent !== selected.parent_organization_code) {
    const target = getOrganizationByCode(organizations, nextParent);
    hierarchyBody = `${target?.organization_name || nextParent} 아래로 이동하면 현재 하위 ${Math.max(
      descendantCodes.length - 1,
      0,
    )}개 조직이 함께 이동합니다.`;
  } else if (!nextParent && selected.parent_organization_code) {
    hierarchyBody = "최상위 조직으로 승격됩니다.";
  }

  const deleteBlocked = activeChildren > 0 || activeDirectMembers > 0;
  return [
    {
      title: "계층 변경 영향",
      body: hierarchyBody,
    },
    {
      title: "삭제 가능 여부",
      body: deleteBlocked
        ? `지금은 삭제 불가입니다. 직속 하위 조직 ${activeChildren}개, 직속 활성 구성원 ${activeDirectMembers}명이 남아 있습니다.`
        : "직속 하위 조직과 직속 활성 구성원이 없어 삭제 가능합니다.",
    },
    {
      title: "하위 포함 영향",
      body: `현재 조직 포함 하위 ${descendantCodes.length}개 조직, 하위 포함 활성 구성원 ${activeSubtreeMembers}명에 영향을 줄 수 있습니다.`,
    },
  ];
}

function summarizeOrganizationMemberActionPreview(organizations, workforceItems) {
  const organizationCode = document
    .getElementById("organization-member-organization-code-input")
    ?.value.trim();
  const employeeNumber = document
    .getElementById("organization-member-employee-number-input")
    ?.value.trim();
  const targetOrganizationCode = document
    .getElementById("organization-member-target-organization-code-input")
    ?.value.trim();

  if (!organizationCode || !employeeNumber) {
    return [{ title: "구성원 정보 필요", body: "대상 조직과 사번을 입력하면 이동 또는 비활성화 영향을 계산합니다." }];
  }

  const currentOrganization = getOrganizationByCode(organizations, organizationCode);
  const targetOrganization = targetOrganizationCode
    ? getOrganizationByCode(organizations, targetOrganizationCode)
    : null;
  const member = workforceItems.find((item) => item.employee_number === employeeNumber) || null;

  const currentDirectMembers = workforceItems.filter(
    (item) =>
      item.primary_organization_code === organizationCode && item.employment_status !== "inactive",
  ).length;

  if (!member) {
    return [
      {
        title: "신규 구성원 등록 모드",
        body: `${organizationCode} 에 새 구성원을 등록할 예정입니다. 현재 이 조직의 활성 직속 구성원은 ${currentDirectMembers}명입니다.`,
      },
      {
        title: "초기 배치",
        body: targetOrganizationCode
          ? "신규 등록에는 이동 대상 조직을 사용하지 않습니다. 대상 조직 입력은 비워 두세요."
          : "등록 완료 시 선택 조직의 직속 구성원 목록에 즉시 반영됩니다.",
      },
    ];
  }

  if (!targetOrganizationCode) {
    return [
      {
        title: "비활성화 영향",
        body: `${member.display_name} 을 비활성화하면 ${currentOrganization?.organization_name || organizationCode} 의 활성 직속 구성원이 ${Math.max(
          currentDirectMembers - (member.employment_status === "inactive" ? 0 : 1),
          0,
        )}명으로 줄어듭니다.`,
      },
      {
        title: "후속 확인",
        body: "비활성화 후에는 조직 이력과 인력 화면에서 담당자 참조 누락이 없는지 점검해야 합니다.",
      },
    ];
  }

  const targetDirectMembers = workforceItems.filter(
    (item) =>
      item.primary_organization_code === targetOrganizationCode && item.employment_status !== "inactive",
  ).length;

  let movementBody = "";
  if (targetOrganizationCode === organizationCode) {
    movementBody = "같은 조직으로는 이동할 수 없습니다. 대상 조직 코드를 비우면 현재 조직 유지 상태로 수정됩니다.";
  } else if (!targetOrganization) {
    movementBody = "입력한 이동 대상 조직이 존재하지 않습니다.";
  } else {
    movementBody = `${member.display_name} 을 ${targetOrganization.organization_name} 으로 이동하면 현재 조직 활성 인원은 ${Math.max(
      currentDirectMembers - 1,
      0,
    )}명, 대상 조직 활성 인원은 ${targetDirectMembers + 1}명이 됩니다.`;
  }

  return [
    {
      title: "이동 영향",
      body: movementBody,
    },
    {
      title: "현재 소속",
      body: `${member.primary_organization_name} · 재직 상태 ${member.employment_status} · 직군 ${member.job_family || "미지정"}`,
    },
    {
      title: "운영 체크",
      body: "이동 후에는 선택 조직의 직속 구성원 패널과 구성원 이동 이력이 함께 갱신됩니다.",
    },
  ];
}

function renderOrganizationAdminPreviews() {
  renderBulletSummary(
    "organization-action-preview",
    summarizeOrganizationActionPreview(organizationAdminState.organizations, organizationAdminState.workforce),
    "조직 영향 미리보기를 계산하지 못했습니다.",
  );
  renderBulletSummary(
    "organization-member-action-preview",
    summarizeOrganizationMemberActionPreview(organizationAdminState.organizations, organizationAdminState.workforce),
    "구성원 영향 미리보기를 계산하지 못했습니다.",
  );
}

function populateOrganizationForm(record) {
  const codeInput = document.getElementById("organization-code-input");
  const nameInput = document.getElementById("organization-name-input");
  const parentInput = document.getElementById("organization-parent-input");
  const statusInput = document.getElementById("organization-status-input");
  const effectiveFromInput = document.getElementById("organization-effective-from-input");
  const effectiveToInput = document.getElementById("organization-effective-to-input");
  if (!record) return;

  if (codeInput) codeInput.value = record.organization_code || "";
  if (nameInput) nameInput.value = record.organization_name || "";
  if (parentInput) parentInput.value = record.parent_organization_code || "";
  if (statusInput) statusInput.value = record.organization_status || "active";
  if (effectiveFromInput) effectiveFromInput.value = record.effective_from || "";
  if (effectiveToInput) effectiveToInput.value = record.effective_to || "";
  renderOrganizationSelectionState();
  renderOrganizationAdminPreviews();
}

function syncOrganizationSelectionToWorkforce(record) {
  if (!record) return;
  setSelectedOrganizationCode(record.organization_code);
  const filterInput = document.getElementById("organization-filter-input");
  const memberOrganizationInput = document.getElementById("member-organization-code-input");
  const organizationMemberOrganizationInput = document.getElementById(
    "organization-member-organization-code-input",
  );
  if (filterInput) filterInput.value = record.organization_code;
  if (memberOrganizationInput) memberOrganizationInput.value = record.organization_code;
  if (organizationMemberOrganizationInput) {
    organizationMemberOrganizationInput.value = record.organization_code;
  }
  if (organizationAdminState.organizations.length) {
    renderOrganizationDirectoryAndTree();
  } else {
    renderOrganizationSelectionState();
  }
  renderOrganizationAdminPreviews();
}

function populateOrganizationMemberForm(record, organizationCode) {
  const organizationInput = document.getElementById("organization-member-organization-code-input");
  const employeeInput = document.getElementById("organization-member-employee-number-input");
  const displayNameInput = document.getElementById("organization-member-display-name-input");
  const employmentStatusInput = document.getElementById("organization-member-employment-status-input");
  const targetOrganizationInput = document.getElementById(
    "organization-member-target-organization-code-input",
  );
  const jobFamilyInput = document.getElementById("organization-member-job-family-input");
  const emailInput = document.getElementById("organization-member-email-input");

  if (organizationInput) organizationInput.value = organizationCode || "";
  if (!record) {
    if (employeeInput) employeeInput.value = "E3001";
    if (displayNameInput) displayNameInput.value = "정조직";
    if (employmentStatusInput) employmentStatusInput.value = "active";
    if (targetOrganizationInput) targetOrganizationInput.value = "";
    if (jobFamilyInput) jobFamilyInput.value = "operations";
    if (emailInput) emailInput.value = "org-admin@example.com";
    renderOrganizationAdminPreviews();
    return;
  }

  if (employeeInput) employeeInput.value = record.employee_number || "";
  if (displayNameInput) displayNameInput.value = record.display_name || "";
  if (employmentStatusInput) employmentStatusInput.value = record.employment_status || "active";
  if (targetOrganizationInput) targetOrganizationInput.value = "";
  if (jobFamilyInput) jobFamilyInput.value = record.job_family || "";
  if (emailInput) emailInput.value = record.email || "";
  renderOrganizationAdminPreviews();
}

function renderOrganizationTree(targetId, organizations, workforceItems) {
  const target = document.getElementById(targetId);
  if (!target) return;

  if (!organizations.length) {
    target.innerHTML = `<div class="empty-state">표시할 조직 트리가 없습니다.</div>`;
    return;
  }

  const childrenMap = new Map();
  organizations.forEach((item) => {
    const parent = item.parent_organization_code || "__root__";
    const items = childrenMap.get(parent) || [];
    items.push(item);
    childrenMap.set(parent, items);
  });
  childrenMap.forEach((items) => {
    items.sort((left, right) => left.organization_code.localeCompare(right.organization_code));
  });

  const depthMap = getOrganizationDepthMap(organizations);
  const selectedCode = getSelectedOrganizationCode();
  const memberCountByOrganization = new Map();
  workforceItems.forEach((item) => {
    memberCountByOrganization.set(
      item.primary_organization_code,
      (memberCountByOrganization.get(item.primary_organization_code) || 0) + 1,
    );
  });

  function renderNodes(parentCode) {
    const nodes = childrenMap.get(parentCode) || [];
    if (!nodes.length) return "";

    return `<ul>${nodes
      .map((item) => {
        const memberCount = memberCountByOrganization.get(item.organization_code) || 0;
        const depth = depthMap.get(item.organization_code) || 0;
        const childrenHtml = renderNodes(item.organization_code);
        return `<li><button class="tree-node${selectedCode === item.organization_code ? " active" : ""}" type="button" data-organization-code="${escapeHtml(
          item.organization_code,
        )}" data-organization-name="${escapeHtml(item.organization_name)}" data-parent-organization-code="${escapeHtml(
          item.parent_organization_code || "",
        )}" data-organization-status="${escapeHtml(item.organization_status)}" data-effective-from="${escapeHtml(
          item.effective_from || "",
        )}" data-effective-to="${escapeHtml(
          item.effective_to || "",
        )}"><strong>${escapeHtml(item.organization_name)}</strong><small>${escapeHtml(
          getOrganizationLevelLabel(depth),
        )} · ${escapeHtml(item.organization_code)} · 구성원 ${memberCount}명</small></button>${childrenHtml}</li>`;
      })
      .join("")}</ul>`;
  }

  target.innerHTML = `<div class="tree-root">${renderNodes("__root__")}</div>`;
  renderOrganizationSelectionState();
}

function summarizeSelectedOrganization(organizations, workforceItems, organizationCode) {
  const selected =
    organizations.find((item) => item.organization_code === organizationCode) || organizations[0] || null;
  if (!selected) {
    return [{ title: "선택 조직 없음", body: "조직을 먼저 생성하거나 선택하세요." }];
  }

  const descendantCodes = getOrganizationDescendantCodes(organizations, selected.organization_code);
  const directMembers = workforceItems.filter(
    (item) => item.primary_organization_code === selected.organization_code,
  ).length;
  const descendantMembers = workforceItems.filter((item) =>
    descendantCodes.includes(item.primary_organization_code),
  ).length;

  return [
    {
      title: `${selected.organization_name} (${selected.organization_code})`,
      body: `상위 조직 ${selected.parent_organization_code || "없음"} · 상태 ${selected.organization_status}`,
    },
    {
      title: "하위 구조",
      body: `자기 포함 ${descendantCodes.length}개 조직이 연결되어 있습니다.`,
    },
    {
      title: "구성원 범위",
      body: `직속 구성원 ${directMembers}명 · 하위 포함 ${descendantMembers}명입니다.`,
    },
  ];
}

function summarizeOrganizationStructure(snapshot) {
  if (!snapshot) {
    return [{ title: "선택 조직 없음", body: "조직을 먼저 생성하거나 선택하세요." }];
  }

  const ancestorPath = snapshot.ancestors.length
    ? snapshot.ancestors.map((item) => item.organization_name).join(" > ")
    : "최상위 조직";
  const childCodes = snapshot.children.length
    ? snapshot.children.map((item) => item.organization_code).join(", ")
    : "직속 하위 조직 없음";

  return [
    {
      title: `${snapshot.organization_name} (${snapshot.organization_code})`,
      body: `상위 경로 ${ancestorPath} · 상태 ${snapshot.organization_status}`,
    },
    {
      title: "직속 하위 조직",
      body: `${snapshot.children.length}개 · ${childCodes}`,
    },
    {
      title: "조직 범위",
      body: `하위 포함 ${snapshot.subtree_organization_count}개 조직 · 직속 구성원 ${snapshot.direct_member_count}명 · 하위 포함 활성 구성원 ${snapshot.subtree_active_member_count}명`,
    },
  ];
}

function summarizeDirectMembers(workforceItems, organizationCode) {
  const directMembers = workforceItems.filter(
    (item) => item.primary_organization_code === organizationCode,
  );

  if (!directMembers.length) {
    return [{ title: "직속 구성원 없음", body: "선택 조직에 직속 구성원이 없습니다." }];
  }

  return [
    {
      title: `직속 구성원 ${directMembers.length}명`,
      body: `${organizationCode} 기준으로 인력 관리 화면과 바로 연결할 수 있습니다.`,
    },
    {
      title: "빠른 이동",
      body: "인력 관리 화면에서 같은 조직 코드를 필터로 열면 바로 이어서 수정할 수 있습니다.",
    },
    {
      title: "대표 직군",
      body: `${[...new Set(directMembers.map((item) => item.job_family).filter(Boolean))].slice(0, 3).join(", ") || "미지정"} 중심으로 구성됩니다.`,
    },
  ];
}

async function createOrganizationDummyData(baseUrl) {
  const organizations = [
    ["biz_platform", "플랫폼사업부", null],
    ["team_integration", "통합플랫폼팀", "biz_platform"],
    ["group_data_hub", "데이터허브그룹", "team_integration"],
    ["part_ingestion", "수집연계파트", "group_data_hub"],
    ["part_normalization", "표준화파트", "group_data_hub"],
    ["team_delivery", "전달운영팀", "biz_platform"],
    ["group_release", "릴리스그룹", "team_delivery"],
    ["part_ci", "CI운영파트", "group_release"],
    ["biz_business", "업무혁신사업부", null],
    ["team_alm", "ALM운영팀", "biz_business"],
    ["group_project", "프로젝트그룹", "team_alm"],
    ["part_pm", "프로젝트관리파트", "group_project"],
  ];
  const members = [
    ["part_ingestion", "E2001", "김수집", "integration_engineering", "ingestion@example.com"],
    ["part_normalization", "E2002", "박표준", "data_management", "normalization@example.com"],
    ["part_ci", "E2003", "이릴리스", "devops", "release@example.com"],
    ["part_pm", "E2004", "최프로젝트", "project_management", "pm@example.com"],
  ];

  for (const [organizationCode, organizationName, parentOrganizationCode] of organizations) {
    await sendJson(baseUrl, "admin/master-data/organizations", "POST", {
      organization_code: organizationCode,
      organization_name: organizationName,
      parent_organization_code: parentOrganizationCode,
      organization_status: "active",
      effective_from: "2026-04-08T00:00:00Z",
      effective_to: null,
    });
  }

  for (const [organizationCode, employeeNumber, displayName, jobFamily, email] of members) {
    await sendJson(baseUrl, `admin/master-data/organizations/${organizationCode}/members`, "POST", {
      employee_number: employeeNumber,
      display_name: displayName,
      employment_status: "active",
      job_family: jobFamily,
      email,
    });
  }

  setSelectedOrganizationCode("biz_platform");
}

async function loadDataAdminLiveData() {
  const filterInput = document.getElementById("organization-filter-input");
  if (!filterInput) return;

  const baseUrl = getActiveApiBaseUrl();
  const organizationCode = filterInput.value.trim();
  localStorage.setItem(PROTOTYPE_API_BASE_KEY, baseUrl);

  setApiStatus("loading", `통합 데이터 관리 화면의 라이브 데이터를 불러오는 중입니다. ${baseUrl}`);

  try {
    const [syncRuns, organizations, workforce, projects, workItems] = await Promise.all([
      fetchJson(baseUrl, "admin/sync-runs"),
      fetchJson(baseUrl, "admin/master-data/organizations", { organization_status: "active" }),
      fetchJson(baseUrl, "admin/master-data/workforce", {
        employment_status: "active",
        primary_organization_code: organizationCode,
      }),
      fetchJson(baseUrl, "admin/projects"),
      fetchJson(baseUrl, "admin/work-items"),
    ]);

    const syncRunItems = syncRuns?.items || [];
    const organizationItems = organizations?.items || [];
    const workforceItems = workforce?.items || [];
    const projectItems = projects?.items || [];
    const workItemItems = workItems?.items || [];

    setMetricBlock("data-admin-metrics", [
      { label: "내부 ALM", value: `${projectItems.length}/${workItemItems.length}` },
      { label: "외부 시스템", value: `${new Set(syncRunItems.map((item) => item.source_system)).size}개` },
      { label: "조직/인력", value: `${organizationItems.length}/${workforceItems.length}` },
    ]);

    renderTableRows(
      "alm-projects-body",
      projectItems.slice(0, 6).map(
        (item) =>
          `<tr><td class="mono">${escapeHtml(item.project_code)}</td><td>${escapeHtml(
            item.project_name,
          )}</td><td>${escapeHtml(item.project_status)}</td></tr>`,
      ),
      "내부 ALM 프로젝트가 없습니다.",
    );
    renderTableRows(
      "alm-work-items-body",
      workItemItems.slice(0, 6).map(
        (item) =>
          `<tr><td class="mono">${escapeHtml(item.work_item_key)}</td><td>${escapeHtml(
            item.assignee_display_name || "미지정",
          )}</td><td>${escapeHtml(item.current_common_status)}</td></tr>`,
      ),
      "내부 ALM 업무 항목이 없습니다.",
    );

    renderBulletSummary("external-jira-summary", summarizeExternalRuns(syncRunItems, "jira"), "실행 이력이 없습니다.");
    renderBulletSummary(
      "external-bitbucket-summary",
      summarizeExternalRuns(syncRunItems, "bitbucket"),
      "실행 이력이 없습니다.",
    );
    renderBulletSummary(
      "external-bamboo-summary",
      summarizeExternalRuns(syncRunItems, "bamboo"),
      "실행 이력이 없습니다.",
    );
    renderBulletSummary(
      "external-confluence-summary",
      summarizeExternalRuns(syncRunItems, "confluence"),
      "실행 이력이 없습니다.",
    );

    renderTableRows(
      "data-admin-organizations-body",
      organizationItems.slice(0, 6).map(
        (item) =>
          `<tr><td class="mono">${escapeHtml(item.organization_code)}</td><td>${escapeHtml(
            item.organization_name,
          )}</td><td>${escapeHtml(item.organization_status)}</td></tr>`,
      ),
      "조직 데이터가 없습니다.",
    );
    renderBulletSummary(
      "data-admin-organization-detail",
      organizationItems.slice(0, 3).map((item) => ({
        title: `${item.organization_name} (${item.organization_code})`,
        body: `상위 조직 ${item.parent_organization_code || "없음"} · 유효 시작 ${item.effective_from || "-"}`,
      })),
      "조직 상세 데이터가 없습니다.",
    );

    renderBulletSummary(
      "data-admin-workforce-summary",
      workforceItems.slice(0, 4).map((item) => ({
        title: `${item.display_name} (${item.employee_number})`,
        body: `${item.primary_organization_name} · ${item.employment_status}${item.job_family ? ` · ${item.job_family}` : ""}`,
      })),
      "선택 조직에 속한 인력이 없습니다.",
    );
    renderTableRows(
      "data-admin-workforce-body",
      workforceItems.slice(0, 6).map(
        (item) =>
          `<tr><td class="mono">${escapeHtml(item.employee_number)}</td><td>${escapeHtml(
            item.display_name,
          )}</td><td>${escapeHtml(item.primary_organization_name)}</td></tr>`,
      ),
      "선택 조직 인력이 없습니다.",
    );

    setApiStatus("ok", `통합 데이터 관리 화면 연결 완료. 조직 필터 ${organizationCode || "없음"} 기준입니다.`);
  } catch (error) {
    setMetricBlock("data-admin-metrics", [
      { label: "내부 ALM", value: "-" },
      { label: "외부 시스템", value: "-" },
      { label: "조직/인력", value: "-" },
    ]);
    renderTableRows("alm-projects-body", [], "내부 ALM 프로젝트를 불러오지 못했습니다.");
    renderTableRows("alm-work-items-body", [], "내부 ALM 업무 항목을 불러오지 못했습니다.");
    renderBulletSummary("external-jira-summary", [], "외부 시스템 데이터를 불러오지 못했습니다.");
    renderBulletSummary("external-bitbucket-summary", [], "외부 시스템 데이터를 불러오지 못했습니다.");
    renderBulletSummary("external-bamboo-summary", [], "외부 시스템 데이터를 불러오지 못했습니다.");
    renderBulletSummary("external-confluence-summary", [], "외부 시스템 데이터를 불러오지 못했습니다.");
    renderTableRows("data-admin-organizations-body", [], "조직 데이터를 불러오지 못했습니다.");
    renderBulletSummary("data-admin-organization-detail", [], "조직 상세 데이터를 불러오지 못했습니다.");
    renderBulletSummary("data-admin-workforce-summary", [], "인력 데이터를 불러오지 못했습니다.");
    renderTableRows("data-admin-workforce-body", [], "인력 목록을 불러오지 못했습니다.");
    setApiStatus("danger", `통합 데이터 관리 화면 연결 실패: ${error.message}`);
  }
}

async function loadDataAlmLiveData() {
  const baseUrl = getActiveApiBaseUrl();
  localStorage.setItem(PROTOTYPE_API_BASE_KEY, baseUrl);
  setApiStatus("loading", `자체 ALM 데이터를 불러오는 중입니다. ${baseUrl}`);

  try {
    const [projects, workItems] = await Promise.all([
      fetchJson(baseUrl, "admin/projects"),
      fetchJson(baseUrl, "admin/work-items"),
    ]);

    const projectItems = projects?.items || [];
    const workItemItems = workItems?.items || [];
    const referencedItems = workItemItems.filter(
      (item) => item.owning_organization_name || item.assignee_display_name || item.reporter_display_name,
    ).length;

    setMetricBlock("data-alm-metrics", [
      { label: "프로젝트", value: `${projectItems.length}건` },
      { label: "업무 항목", value: `${workItemItems.length}건` },
      { label: "조직/인력 참조", value: `${referencedItems}건` },
    ]);

    renderTableRows(
      "alm-projects-body",
      projectItems.slice(0, 8).map(
        (item) =>
          `<tr><td class="mono">${escapeHtml(item.project_code)}</td><td>${escapeHtml(
            item.project_name,
          )}</td><td>${escapeHtml(item.project_status)}</td></tr>`,
      ),
      "내부 ALM 프로젝트가 없습니다.",
    );
    renderTableRows(
      "alm-work-items-body",
      workItemItems.slice(0, 8).map(
        (item) =>
          `<tr><td class="mono">${escapeHtml(item.work_item_key)}</td><td>${escapeHtml(
            item.assignee_display_name || "미지정",
          )}</td><td>${escapeHtml(item.current_common_status || "미정")}</td></tr>`,
      ),
      "내부 ALM 업무 항목이 없습니다.",
    );
    renderBulletSummary(
      "alm-detail-summary",
      summarizeAlmDetail(projectItems, workItemItems),
      "대표 상세가 없습니다.",
    );
    renderBulletSummary(
      "alm-action-summary",
      summarizeAlmActions(projectItems, workItemItems),
      "운영 액션이 없습니다.",
    );
    renderBulletSummary(
      "alm-impact-summary",
      summarizeAlmImpact(projectItems, workItemItems),
      "영향 범위를 계산하지 못했습니다.",
    );

    setApiStatus(
      "ok",
      `자체 ALM 데이터 연결 완료. 프로젝트 ${projectItems.length}건, 업무 항목 ${workItemItems.length}건입니다.`,
    );
  } catch (error) {
    setMetricBlock("data-alm-metrics", [
      { label: "프로젝트", value: "-" },
      { label: "업무 항목", value: "-" },
      { label: "조직/인력 참조", value: "-" },
    ]);
    renderTableRows("alm-projects-body", [], "내부 ALM 프로젝트를 불러오지 못했습니다.");
    renderTableRows("alm-work-items-body", [], "내부 ALM 업무 항목을 불러오지 못했습니다.");
    renderBulletSummary("alm-detail-summary", [], "대표 상세를 불러오지 못했습니다.");
    renderBulletSummary("alm-action-summary", [], "운영 액션을 계산하지 못했습니다.");
    renderBulletSummary("alm-impact-summary", [], "영향 범위를 계산하지 못했습니다.");
    setApiStatus("danger", `자체 ALM 데이터 연결 실패: ${error.message}`);
  }
}

function getExternalSystemName(sourceSystem) {
  if (sourceSystem === "jira") return "`Jira`";
  if (sourceSystem === "bitbucket") return "`Bitbucket`";
  if (sourceSystem === "bamboo") return "`Bamboo`";
  if (sourceSystem === "confluence") return "`Confluence`";
  return sourceSystem;
}

async function loadExternalSystemLiveData(sourceSystem) {
  const baseUrl = getActiveApiBaseUrl();
  const systemLabel = getExternalSystemName(sourceSystem);
  localStorage.setItem(PROTOTYPE_API_BASE_KEY, baseUrl);
  setApiStatus("loading", `${systemLabel} 실행 이력을 불러오는 중입니다. ${baseUrl}`);

  try {
    const syncRuns = await fetchJson(baseUrl, "admin/sync-runs", {
      source_system: sourceSystem,
    });
    const systemItems = syncRuns?.items || [];
    const latestItem = systemItems[0];

    setMetricBlock("external-system-metrics", [
      { label: "시스템", value: systemLabel },
      { label: "실행 건수", value: `${systemItems.length}건` },
      { label: "최근 상태", value: latestItem ? formatRunStatus(latestItem) : "없음" },
    ]);
    renderBulletSummary(
      "external-system-summary",
      summarizeExternalRuns(systemItems, sourceSystem),
      `${systemLabel} 실행 이력이 없습니다.`,
    );
    renderBulletSummary(
      "external-system-detail",
      summarizeExternalDetail(systemItems, sourceSystem),
      `${systemLabel} 대표 실행 상세가 없습니다.`,
    );
    renderBulletSummary(
      "external-system-actions",
      summarizeExternalActions(systemItems, sourceSystem),
      `${systemLabel} 운영 액션이 없습니다.`,
    );
    renderBulletSummary("external-system-scope", [
      {
        title: "현재 노출 범위",
        body: `${systemLabel} 화면은 현재 시스템별 실행 이력과 최근 상태를 우선 노출합니다.`,
      },
      {
        title: "다음 확장",
        body: "원시 적재, 표준화, 매핑, 오류 큐를 시스템별 상세 플로우로 연결할 예정입니다.",
      },
    ]);

    setApiStatus("ok", `${systemLabel} 실행 이력 연결 완료. 최근 ${systemItems.length}건을 반영했습니다.`);
  } catch (error) {
    setMetricBlock("external-system-metrics", [
      { label: "시스템", value: systemLabel },
      { label: "실행 건수", value: "-" },
      { label: "최근 상태", value: "-" },
    ]);
    renderBulletSummary("external-system-summary", [], `${systemLabel} 실행 이력을 불러오지 못했습니다.`);
    renderBulletSummary("external-system-detail", [], `${systemLabel} 대표 실행 상세를 불러오지 못했습니다.`);
    renderBulletSummary("external-system-actions", [], `${systemLabel} 운영 액션을 계산하지 못했습니다.`);
    setApiStatus("danger", `${systemLabel} 실행 이력 연결 실패: ${error.message}`);
  }
}

async function loadOrganizationsAdminLiveData() {
  const baseUrl = getActiveApiBaseUrl();
  localStorage.setItem(PROTOTYPE_API_BASE_KEY, baseUrl);
  setApiStatus("loading", `조직 마스터를 불러오는 중입니다. ${baseUrl}`);

  try {
    const [organizations, workforce] = await Promise.all([
      fetchJson(baseUrl, "admin/master-data/organizations", {
        organization_status: "active",
      }),
      fetchJson(baseUrl, "admin/master-data/workforce", {
        employment_status: "active",
      }),
    ]);
    const organizationItems = organizations?.items || [];
    const workforceItems = workforce?.items || [];
    const selectedCode = getSelectedOrganizationCode() || organizationItems[0]?.organization_code || "";
    renderBulletSummary(
      "data-admin-organization-detail",
      organizationItems.slice(0, 3).map((item) => ({
        title: `${item.organization_name} (${item.organization_code})`,
        body: `상위 조직 ${item.parent_organization_code || "없음"} · 유효 시작 ${item.effective_from || "-"} · 유효 종료 ${item.effective_to || "-"}`,
      })),
      "조직 상세 정보가 없습니다.",
    );
    renderBulletSummary(
      "organization-admin-action-summary",
      summarizeOrganizationActions(organizationItems),
      "운영 액션이 없습니다.",
    );
    renderBulletSummary(
      "organization-admin-impact-summary",
      summarizeOrganizationImpact(organizationItems),
      "영향 범위가 없습니다.",
    );
    let organizationStructure = null;
    if (selectedCode) {
      const structureResponse = await fetchJson(
        baseUrl,
        `admin/master-data/organizations/${selectedCode}/structure`,
      );
      organizationStructure = structureResponse?.item || null;
    }
    renderBulletSummary(
      "organization-tree-summary",
      organizationStructure
        ? summarizeOrganizationStructure(organizationStructure)
        : summarizeSelectedOrganization(organizationItems, workforceItems, selectedCode),
      "선택 조직 요약이 없습니다.",
    );
    renderTableRows(
      "organization-direct-members-body",
      workforceItems
        .filter((item) => item.primary_organization_code === selectedCode)
        .slice(0, 8)
        .map(
          (item) =>
            `<tr data-employee-number="${escapeHtml(item.employee_number)}" data-display-name="${escapeHtml(
              item.display_name,
            )}" data-employment-status="${escapeHtml(
              item.employment_status,
            )}" data-job-family="${escapeHtml(item.job_family || "")}" data-email="${escapeHtml(
              item.email || "",
            )}"><td class="mono">${escapeHtml(item.employee_number)}</td><td>${escapeHtml(
              item.display_name,
            )}</td><td>${escapeHtml(item.job_family || "미지정")}</td></tr>`,
        ),
      "선택 조직의 직속 구성원이 없습니다.",
    );
    renderOrganizationSelectionState();
    renderBulletSummary(
      "organization-direct-members-summary",
      summarizeDirectMembers(workforceItems, selectedCode),
      "조직-인력 연결 요약이 없습니다.",
    );

    let organizationHistoryItems = [];
    let organizationMemberHistoryItems = [];
    if (selectedCode) {
      const [organizationHistory, organizationMemberHistory] = await Promise.all([
        fetchJson(baseUrl, `admin/master-data/organizations/${selectedCode}/history`),
        fetchJson(baseUrl, `admin/master-data/organizations/${selectedCode}/member-history`),
      ]);
      organizationHistoryItems = organizationHistory?.items || [];
      organizationMemberHistoryItems = organizationMemberHistory?.items || [];
    }
    renderBulletSummary(
      "organization-history-summary",
      organizationHistoryItems.slice(0, 6).map((item) => ({
        title: `${item.action_type} · ${formatRelativeTime(item.changed_at)}`,
        body: item.summary,
      })),
      selectedCode ? "선택 조직의 변경 이력이 없습니다." : "선택된 조직이 없습니다.",
    );
    renderBulletSummary(
      "organization-member-history-summary",
      organizationMemberHistoryItems.slice(0, 6).map((item) => ({
        title: `${item.employee_number} · ${item.action_type}`,
        body: `${item.from_organization_code || "-"} -> ${item.to_organization_code || "-"} · ${formatRelativeTime(item.changed_at)} · ${item.summary}`,
      })),
      selectedCode ? "선택 조직의 구성원 이동 이력이 없습니다." : "선택된 조직이 없습니다.",
    );

    const selectedRecord =
      organizationItems.find((item) => item.organization_code === selectedCode) || organizationItems[0] || null;
    organizationAdminState.organizations = organizationItems;
    organizationAdminState.workforce = workforceItems;
    renderOrganizationDirectoryAndTree();
    if (selectedRecord) {
      populateOrganizationForm(selectedRecord);
      syncOrganizationSelectionToWorkforce(selectedRecord);
      const selectedMember =
        workforceItems.find((item) => item.primary_organization_code === selectedRecord.organization_code) || null;
      populateOrganizationMemberForm(selectedMember, selectedRecord.organization_code);
    } else {
      renderOrganizationAdminPreviews();
    }

    setApiStatus("ok", `조직 마스터 연결 완료. 활성 조직 ${organizationItems.length}개입니다.`);
  } catch (error) {
    organizationAdminState.organizations = [];
    organizationAdminState.workforce = [];
    setMetricBlock("organization-admin-metrics", [
      { label: "활성 조직", value: "-" },
      { label: "최상위 조직", value: "-" },
      { label: "상세 카드", value: "-" },
    ]);
    renderTableRows("data-admin-organizations-body", [], "조직 목록을 불러오지 못했습니다.");
    renderBulletSummary("data-admin-organization-detail", [], "조직 상세를 불러오지 못했습니다.");
    renderBulletSummary("organization-admin-action-summary", [], "운영 액션을 계산하지 못했습니다.");
    renderBulletSummary("organization-admin-impact-summary", [], "영향 범위를 계산하지 못했습니다.");
    renderBulletSummary("organization-tree-summary", [], "선택 조직 요약을 계산하지 못했습니다.");
    renderTableRows("organization-direct-members-body", [], "직속 구성원 목록을 불러오지 못했습니다.");
    renderBulletSummary("organization-direct-members-summary", [], "조직-인력 연결 요약을 계산하지 못했습니다.");
    renderBulletSummary("organization-history-summary", [], "조직 변경 이력을 불러오지 못했습니다.");
    renderBulletSummary("organization-member-history-summary", [], "구성원 이동 이력을 불러오지 못했습니다.");
    renderBulletSummary("organization-action-preview", [], "조직 영향 미리보기를 계산하지 못했습니다.");
    renderBulletSummary("organization-member-action-preview", [], "구성원 영향 미리보기를 계산하지 못했습니다.");
    populateOrganizationMemberForm(null, getSelectedOrganizationCode());
    const treePanel = document.getElementById("organization-tree-panel");
    if (treePanel) {
      treePanel.innerHTML = `<div class="empty-state">조직 트리를 불러오지 못했습니다.</div>`;
    }
    setApiStatus("danger", `조직 마스터 연결 실패: ${error.message}`);
  }
}

async function loadWorkforceAdminLiveData() {
  const filterInput = document.getElementById("organization-filter-input");
  if (!filterInput) return;

  const baseUrl = getActiveApiBaseUrl();
  if (!filterInput.value.trim() || filterInput.value.trim() === "default_org") {
    const selectedCode = getSelectedOrganizationCode();
    if (selectedCode) {
      filterInput.value = selectedCode;
    }
  }

  const organizationCode = filterInput.value.trim();
  localStorage.setItem(PROTOTYPE_API_BASE_KEY, baseUrl);
  setApiStatus("loading", `인력 마스터를 불러오는 중입니다. ${organizationCode || "전체"} 조직 기준입니다.`);

  try {
    const workforce = await fetchJson(baseUrl, "admin/master-data/workforce", {
      employment_status: "active",
      primary_organization_code: organizationCode,
    });
    const workforceItems = workforce?.items || [];
    workforceAdminState.items = workforceItems;
    workforceAdminState.organizationCode = organizationCode;
    if (!workforceItems.find((item) => item.employee_number === workforceAdminState.selectedEmployeeNumber)) {
      setSelectedWorkforceEmployeeNumber(workforceItems[0]?.employee_number || "");
    }
    renderWorkforceAdminView();
    populateWorkforceForm(getSelectedWorkforceItem(getFilteredWorkforceItems(workforceItems)), organizationCode);

    setApiStatus("ok", `인력 마스터 연결 완료. ${organizationCode || "전체"} 기준 ${workforceItems.length}명입니다.`);
  } catch (error) {
    workforceAdminState.items = [];
    workforceAdminState.organizationCode = organizationCode;
    setMetricBlock("workforce-admin-metrics", [
      { label: "조직 필터", value: organizationCode || "-" },
      { label: "활성 인력", value: "-" },
      { label: "직군 정보", value: "-" },
    ]);
    renderBulletSummary("workforce-directory-summary", [], "인력 범위 요약을 불러오지 못했습니다.");
    renderTableRows("data-admin-workforce-body", [], "인력 목록을 불러오지 못했습니다.");
    renderBulletSummary("workforce-selected-summary", [], "선택 인력 상세를 불러오지 못했습니다.");
    renderBulletSummary("workforce-organization-context-summary", [], "조직 맥락을 계산하지 못했습니다.");
    renderBulletSummary("workforce-admin-action-summary", [], "운영 액션을 계산하지 못했습니다.");
    renderBulletSummary("workforce-impact-summary", [], "후속 영향 계산에 실패했습니다.");
    renderBulletSummary("workforce-action-preview", [], "액션 미리보기를 계산하지 못했습니다.");
    const filterSummary = document.getElementById("workforce-filter-summary");
    if (filterSummary) {
      filterSummary.innerHTML = `<span class="status-chip danger">필터 오류</span><p>필터 요약을 계산하지 못했습니다.</p>`;
    }
    renderWorkforceSelectionState(null);
    setApiStatus("danger", `인력 마스터 연결 실패: ${error.message}`);
  }
}

function setupOrganizationAdminActions(load) {
  const saveButton = document.getElementById("organization-save-button");
  const deleteButton = document.getElementById("organization-delete-button");
  const status = document.getElementById("organization-action-status");
  if (!saveButton || !deleteButton || !status) return;

  saveButton.addEventListener("click", async () => {
    const baseUrl = getActiveApiBaseUrl();
    const organizationCode = document.getElementById("organization-code-input")?.value.trim();
    const organizationName = document.getElementById("organization-name-input")?.value.trim();
    const parentOrganizationCode = document.getElementById("organization-parent-input")?.value.trim();
    const organizationStatus = document.getElementById("organization-status-input")?.value.trim();
    const effectiveFrom = document.getElementById("organization-effective-from-input")?.value.trim();
    const effectiveTo = document.getElementById("organization-effective-to-input")?.value.trim();

    if (!organizationCode || !organizationName || !organizationStatus) {
      status.textContent = "조직 코드, 조직명, 상태는 필수입니다.";
      return;
    }

    status.textContent = "조직 등록/수정 요청 중입니다.";

    try {
      await sendJson(baseUrl, "admin/master-data/organizations", "POST", {
        organization_code: organizationCode,
        organization_name: organizationName,
        parent_organization_code: parentOrganizationCode || null,
        organization_status: organizationStatus,
        effective_from: effectiveFrom || null,
        effective_to: effectiveTo || null,
      });
      status.textContent = `조직 ${organizationCode} 저장이 완료되었습니다.`;
      setSelectedOrganizationCode(organizationCode);
      await load();
    } catch (error) {
      status.textContent = `조직 저장 실패: ${error.message}`;
    }
  });

  deleteButton.addEventListener("click", async () => {
    const baseUrl = getActiveApiBaseUrl();
    const organizationCode = document.getElementById("organization-code-input")?.value.trim();

    if (!organizationCode) {
      status.textContent = "삭제할 조직 코드를 입력하세요.";
      return;
    }

    status.textContent = "조직 삭제 요청 중입니다.";

    try {
      await sendJson(baseUrl, `admin/master-data/organizations/${organizationCode}`, "DELETE");
      status.textContent = `조직 ${organizationCode} 삭제가 완료되었습니다.`;
      if (getSelectedOrganizationCode() === organizationCode) {
        setSelectedOrganizationCode("");
      }
      await load();
    } catch (error) {
      status.textContent = `조직 삭제 실패: ${error.message}`;
    }
  });

  const memberSaveButton = document.getElementById("organization-member-save-button");
  const memberRemoveButton = document.getElementById("organization-member-remove-button");
  const memberStatus = document.getElementById("organization-member-action-status");
  if (memberSaveButton && memberRemoveButton && memberStatus) {
    memberSaveButton.addEventListener("click", async () => {
      const baseUrl = getActiveApiBaseUrl();
      const organizationCode = document.getElementById("organization-member-organization-code-input")?.value.trim();
      const targetOrganizationCode = document
        .getElementById("organization-member-target-organization-code-input")
        ?.value.trim();
      const employeeNumber = document.getElementById("organization-member-employee-number-input")?.value.trim();
      const displayName = document.getElementById("organization-member-display-name-input")?.value.trim();
      const employmentStatus = document
        .getElementById("organization-member-employment-status-input")
        ?.value.trim();
      const jobFamily = document.getElementById("organization-member-job-family-input")?.value.trim();
      const email = document.getElementById("organization-member-email-input")?.value.trim();

      if (!organizationCode || !employeeNumber) {
        memberStatus.textContent = "대상 조직과 사번은 필수입니다.";
        return;
      }

      memberStatus.textContent = "직속 구성원 저장 요청 중입니다.";

      try {
        if (targetOrganizationCode) {
          await sendJson(
            baseUrl,
            `admin/master-data/organizations/${organizationCode}/members/${employeeNumber}`,
            "PATCH",
            {
              display_name: displayName || null,
              employment_status: employmentStatus || null,
              primary_organization_code: targetOrganizationCode,
              job_family: jobFamily || null,
              email: email || null,
            },
          );
          setSelectedOrganizationCode(targetOrganizationCode);
        } else {
          await sendJson(baseUrl, `admin/master-data/organizations/${organizationCode}/members`, "POST", {
            employee_number: employeeNumber,
            display_name: displayName,
            employment_status: employmentStatus || "active",
            job_family: jobFamily || null,
            email: email || null,
          });
          setSelectedOrganizationCode(organizationCode);
        }
        memberStatus.textContent = `직속 구성원 ${employeeNumber} 저장이 완료되었습니다.`;
        await load();
      } catch (error) {
        memberStatus.textContent = `직속 구성원 저장 실패: ${error.message}`;
      }
    });

    memberRemoveButton.addEventListener("click", async () => {
      const baseUrl = getActiveApiBaseUrl();
      const organizationCode = document.getElementById("organization-member-organization-code-input")?.value.trim();
      const employeeNumber = document.getElementById("organization-member-employee-number-input")?.value.trim();

      if (!organizationCode || !employeeNumber) {
        memberStatus.textContent = "대상 조직과 사번은 필수입니다.";
        return;
      }

      memberStatus.textContent = "직속 구성원 제거 요청 중입니다.";

      try {
        await sendJson(
          baseUrl,
          `admin/master-data/organizations/${organizationCode}/members/${employeeNumber}`,
          "DELETE",
        );
        memberStatus.textContent = `직속 구성원 ${employeeNumber} 비활성화가 완료되었습니다.`;
        await load();
      } catch (error) {
        memberStatus.textContent = `직속 구성원 제거 실패: ${error.message}`;
      }
    });
  }

  const dummyButton = document.getElementById("organization-dummy-button");
  if (dummyButton) {
    dummyButton.addEventListener("click", async () => {
      const baseUrl = getActiveApiBaseUrl();
      status.textContent = "사업부 · 팀 · 그룹 · 파트 더미 데이터를 생성하는 중입니다.";
      try {
        await createOrganizationDummyData(baseUrl);
        status.textContent = "더미 데이터 생성이 완료되었습니다.";
        await load();
      } catch (error) {
        status.textContent = `더미 데이터 생성 실패: ${error.message}`;
      }
    });
  }

  const treePanel = document.getElementById("organization-tree-panel");
  if (treePanel) {
    treePanel.addEventListener("click", (event) => {
      const button = event.target.closest("[data-organization-code]");
      if (!button) return;
      const record = {
        organization_code: button.dataset.organizationCode || "",
        organization_name: button.dataset.organizationName || "",
        parent_organization_code: button.dataset.parentOrganizationCode || null,
        organization_status: button.dataset.organizationStatus || "active",
        effective_from: button.dataset.effectiveFrom || null,
        effective_to: button.dataset.effectiveTo || null,
      };
      populateOrganizationForm(record);
      syncOrganizationSelectionToWorkforce(record);
      load();
    });
  }

  const directoryTable = document.getElementById("data-admin-organizations-body");
  if (directoryTable) {
    directoryTable.addEventListener("click", (event) => {
      const row = event.target.closest("tr[data-organization-code]");
      if (!row) return;
      const record = {
        organization_code: row.dataset.organizationCode || "",
        organization_name: row.dataset.organizationName || "",
        parent_organization_code: row.dataset.parentOrganizationCode || null,
        organization_status: row.dataset.organizationStatus || "active",
        effective_from: row.dataset.effectiveFrom || null,
        effective_to: row.dataset.effectiveTo || null,
      };
      populateOrganizationForm(record);
      syncOrganizationSelectionToWorkforce(record);
      load();
    });
  }

  const memberTable = document.getElementById("organization-direct-members-body");
  if (memberTable) {
    memberTable.addEventListener("click", (event) => {
      const row = event.target.closest("tr[data-employee-number]");
      if (!row) return;
      populateOrganizationMemberForm(
        {
          employee_number: row.dataset.employeeNumber || "",
          display_name: row.dataset.displayName || "",
          employment_status: row.dataset.employmentStatus || "active",
          job_family: row.dataset.jobFamily || "",
          email: row.dataset.email || "",
        },
        getSelectedOrganizationCode(),
      );
    });
  }

  ["organization-directory-search-input", "organization-directory-scope-select"].forEach((id) => {
    const element = document.getElementById(id);
    if (!element) return;
    element.addEventListener("input", () => {
      renderOrganizationDirectoryAndTree();
    });
    element.addEventListener("change", () => {
      renderOrganizationDirectoryAndTree();
    });
  });

  const organizationFilterResetButton = document.getElementById("organization-filter-reset-button");
  if (organizationFilterResetButton) {
    organizationFilterResetButton.addEventListener("click", () => {
      const searchInput = document.getElementById("organization-directory-search-input");
      const scopeSelect = document.getElementById("organization-directory-scope-select");
      if (searchInput) searchInput.value = "";
      if (scopeSelect) scopeSelect.value = "all";
      renderOrganizationDirectoryAndTree();
    });
  }

  [
    "organization-code-input",
    "organization-parent-input",
    "organization-member-organization-code-input",
    "organization-member-employee-number-input",
    "organization-member-target-organization-code-input",
  ].forEach((id) => {
    const element = document.getElementById(id);
    if (!element) return;
    element.addEventListener("input", () => {
      renderOrganizationAdminPreviews();
    });
    element.addEventListener("change", () => {
      renderOrganizationAdminPreviews();
    });
  });
}

function setupWorkforceAdminActions(load) {
  const saveButton = document.getElementById("member-save-button");
  const removeButton = document.getElementById("member-remove-button");
  const status = document.getElementById("member-action-status");
  if (!saveButton || !removeButton || !status) return;

  const workforceSearchInput = document.getElementById("workforce-search-input");
  if (workforceSearchInput) {
    workforceSearchInput.addEventListener("input", () => {
      renderWorkforceAdminView();
    });
  }

  const workforceTable = document.getElementById("data-admin-workforce-body");
  if (workforceTable) {
    workforceTable.addEventListener("click", (event) => {
      const row = event.target.closest("tr[data-employee-number]");
      if (!row) return;
      const record = {
        employee_number: row.dataset.employeeNumber || "",
        display_name: row.dataset.displayName || "",
        primary_organization_code: row.dataset.primaryOrganizationCode || "",
        primary_organization_name: row.dataset.primaryOrganizationName || "",
        employment_status: row.dataset.employmentStatus || "active",
        job_family: row.dataset.jobFamily || "",
        email: row.dataset.email || "",
      };
      setSelectedWorkforceEmployeeNumber(record.employee_number);
      populateWorkforceForm(record, record.primary_organization_code);
      renderWorkforceAdminView();
    });
  }

  const workforceFilterResetButton = document.getElementById("workforce-filter-reset-button");
  if (workforceFilterResetButton) {
    workforceFilterResetButton.addEventListener("click", () => {
      const filterInput = document.getElementById("organization-filter-input");
      const searchInput = document.getElementById("workforce-search-input");
      if (filterInput) {
        filterInput.value = getSelectedOrganizationCode() || "";
      }
      if (searchInput) searchInput.value = "";
      setSelectedWorkforceEmployeeNumber("");
      load();
    });
  }

  [
    "member-organization-code-input",
    "member-employee-number-input",
    "member-target-organization-code-input",
    "member-employment-status-input",
  ].forEach((id) => {
    const element = document.getElementById(id);
    if (!element) return;
    element.addEventListener("input", () => {
      renderBulletSummary(
        "workforce-action-preview",
        summarizeWorkforceActionPreview(getFilteredWorkforceItems(workforceAdminState.items)),
        "액션 미리보기를 계산하지 못했습니다.",
      );
    });
    element.addEventListener("change", () => {
      renderBulletSummary(
        "workforce-action-preview",
        summarizeWorkforceActionPreview(getFilteredWorkforceItems(workforceAdminState.items)),
        "액션 미리보기를 계산하지 못했습니다.",
      );
    });
  });

  saveButton.addEventListener("click", async () => {
    const baseUrl = getActiveApiBaseUrl();
    const organizationCode = document.getElementById("member-organization-code-input")?.value.trim();
    const targetOrganizationCode = document.getElementById("member-target-organization-code-input")?.value.trim();
    const employeeNumber = document.getElementById("member-employee-number-input")?.value.trim();
    const displayName = document.getElementById("member-display-name-input")?.value.trim();
    const employmentStatus = document.getElementById("member-employment-status-input")?.value.trim();
    const jobFamily = document.getElementById("member-job-family-input")?.value.trim();
    const email = document.getElementById("member-email-input")?.value.trim();

    if (!organizationCode || !employeeNumber) {
      status.textContent = "대상 조직과 사번은 필수입니다.";
      return;
    }

    status.textContent = "조직 구성원 저장 요청 중입니다.";

    try {
      if (targetOrganizationCode) {
        await sendJson(
          baseUrl,
          `admin/master-data/organizations/${organizationCode}/members/${employeeNumber}`,
          "PATCH",
          {
            display_name: displayName || null,
            employment_status: employmentStatus || null,
            primary_organization_code: targetOrganizationCode,
            job_family: jobFamily ? jobFamily : null,
            email: email ? email : null,
          },
        );
      } else {
        await sendJson(baseUrl, `admin/master-data/organizations/${organizationCode}/members`, "POST", {
          employee_number: employeeNumber,
          display_name: displayName,
          employment_status: employmentStatus || "active",
          job_family: jobFamily || null,
          email: email || null,
        });
      }
      status.textContent = `구성원 ${employeeNumber} 저장이 완료되었습니다.`;
      const filterInput = document.getElementById("organization-filter-input");
      if (filterInput && targetOrganizationCode) {
        filterInput.value = targetOrganizationCode;
      } else if (filterInput) {
        filterInput.value = organizationCode;
      }
      await load();
    } catch (error) {
      status.textContent = `구성원 저장 실패: ${error.message}`;
    }
  });

  removeButton.addEventListener("click", async () => {
    const baseUrl = getActiveApiBaseUrl();
    const organizationCode = document.getElementById("member-organization-code-input")?.value.trim();
    const employeeNumber = document.getElementById("member-employee-number-input")?.value.trim();

    if (!organizationCode || !employeeNumber) {
      status.textContent = "대상 조직과 사번은 필수입니다.";
      return;
    }

    status.textContent = "구성원 제거 요청 중입니다.";

    try {
      await sendJson(
        baseUrl,
        `admin/master-data/organizations/${organizationCode}/members/${employeeNumber}`,
        "DELETE",
      );
      status.textContent = `구성원 ${employeeNumber} 비활성화가 완료되었습니다.`;
      await load();
    } catch (error) {
      status.textContent = `구성원 제거 실패: ${error.message}`;
    }
  });

  const dummyButton = document.getElementById("member-dummy-button");
  if (dummyButton) {
    dummyButton.addEventListener("click", async () => {
      const baseUrl = getActiveApiBaseUrl();
      status.textContent = "사업부 · 팀 · 그룹 · 파트 더미 데이터와 구성원을 생성하는 중입니다.";
      try {
        await createOrganizationDummyData(baseUrl);
        const filterInput = document.getElementById("organization-filter-input");
        if (filterInput) {
          filterInput.value = getSelectedOrganizationCode() || "biz_platform";
        }
        status.textContent = "더미 데이터 생성이 완료되었습니다.";
        await load();
      } catch (error) {
        status.textContent = `더미 데이터 생성 실패: ${error.message}`;
      }
    });
  }
}

function setupLiveOperations() {
  const input = document.getElementById("api-base-url-input");

  const load =
    currentScreen === "admin"
      ? loadAdminLiveData
      : currentScreen === "organization"
        ? loadOrganizationLiveData
        : currentScreen === "data-alm"
            ? loadDataAlmLiveData
            : currentScreen === "data-organizations"
              ? loadOrganizationsAdminLiveData
              : currentScreen === "data-workforce"
                ? loadWorkforceAdminLiveData
                : currentScreen === "data-external-jira"
                  ? () => loadExternalSystemLiveData("jira")
                  : currentScreen === "data-external-bitbucket"
                    ? () => loadExternalSystemLiveData("bitbucket")
                    : currentScreen === "data-external-bamboo"
                      ? () => loadExternalSystemLiveData("bamboo")
                      : currentScreen === "data-external-confluence"
                        ? () => loadExternalSystemLiveData("confluence")
          : null;
  if (input) {
    input.value = resolveApiBaseUrl();
    input.addEventListener("change", () => {
      localStorage.setItem(PROTOTYPE_API_BASE_KEY, input.value.trim());
      updateCurrentApiBaseUrlDisplays();
    });
  }

  updateCurrentApiBaseUrlDisplays();

  if (!load) {
    if (currentScreen === "data-settings") {
      setupDataSettingsPage();
    }
    return;
  }

  const filterInput = document.getElementById("organization-filter-input");
  if (filterInput) {
    filterInput.addEventListener("keydown", (event) => {
      if (event.key === "Enter") {
        load();
      }
    });
  }

  if (currentScreen === "data-organizations") {
    setupOrganizationAdminActions(load);
  }
  if (currentScreen === "data-workforce") {
    setupWorkforceAdminActions(load);
  }

  load();
}

function setupDataSettingsPage() {
  const input = document.getElementById("api-base-url-settings-input");
  const saveButton = document.getElementById("api-settings-save-button");
  const resetButton = document.getElementById("api-settings-reset-button");
  const testButton = document.getElementById("api-settings-test-button");
  const dummyButton = document.getElementById("settings-dummy-button");
  const supportStatus = document.getElementById("settings-support-status");

  if (!input || !saveButton || !resetButton || !testButton) return;

  input.value = resolveApiBaseUrl();
  updateCurrentApiBaseUrlDisplays();
  setApiStatus("loading", `현재 저장된 관리자 API URL은 ${resolveApiBaseUrl()} 입니다.`);

  saveButton.addEventListener("click", () => {
    const value = input.value.trim() || DEFAULT_API_BASE_URL;
    localStorage.setItem(PROTOTYPE_API_BASE_KEY, value);
    updateCurrentApiBaseUrlDisplays();
    setApiStatus("ok", `관리자 API URL을 저장했습니다. ${value}`);
  });

  resetButton.addEventListener("click", () => {
    localStorage.removeItem(PROTOTYPE_API_BASE_KEY);
    input.value = DEFAULT_API_BASE_URL;
    updateCurrentApiBaseUrlDisplays();
    setApiStatus("warn", `설정을 기본값으로 되돌렸습니다. ${DEFAULT_API_BASE_URL}`);
  });

  testButton.addEventListener("click", async () => {
    const value = input.value.trim() || DEFAULT_API_BASE_URL;
    setApiStatus("loading", `헬스 체크를 확인 중입니다. ${value}`);
    try {
      await fetchJson(value, "health");
      localStorage.setItem(PROTOTYPE_API_BASE_KEY, value);
      updateCurrentApiBaseUrlDisplays();
      setApiStatus("ok", `연결 확인 완료. ${value} 에서 health 응답을 받았습니다.`);
    } catch (error) {
      setApiStatus("danger", `연결 확인 실패: ${error.message}`);
    }
  });

  if (dummyButton && supportStatus) {
    dummyButton.addEventListener("click", async () => {
      const value = input.value.trim() || DEFAULT_API_BASE_URL;
      supportStatus.textContent = "조직/인력 더미 데이터를 생성하는 중입니다.";
      try {
        await createOrganizationDummyData(value);
        localStorage.setItem(PROTOTYPE_API_BASE_KEY, value);
        updateCurrentApiBaseUrlDisplays();
        supportStatus.textContent = "조직/인력 더미 데이터 생성이 완료되었습니다.";
        setApiStatus("ok", `더미 데이터 생성 완료. ${value} 기준으로 샘플 데이터가 적재되었습니다.`);
      } catch (error) {
        supportStatus.textContent = `더미 데이터 생성 실패: ${error.message}`;
        setApiStatus("danger", `더미 데이터 생성 실패: ${error.message}`);
      }
    });
  }
}

setupLiveOperations();
