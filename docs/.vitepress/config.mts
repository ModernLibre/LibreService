import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Modern Libre - Service",
  description: "Libre service for Modern Libre project.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    // nav: [
    //   { text: 'Home', link: '/' },

    //   { text: 'Api', link: '/auth_api' }
    // ],

    sidebar: [
      {
        text: 'Api',
        items: [
          { text: 'Authentication', link: '/auth_api' },
          { text: 'User', link: '/user_api' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/ModernLibre/LibreService' }
    ]
  },
  base: '/LibreService/',
  rewrites: {
    'auth_api.md': 'index.md',
    // 'auth_api.md': 'auth_api.md'
  }
})
