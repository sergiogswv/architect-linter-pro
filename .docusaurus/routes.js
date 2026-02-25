import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/__docusaurus/debug',
    component: ComponentCreator('/__docusaurus/debug', '5ff'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/config',
    component: ComponentCreator('/__docusaurus/debug/config', '5ba'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/content',
    component: ComponentCreator('/__docusaurus/debug/content', 'a2b'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/globalData',
    component: ComponentCreator('/__docusaurus/debug/globalData', 'c3c'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/metadata',
    component: ComponentCreator('/__docusaurus/debug/metadata', '156'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/registry',
    component: ComponentCreator('/__docusaurus/debug/registry', '88c'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/routes',
    component: ComponentCreator('/__docusaurus/debug/routes', '000'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', '45c'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', 'dc6'),
        routes: [
          {
            path: '/docs',
            component: ComponentCreator('/docs', '76d'),
            routes: [
              {
                path: '/docs/api-reference/ai-config',
                component: ComponentCreator('/docs/api-reference/ai-config', 'daf'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/api-reference/architect-json',
                component: ComponentCreator('/docs/api-reference/architect-json', '883'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/api-reference/cli-commands',
                component: ComponentCreator('/docs/api-reference/cli-commands', 'ada'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/architecture/design',
                component: ComponentCreator('/docs/architecture/design', '2ae'),
                exact: true
              },
              {
                path: '/docs/architecture/development',
                component: ComponentCreator('/docs/architecture/development', '982'),
                exact: true
              },
              {
                path: '/docs/getting-started/basic-usage',
                component: ComponentCreator('/docs/getting-started/basic-usage', 'd9d'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/configuration',
                component: ComponentCreator('/docs/getting-started/configuration', '468'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/getting-started/first-rules',
                component: ComponentCreator('/docs/getting-started/first-rules', '87b'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/guides/ai-features',
                component: ComponentCreator('/docs/guides/ai-features', '2c4'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/guides/architecture-design',
                component: ComponentCreator('/docs/guides/architecture-design', '8ed'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/guides/daemon-mode',
                component: ComponentCreator('/docs/guides/daemon-mode', '958'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/guides/rule-engine',
                component: ComponentCreator('/docs/guides/rule-engine', '897'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/guides/watch-mode',
                component: ComponentCreator('/docs/guides/watch-mode', 'ac5'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/installation/linux',
                component: ComponentCreator('/docs/installation/linux', 'de4'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/installation/macos',
                component: ComponentCreator('/docs/installation/macos', 'f82'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/installation/windows',
                component: ComponentCreator('/docs/installation/windows', '958'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/intro',
                component: ComponentCreator('/docs/intro', '61d'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/plans/2026-02-25-documentation-unification',
                component: ComponentCreator('/docs/plans/2026-02-25-documentation-unification', '091'),
                exact: true
              },
              {
                path: '/docs/templates/django',
                component: ComponentCreator('/docs/templates/django', 'd47'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/templates/express',
                component: ComponentCreator('/docs/templates/express', '5aa'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/templates/nestjs',
                component: ComponentCreator('/docs/templates/nestjs', '86e'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/templates/nextjs',
                component: ComponentCreator('/docs/templates/nextjs', '31c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/templates/react',
                component: ComponentCreator('/docs/templates/react', '823'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/troubleshooting/common-issues',
                component: ComponentCreator('/docs/troubleshooting/common-issues', '944'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/docs/troubleshooting/config-errors',
                component: ComponentCreator('/docs/troubleshooting/config-errors', 'a3e'),
                exact: true,
                sidebar: "tutorialSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
