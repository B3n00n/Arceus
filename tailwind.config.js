/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        discord: {
          blurple: '#7289da',
          dark: '#424549',
          'dark-2': '#36393e',
          'dark-3': '#282b30',
          'dark-4': '#1e2124',
        },
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
}
