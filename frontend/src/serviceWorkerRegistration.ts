// Camry does not register a service worker any more, and no worker is built into the image: a
// precached shell hides every navigation from the sign-in gate (see README's Sign in section).
//
// nginx answers /service-worker.js as a 404, which is the kill switch - a service worker update
// check that fails 4xx makes the browser unregister whatever worker it holds, with no new JS having
// to run. retire() is the belt-and-braces half for the browsers that do load Camry from the network:
// it unregisters whatever registration is still hanging around and deletes the CacheStorage the old
// precaching worker wrote, so a stale copy of the app cannot keep answering requests forever.
export const retire = async (): Promise<void> => {
  if (!("serviceWorker" in navigator)) {
    return;
  }

  try {
    const registrations = await navigator.serviceWorker.getRegistrations();
    await Promise.all(
      registrations.map((registration) => registration.unregister()),
    );

    if (typeof caches !== "undefined") {
      const names = await caches.keys();
      await Promise.all(names.map((name) => caches.delete(name)));
    }
  } catch (error) {
    // Nothing here is load-bearing for the app; the 404 on /service-worker.js already retires the
    // worker on a browser that refuses this sweep.
    console.warn("camry: could not retire service worker registrations", error);
  }
};
