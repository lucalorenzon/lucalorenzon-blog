/** @type {import('tailwindcss').Config} */

const plugin = require("tailwindcss/plugin")

module.exports = {
  mode: 'jit',
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    screens: {
      sm: '480px',
      md: '768px',
      lg: '976px',
      xl: '1440px',
    },
    fontFamily: {
      'sans': ['Futura', 'Arial', 'sans-serif'],
      'serif': ['Bookerly', 'Georgia', 'Cambria'],
      'mono': ['Menlo', 'Courier'],
      'mui-icon': ['"Material Symbols Outlined"']
    },
    extend: {
      zIndex: {
        '2': '2',
        '3': '3',
        '4': '4'
      },
      scale: {
        '200': '2',
        '300': '3'
      },
      spacing: {
        '300npx': '-300px',
        '600npx': '-600px'
      },
      lineHeight: '2rem',
      textShadow: {
        sm: '0 1px 2px var(--tw-shadow-color)',
        DEFAULT: '0 2px 4px var(--tw-shadow-color)',
        lg: '0 8px 16px var(--tw-shadow-color)',
      },
    },
  },
  darkMode: 'class',
  plugins: [
      require('@kamona/tailwindcss-perspective'),
      plugin(function ({ matchUtilities, theme }) {
        matchUtilities(
            {
              'text-shadow': (value) => ({
                textShadow: value,
              }),
            },
            { values: theme('textShadow') }
        )
      }),
  ],
}

