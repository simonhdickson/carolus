self.addEventListener("fetch", (e) => {
    e.respondWith(
        caches.match(e.request).then(resp => resp || fetch(e.request)),
    );
});

self.addEventListener("install", (e) => {
    e.waitUntil(
        caches.open("carolus-cache").then(cache => cache.addAll([
            "/",
            "/static/manifest.json",
            "/static/css/style.css",
            "/static/img/arrow-back.svg",
            "/static/img/arrow-forward.svg",
            "/static/img/carolus.svg",
            "/static/img/book.svg",
            "/static/img/info.svg",
            "/static/img/unfold-more.svg",
            "/static/js/autocomplete.min.js",
            "/static/js/main.js",
        ])),
    );
});
