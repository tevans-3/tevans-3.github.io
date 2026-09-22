(function () {
  var root = document.documentElement;
  var btn = document.getElementById("theme-toggle");
  if (!btn) return;
  var mq = window.matchMedia("(prefers-color-scheme: dark)");

  function current() {
    return root.dataset.theme || (mq.matches ? "dark" : "light");
  }

  function render() {
    var t = current();
    btn.innerHTML = '<span class="bg-key">bg=</span>' + t;
    btn.setAttribute("aria-pressed", String(t === "dark"));
  }

  btn.addEventListener("click", function () {
    var next = current() === "dark" ? "light" : "dark";
    root.dataset.theme = next;
    try { localStorage.setItem("theme", next); } catch (e) {}
    render();
  });

  mq.addEventListener("change", render);
  render();
})();
