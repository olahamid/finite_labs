// Light/dark toggle: the only hand-written JavaScript on the site (PRD §6.0).
// Without JS the site still follows the system theme via prefers-color-scheme.
(function () {
  var root = document.documentElement;

  var saved = null;
  try { saved = localStorage.getItem("theme"); } catch (e) { /* storage blocked */ }
  if (saved === "light" || saved === "dark") root.dataset.theme = saved;

  function currentTheme() {
    if (root.dataset.theme) return root.dataset.theme;
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }

  document.addEventListener("DOMContentLoaded", function () {
    var button = document.querySelector(".theme-toggle");
    if (!button) return;
    button.hidden = false;
    button.setAttribute("aria-pressed", String(currentTheme() === "dark"));

    button.addEventListener("click", function () {
      var next = currentTheme() === "dark" ? "light" : "dark";
      root.dataset.theme = next;
      button.setAttribute("aria-pressed", String(next === "dark"));
      try { localStorage.setItem("theme", next); } catch (e) { /* storage blocked */ }
    });
  });
})();
