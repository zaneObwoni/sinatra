/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{rs,html,css}",
    "./index.html",
  ],
  theme: {
    extend: {
    },
  },
  variants: {},
  plugins: [
    require('tailwindcss-question-mark'),
  ],
}
