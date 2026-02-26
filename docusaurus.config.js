module.exports = {
  title: 'Architect Linter Pro',
  tagline: 'Multi-language software architecture linter written in Rust',
  baseUrl: '/',
  favicon: 'img/favicon.ico',
  organizationName: 'sergio-linter',
  projectName: 'architect-linter-pro',
  deploymentBranch: 'gh-pages',
  onBrokenLinks: 'throw',
  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'es'],
    localeConfigs: {
      en: { label: 'English' },
      es: { label: 'Español' }
    }
  },
  themeConfig: {
    image: 'img/architect-linter-pro-og.png',
    metadata: [
      {
        name: 'description',
        content: 'Architect Linter Pro - Multi-language software architecture linter'
      }
    ]
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          lastVersion: 'current',
          versions: {
            current: {
              label: 'Next',
              path: 'next'
            }
          }
        },
        blog: { showReadingTime: true },
        theme: { customCss: require.resolve('./src/css/custom.css') }
      }
    ]
  ]
};
