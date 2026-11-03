// Search over the page index, and filters for the catalogue table. No dependencies, no network except our own index.
(function () {
  var input = document.getElementById("q");
  var results = document.getElementById("results");
  var index = null;
  function load() {
    if (index) return Promise.resolve(index);
    var url = document.documentElement.getAttribute("data-root") + "search.json";
    return fetch(url).then(function (r) { return r.json(); }).then(function (j) { index = j; return j; }).catch(function () { index = []; return index; });
  }
  function render(list) {
    results.innerHTML = "";
    if (!list.length) { results.hidden = true; return; }
    list.slice(0, 8).forEach(function (p) {
      var li = document.createElement("li"), a = document.createElement("a"), s = document.createElement("small");
      a.href = document.documentElement.getAttribute("data-root") + p.url; a.textContent = p.title; s.textContent = p.excerpt;
      a.appendChild(s); li.appendChild(a); results.appendChild(li);
    });
    results.hidden = false;
  }
  if (input) {
    input.addEventListener("input", function () {
      var q = input.value.trim().toLowerCase();
      if (q.length < 2) { results.hidden = true; return; }
      load().then(function (idx) {
        var words = q.split(/\s+/);
        var hits = idx.map(function (p) {
          var t = p.title.toLowerCase(), b = p.text.toLowerCase(), score = 0;
          for (var i = 0; i < words.length; i++) {
            if (t.indexOf(words[i]) >= 0) score += 5; else if (b.indexOf(words[i]) >= 0) score += 1; else return null;
          }
          return { p: p, score: score };
        }).filter(Boolean).sort(function (a, b) { return b.score - a.score; }).map(function (h) { return h.p; });
        render(hits);
      });
    });
    document.addEventListener("keydown", function (e) {
      if (e.key === "/" && document.activeElement.tagName !== "INPUT") { e.preventDefault(); input.focus(); }
      if (e.key === "Escape") { results.hidden = true; input.blur(); }
    });
    document.addEventListener("click", function (e) { if (!e.target.closest(".search")) results.hidden = true; });
  }

  // Catalogue: filter rows of the status tables by status chip and text.
  var bar = document.getElementById("filters");
  if (bar) {
    var tables = Array.prototype.slice.call(document.querySelectorAll(".doc table")).filter(function (t) { return t.rows[0].cells.length === 6; });
    var heads = Array.prototype.slice.call(document.querySelectorAll(".doc h2")).filter(function (h) { return h.nextElementSibling && h.nextElementSibling.tagName === "TABLE" && h.nextElementSibling.rows[0].cells.length === 6; });
    var active = "all", text = "";
    var counter = document.getElementById("count");
    function apply() {
      var shown = 0;
      tables.forEach(function (t, i) {
        var h = heads[i], rows = 0, label = h ? h.textContent.toLowerCase() : "";
        Array.prototype.slice.call(t.tBodies[0].rows).forEach(function (r) {
          var ok = (active === "all" || label.indexOf(active) === 0) && (!text || r.textContent.toLowerCase().indexOf(text) >= 0);
          r.hidden = !ok; if (ok) rows++;
        });
        shown += rows; t.hidden = rows === 0; if (h) h.hidden = rows === 0;
      });
      if (counter) counter.textContent = shown + " shown";
    }
    bar.querySelectorAll("button").forEach(function (b) {
      b.addEventListener("click", function () {
        active = b.getAttribute("data-s"); bar.querySelectorAll("button").forEach(function (x) { x.setAttribute("aria-pressed", x === b ? "true" : "false"); }); apply();
      });
    });
    var f = document.getElementById("filter-text");
    if (f) f.addEventListener("input", function () { text = f.value.trim().toLowerCase(); apply(); });
    apply();
  }
})();
