import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Celerix Dev Toolbox",
  description: "High-performance \"Swiss Army Knife\" for your daily development workflow",
  base: '/',
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' }
    ],

    sidebar: [
      {
        text: 'Welcome',
        items: [
          { text: 'Getting Started', link: '/getting-started' },
          { text: 'Privacy', link: '/privacy' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/celerix-dev' }
    ]
  }
})
