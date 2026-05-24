/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        ondc: {
          blue: '#0055FF',
          dark: '#001A4D',
          light: '#F0F5FF',
        }
      }
    },
  },
  plugins: [],
}
