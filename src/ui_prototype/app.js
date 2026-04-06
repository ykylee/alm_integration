const navItems = document.querySelectorAll(".nav-item");
const screens = document.querySelectorAll(".screen");
const roleChips = document.querySelectorAll(".role-chip");

const roleContent = {
  developer: {
    name: "개발자",
    summary: "내 리뷰, 빌드, 테스트 보완이 먼저 보이는 작업 중심 홈",
    heroKicker: "Developer Home",
    heroTitle: "내 리뷰, 빌드 이상, 테스트 보완이 가장 먼저 보여야 한다.",
    heroDescription:
      "개발자는 시스템 설정보다 지금 손대야 할 `PR`, 실패 빌드, 누락 테스트를 가장 빠르게 확인해야 한다.",
    metrics: [
      ["내가 볼 PR", "7", "오늘 기준"],
      ["실패 빌드", "2", "조치 필요"],
      ["테스트 보완", "4", "`AI` 초안 가능"],
    ],
    priorities: [
      ["리뷰 대기 `PR-392`", "변경 파일 14개, 코딩룰 위반 의심 2건", "Code Review"],
      ["빌드 재실행 확인", "`build #812` 재실행 결과 비교 필요", "Build"],
      ["단위테스트 보완", "결제 모듈 누락 시나리오 2건", "Test"],
    ],
    actions: [
      ["리뷰 시작", "`AI` 리뷰 초안과 diff를 함께 열기"],
      ["실패 빌드 비교", "최근 성공 build와 환경 차이 확인"],
      ["테스트 초안 생성", "변경 범위 기준 단위테스트 생성 요청"],
    ],
    risks: [
      ["리뷰 지연", "핵심 리뷰어 2명 부재로 승인 지연 가능"],
      ["코딩룰 위반 의심", "null 처리 규칙 1건 재검토 필요"],
      ["테스트 누락", "결제 취소 경로 검증 케이스 없음"],
    ],
    access: [
      ["주요 권한", "과제 수행, 코드리뷰, 단위테스트 보조"],
      ["기본 필터", "내 과제, 내가 리뷰할 `PR`, 실패 빌드"],
      ["제한 영역", "조직 변경 반영, 권한 관리, 연계 재실행"],
    ],
    taskChip: "개발자 집중 탭: Code Review",
    taskLabel: "Developer Focus",
    taskTitle: "Code Review",
    taskCopy: "리뷰 대기 `PR`, 코딩룰 위반 의심, 테스트 부족 범위를 먼저 본다.",
    taskPoints: [
      ["`PR-392`", "변경 14개 파일, 위험 3개 파일 우선 확인"],
      ["코딩룰", "예외 승인 없는 위반 의심 2건"],
      ["테스트", "변경 범위 대비 단위테스트 미존재 1건"],
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
      ["릴리스 일정 조정", "조직장 승인 없이는 다음 주 배포 불가", "Approval"],
      ["품질 게이트 해소", "결함 재검증 2건 미완료", "Quality"],
      ["리뷰 병목 정리", "핵심 PR 1건이 일정에 직접 영향", "Schedule"],
    ],
    actions: [
      ["승인 요청 전달", "조직장과 시스템 관리자에게 병목 전달"],
      ["일정 조정안 작성", "배포 대체 시나리오 생성"],
      ["위험 과제 정렬", "보고용 보드에 우선순위 재배치"],
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
    taskChip: "프로젝트리더 집중 탭: Release Readiness",
    taskLabel: "Project Lead Focus",
    taskTitle: "Release Readiness",
    taskCopy: "릴리스 조건, 승인 병목, 품질 게이트 미충족 항목을 먼저 본다.",
    taskPoints: [
      ["승인 상태", "조직장 승인 1건, 관리자 검토 1건"],
      ["품질 게이트", "정적분석 1건, 테스트 1건 미충족"],
      ["일정 영향", "3일 지연 가능성, 대체안 필요"],
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
      ["관리 대상 전환", "비관리 프로젝트 2건 승인 판단", "Transition"],
      ["승인 루트 갱신", "조직 변경으로 승인선 4건 영향", "Governance"],
      ["리뷰 가용성 감소", "결제플랫폼실 모수 8명 → 6명", "Calendar"],
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
    taskChip: "조직장 집중 탭: People Calendar",
    taskLabel: "Organization Head Focus",
    taskTitle: "People Calendar",
    taskCopy: "승인과 일정에 영향을 주는 조직 부재와 과제 배치를 먼저 본다.",
    taskPoints: [
      ["가용성", "주간 리뷰 모수 감소, 배포 일정 충돌"],
      ["조직 영향", "변경 후 승인 루트 재배치 필요"],
      ["과제 분포", "중요 과제 3건이 동일 조직에 집중"],
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
      ["HR Sync 검토", "조직명 변경과 인력 이동 영향 반영", "Migration"],
      ["`Bamboo` 재실행 정책", "재실행 큐 1건, 예외 승인 필요", "Integration"],
      ["권한 정책 점검", "`AI` 테스트 실행 권한 2건 검토", "Access"],
    ],
    actions: [
      ["영향 분석 실행", "조직 변경으로 영향 받는 과제/계정 확인"],
      ["동기화 재처리", "`Bamboo` 지연 큐 재실행"],
      ["감사 로그 확인", "역할 정책 변경 이력 추적"],
    ],
    risks: [
      ["동기화 지연", "`Bamboo` 응답 지연이 릴리스 판단에 영향"],
      ["계정계 불일치", "`Crowd` 그룹 2건 재계산 필요"],
      ["권한 과부여", "테스트 실행 권한 예외 설정 증가"],
    ],
    access: [
      ["주요 권한", "연계 관리, 조직 변경 반영, 권한/감사 관리"],
      ["기본 필터", "실패 동기화, 변경 검토 대기, 예외 승인"],
      ["제한 영역", "일반 사용자 홈의 일상 작업 흐름에는 기본 진입하지 않음"],
    ],
    taskChip: "시스템 관리자 집중 탭: Governance Snapshot",
    taskLabel: "Admin Focus",
    taskTitle: "Governance Snapshot",
    taskCopy: "과제 자체보다 승인 이력, 연계 상태, 권한 예외를 함께 본다.",
    taskPoints: [
      ["연계 상태", "외부 시스템 동기화와 데이터 최신성 확인"],
      ["권한 정책", "역할별 접근 정책 위반 의심 1건"],
      ["감사 이력", "상태 변경 37건 추적 가능"],
    ],
  },
  se: {
    name: "SE 담당자",
    summary: "환경 차이, `CI` 플랜, 산출물, 릴리스 준비도를 먼저 보는 품질 홈",
    heroKicker: "SE Home",
    heroTitle: "환경 구성, 배포 준비도, `CI` 초안 검토가 먼저 보여야 한다.",
    heroDescription:
      "`SE` 담당자는 개발 자체보다 반영 가능성, 환경 차이, 산출물, `CI` 설정의 일관성을 우선 본다.",
    metrics: [
      ["환경 경고", "2", "운영 반영 영향"],
      ["`CI` 초안", "1", "변수 확인 필요"],
      ["릴리스 체크", "8/10", "완료"],
    ],
    priorities: [
      ["운영 환경 차이", "패키지 의존성 1건 경고", "Environment"],
      ["`Bamboo Specs` 초안", "템플릿 변수 3건 확인 필요", "CI"],
      ["릴리스 체크리스트", "모니터링 태그 누락 1건", "Release"],
    ],
    actions: [
      ["환경 차이 비교", "staging/production 설정 비교"],
      ["`CI` 초안 검토", "템플릿 변수와 예외 규칙 확인"],
      ["산출물 검증", "패키지 서명과 배포 메타데이터 점검"],
    ],
    risks: [
      ["환경 불일치", "운영 환경 라이브러리 버전 차이"],
      ["배포 누락", "모니터링 태그 자동 주입 미확인"],
      ["초안 과신", "`AI` CI 초안 검토 없이 확정 금지"],
    ],
    access: [
      ["주요 권한", "환경 검토, 산출물 확인, 릴리스 준비도 관리"],
      ["기본 필터", "릴리스 후보, 환경 경고, `CI` 초안"],
      ["제한 영역", "조직 승인, 권한 정책, 조직 마스터 변경"],
    ],
    taskChip: "SE 담당자 집중 탭: Environment",
    taskLabel: "SE Focus",
    taskTitle: "Environment",
    taskCopy: "환경 차이, `CI` 초안, 산출물 메타데이터를 먼저 본다.",
    taskPoints: [
      ["환경 차이", "staging/production 변수 2건 차이"],
      ["산출물", "배포 패키지 2개 검토 필요"],
      ["릴리스 준비", "운영 반영 체크리스트 2건 미완료"],
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
      ["차단 결함 확인", "결제 취소 시나리오 재현 필요", "Defect"],
      ["회귀 테스트 완료", "남은 6건 오늘 마감", "Regression"],
      ["단위테스트 보조 검토", "`AI` 초안 4건 중 2건 채택", "AI Test"],
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
    taskChip: "테스트 담당자 집중 탭: Defects",
    taskLabel: "QA Focus",
    taskTitle: "Defects",
    taskCopy: "결함 심각도, 재검증 대기, 릴리스 차단 여부를 먼저 본다.",
    taskPoints: [
      ["차단 결함", "1건, 릴리스 전 필수 해소"],
      ["재검증", "2건 대기, 테스트 환경 준비 완료"],
      ["품질 게이트", "통합테스트 6건 남음"],
    ],
  },
};

function renderCards(target, items, mapper) {
  target.innerHTML = items
    .map((item) => mapper(item))
    .join("");
}

function renderRole(roleKey) {
  const role = roleContent[roleKey] || roleContent.developer;

  document.getElementById("role-name").textContent = role.name;
  document.getElementById("role-summary").textContent = role.summary;
  document.getElementById("hero-kicker").textContent = role.heroKicker;
  document.getElementById("hero-title").textContent = role.heroTitle;
  document.getElementById("hero-description").textContent = role.heroDescription;
  document.getElementById("task-role-chip").textContent = role.taskChip;
  document.getElementById("task-focus-label").textContent = role.taskLabel;
  document.getElementById("task-focus-title").textContent = role.taskTitle;
  document.getElementById("task-focus-copy").textContent = role.taskCopy;

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

  renderCards(document.getElementById("task-focus-points"), role.taskPoints, ([title, copy]) => {
    return `<div><strong>${title}</strong><p>${copy}</p></div>`;
  });

  roleChips.forEach((chip) => {
    chip.classList.toggle("active", chip.dataset.role === roleKey);
  });
}

function activateScreen(screenKey) {
  const nextKey = screenKey || "overview";

  navItems.forEach((item) => {
    item.classList.toggle("active", item.dataset.screen === nextKey);
  });

  screens.forEach((screen) => {
    screen.classList.toggle("active", screen.id === `screen-${nextKey}`);
  });
}

navItems.forEach((item) => {
  item.addEventListener("click", () => {
    const nextKey = item.dataset.screen;
    window.location.hash = nextKey;
    activateScreen(nextKey);
  });
});

roleChips.forEach((chip) => {
  chip.addEventListener("click", () => {
    renderRole(chip.dataset.role);
  });
});

function syncFromHash() {
  const hashKey = window.location.hash.replace("#", "");
  activateScreen(hashKey || "overview");
}

window.addEventListener("hashchange", syncFromHash);
renderRole("developer");
syncFromHash();
