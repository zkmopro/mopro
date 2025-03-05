module.exports = {
  server: {
    baseDir: "./",
    routes: {
      "/test-vectors": "../../test-vectors",
      "/MoproWasmBindings": "../MoproWasmBindings"
    },
    middleware: [
      // To allow COR for only testing and development
      function (req, res, next) {
        res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
        res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
        next();
      }
    ]
  }
};
