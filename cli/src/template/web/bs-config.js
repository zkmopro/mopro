module.exports = {
  server: {
    baseDir: "./",
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
