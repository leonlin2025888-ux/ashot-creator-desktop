// Local splash shown while the remote creator.ashot.live UI loads (or if offline).
// The main window navigates to the remote site directly (see tauri.conf.json),
// so this runs only as the first-paint / offline fallback.

window.addEventListener("DOMContentLoaded", () => {
  const status = document.getElementById("status");
  if (status && !navigator.onLine) {
    status.textContent = "当前离线，请检查网络后重试 …";
  }
});
