/// <reference lib="webworker" />
/* eslint-disable no-restricted-globals */

// This service worker is retired, and this file is what replaces it in browsers that still have the
// old one installed.
//
// The version that used to live here was Create React App's Workbox example: it precached every build
// asset and answered *all* navigation requests with the precached index.html. That made Camry look
// broken the moment sign-in moved to the cluster gate. A precached navigation never reaches Traefik,
// so the gate never gets a chance to redirect to Google; the SPA then loads from cache, its /api and
// /media requests do hit the network, get 302'd to Google, and the app is left staring at a sign-in
// card that a reload cannot fix - the reload is answered from cache again. Offline caching is not a
// thing a LAN camera viewer needs; the gate seeing every navigation is.
//
// So: install quietly, delete the caches the old worker left behind, unregister, and register no fetch
// handler at all so requests go to the network. Chrome update-checks /service-worker.js on navigation,
// which is what lets a browser that is stuck on the old worker climb out of it.

declare const self: ServiceWorkerGlobalScope;

// An export keeps this file a module. Without one, TypeScript reads the `self` declaration above as a
// redeclaration of the global `self` that lib.dom already declares, and CRA's build fails.
export {};

// CRA's Workbox build step looks for this placeholder and refuses to build the worker without it.
// The assets it injects are deliberately left uncached.
const manifest: unknown[] = (self as any).__WB_MANIFEST ?? [];

self.addEventListener("install", () => {
  console.info(
    `camry service worker retiring: ${manifest.length} build assets left uncached`,
  );
  self.skipWaiting();
});

self.addEventListener("activate", (event: ExtendableEvent) => {
  event.waitUntil(
    caches
      .keys()
      .then((names) => Promise.all(names.map((name) => caches.delete(name))))
      .then(() => self.registration.unregister()),
  );
});
