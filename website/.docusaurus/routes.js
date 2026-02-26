import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/architect-linter-pro/es/blog',
    component: ComponentCreator('/architect-linter-pro/es/blog', 'f40'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/archive',
    component: ComponentCreator('/architect-linter-pro/es/blog/archive', '803'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/authors',
    component: ComponentCreator('/architect-linter-pro/es/blog/authors', 'cb1'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/authors/all-sebastien-lorber-articles',
    component: ComponentCreator('/architect-linter-pro/es/blog/authors/all-sebastien-lorber-articles', '537'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/authors/yangshun',
    component: ComponentCreator('/architect-linter-pro/es/blog/authors/yangshun', '53d'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/first-blog-post',
    component: ComponentCreator('/architect-linter-pro/es/blog/first-blog-post', '9bc'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/long-blog-post',
    component: ComponentCreator('/architect-linter-pro/es/blog/long-blog-post', '031'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/mdx-blog-post',
    component: ComponentCreator('/architect-linter-pro/es/blog/mdx-blog-post', '021'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/tags',
    component: ComponentCreator('/architect-linter-pro/es/blog/tags', 'e14'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/tags/docusaurus',
    component: ComponentCreator('/architect-linter-pro/es/blog/tags/docusaurus', 'e3f'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/tags/facebook',
    component: ComponentCreator('/architect-linter-pro/es/blog/tags/facebook', 'a39'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/tags/hello',
    component: ComponentCreator('/architect-linter-pro/es/blog/tags/hello', '04d'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/tags/hola',
    component: ComponentCreator('/architect-linter-pro/es/blog/tags/hola', 'e3e'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/blog/welcome',
    component: ComponentCreator('/architect-linter-pro/es/blog/welcome', '68d'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/markdown-page',
    component: ComponentCreator('/architect-linter-pro/es/markdown-page', '3b0'),
    exact: true
  },
  {
    path: '/architect-linter-pro/es/docs',
    component: ComponentCreator('/architect-linter-pro/es/docs', 'b33'),
    routes: [
      {
        path: '/architect-linter-pro/es/docs',
        component: ComponentCreator('/architect-linter-pro/es/docs', '5d8'),
        routes: [
          {
            path: '/architect-linter-pro/es/docs',
            component: ComponentCreator('/architect-linter-pro/es/docs', 'd56'),
            routes: [
              {
                path: '/architect-linter-pro/es/docs/category/api--reference',
                component: ComponentCreator('/architect-linter-pro/es/docs/category/api--reference', '6b6'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/category/framework-guides',
                component: ComponentCreator('/architect-linter-pro/es/docs/category/framework-guides', 'd67'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/category/technical-guides',
                component: ComponentCreator('/architect-linter-pro/es/docs/category/technical-guides', 'c8b'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/changelog',
                component: ComponentCreator('/architect-linter-pro/es/docs/changelog', '388'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/guides/frontend',
                component: ComponentCreator('/architect-linter-pro/es/docs/guides/frontend', '50a'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/guides/go',
                component: ComponentCreator('/architect-linter-pro/es/docs/guides/go', 'e21'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/guides/java',
                component: ComponentCreator('/architect-linter-pro/es/docs/guides/java', 'b95'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/guides/laravel',
                component: ComponentCreator('/architect-linter-pro/es/docs/guides/laravel', 'f54'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/guides/nestjs',
                component: ComponentCreator('/architect-linter-pro/es/docs/guides/nestjs', '89c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/guides/python',
                component: ComponentCreator('/architect-linter-pro/es/docs/guides/python', '8dc'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/intro',
                component: ComponentCreator('/architect-linter-pro/es/docs/intro', '46a'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/reference/api',
                component: ComponentCreator('/architect-linter-pro/es/docs/reference/api', 'b9b'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/roadmap',
                component: ComponentCreator('/architect-linter-pro/es/docs/roadmap', 'dc2'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/technical/config-validation',
                component: ComponentCreator('/architect-linter-pro/es/docs/technical/config-validation', 'b3c'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/technical/error-handling',
                component: ComponentCreator('/architect-linter-pro/es/docs/technical/error-handling', '7c2'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/technical/performance',
                component: ComponentCreator('/architect-linter-pro/es/docs/technical/performance', '9bb'),
                exact: true,
                sidebar: "tutorialSidebar"
              },
              {
                path: '/architect-linter-pro/es/docs/technical/testing-guide',
                component: ComponentCreator('/architect-linter-pro/es/docs/technical/testing-guide', 'c83'),
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
    path: '/architect-linter-pro/es/',
    component: ComponentCreator('/architect-linter-pro/es/', '20c'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
