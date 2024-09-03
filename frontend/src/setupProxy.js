const { createProxyMiddleware } = require("http-proxy-middleware");

module.exports = function (app) {
  app.use(
    "/media",
    createProxyMiddleware({
      target:
        process.env.REMOTE === "1"
          ? "https://camry.initialed85.cc/media"
          : "http://localhost:6060",
      changeOrigin: true,
    }),
  );

  app.use(
    "/api",
    createProxyMiddleware({
      target:
        process.env.REMOTE === "1"
          ? "https://camry.initialed85.cc/api"
          : "http://localhost:7070/api",
      changeOrigin: true,
    }),
  );
};
