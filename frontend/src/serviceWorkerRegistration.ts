// Camry does not register a service worker any more; see frontend/src/service-worker.ts for the
// reason (a precached shell hides every navigation from the sign-in gate).
//
// retire() is the migration. Deleting the worker from the build would leave browsers holding the old
// one until Chrome happened to update-check it, and unregistering from the page is not reliable on its
// own because the page may itself be the copy the old worker served. So both sides do it: the stub
// worker unregisters itself, and any Camry bundle loaded from the network sweeps up registrations and
// caches it left behind.
export const retire = async (): Promise<void> => {
  if (!("serviceWorker" in navigator)) {
    return;
  }

  try {
    const registrations = await navigator.serviceWorker.getRegistrations();
    await Promise.all(registrations.map((registration) => registration.unregister()));

    if (typeof caches !== "undefined") {
      const names = await caches.keys();
      await Promise.all(names.map((name) => caches.delete(name)));
    }
  } catch (error) {
    // Nothing here is load-bearing for the app; a browser that refuses the sweep will still get the
    // stub worker's own retirement on its next navigation.
    console.warn("camry: could not retire service worker registrations", error);
  }
};
