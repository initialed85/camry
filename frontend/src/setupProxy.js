const { createProxyMiddleware } = require("http-proxy-middleware");

module.exports = function (app) {
  app.use(
    "/media",
    createProxyMiddleware({
      target: "http://localhost:6060",
      // target: "https://camry.initialed85.cc/media",
      changeOrigin: true,
    }),
  );

  app.use(
    "/api",
    createProxyMiddleware({
      target: "http://localhost:7070/api",
      // target: "https://camry.initialed85.cc/api",
      changeOrigin: true,
    }),
  );
};
