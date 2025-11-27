/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,css,js,ts,jsx,tsx}",
    "./index.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        jetmono: ['"JetBrainsMono Nerd Font"', 'monospace'],
      },
    },
  },
  plugins: [],
};
