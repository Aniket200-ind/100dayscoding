module.exports = {
    mode: "jit",
    content: {
      files: ["src/**/*.rs", "index.html"],
    },
    darkMode: "media", // 'media' or 'class'
    theme: {
      extend: {
        colors: {
          'raisin-black': '#161925 ',
          'delft-blue': '#23395B',
          'UCLA-blue': '#406E8E',
          'powder-blue': '#8EA8C3', 
          'mint-green': '#CBF7ED',
        }
      },
    },
    variants: {
      extend: {},
    },
    plugins: [],
  };