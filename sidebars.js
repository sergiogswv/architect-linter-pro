module.exports = {
  tutorialSidebar: [
    'intro',
    {
      type: 'category',
      label: 'Installation',
      items: ['installation/windows', 'installation/linux', 'installation/macos']
    },
    {
      type: 'category',
      label: 'Getting Started',
      items: ['getting-started/basic-usage', 'getting-started/configuration', 'getting-started/first-rules']
    },
    {
      type: 'category',
      label: 'Guides',
      items: ['guides/architecture-design', 'guides/rule-engine', 'guides/ai-features', 'guides/watch-mode', 'guides/daemon-mode']
    },
    {
      type: 'category',
      label: 'Templates',
      items: ['templates/nestjs', 'templates/express', 'templates/react', 'templates/nextjs', 'templates/django']
    },
    {
      type: 'category',
      label: 'API Reference',
      items: ['api-reference/cli-commands', 'api-reference/architect-json', 'api-reference/ai-config']
    },
    {
      type: 'category',
      label: 'Troubleshooting',
      items: ['troubleshooting/config-errors', 'troubleshooting/common-issues']
    }
  ]
};
