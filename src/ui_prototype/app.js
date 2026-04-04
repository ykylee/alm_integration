const navItems = document.querySelectorAll(".nav-item");
const screens = document.querySelectorAll(".screen");

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

function syncFromHash() {
  const hashKey = window.location.hash.replace("#", "");
  activateScreen(hashKey || "overview");
}

window.addEventListener("hashchange", syncFromHash);
syncFromHash();
