/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: 'jit',
  content: [
    './storage/framework/views/*.php',
    './resources/views/**/*.blade.php',

    './resources/app/**/*.{css,js,ts,tsx}'
  ],
  darkMode: 'class',
  theme: {
    extend: {}
  },
  plugins: [require('@tailwindcss/typography')]
}
