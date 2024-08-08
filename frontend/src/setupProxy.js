const { createProxyMiddleware } = require("http-proxy-middleware");

module.exports = function (app) {
  app.use(
    "/media",
    createProxyMiddleware({
      target: "http://localhost:6060",
      changeOrigin: true,
    }),
  );

  // app.use(
  //     '/api',
  //     createProxyMiddleware({
  //         target: 'http://localhost:7070',
  //         changeOrigin: true,
  //     })
  // );
};
