# Documentation Maintenance Guide

## Adding New Pages

1. Create .md file in appropriate `docs/` folder
2. Add entry to `sidebars.js`
3. Push to master → auto-deploys

## Updating Existing Pages

Edit the .md file directly → auto-deploys on push

## Adding Spanish Translations

Edit corresponding file in `i18n/es/docusaurus-plugin-content-docs/current/`

## Creating a New Version (Release)

When releasing v4.4.0:

```bash
npm run docusaurus docs:version 4.4.0
git add versioned_docs/ versioned_sidebars/
git commit -m "docs: version for v4.4.0"
git tag v4.4.0
git push --tags
```

## Testing Locally

```bash
npm install
npm run start
```

## Building for Production

```bash
npm run build
npm run serve
```

## Troubleshooting

- **Broken links:** Run `npm run build` - shows broken refs
- **Sidebar not updating:** Restart `npm run start`
- **Language switcher missing:** Check i18n config
