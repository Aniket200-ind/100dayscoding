const { colors } = require("prompt");

module.exports = {
    mode: "jit",
    content: {
      files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
      extend: {
        colors: {
          "white": "#FFFFFF",
          "black": "#000000",
          "oxford-blue": "#14213D",
          "orange": "#FCA311",
          "platinum": "#E5E5E5"
        }
      },
    },
    variants: {
      extend: {},
    },
    plugins: [],
  };