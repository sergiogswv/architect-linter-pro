const { execSync } = require('child_process');
const fs = require('fs');

try {
  const tags = execSync('git tag --sort=-version:refname | head -20')
    .toString()
    .split('\n')
    .filter(Boolean);

  let changelog = '# Changelog\n\n';

  for (let i = 0; i < tags.length - 1; i++) {
    const currentTag = tags[i];
    const previousTag = tags[i + 1];
    const commits = execSync(`git log ${previousTag}..${currentTag} --oneline`)
      .toString()
      .trim();

    if (commits) {
      changelog += `## [${currentTag}]\n\n${commits}\n\n`;
    }
  }

  fs.writeFileSync('docs/changelog.md',
    '---\ntitle: Changelog\n---\n\n' + changelog);
  console.log('Changelog generated');
} catch (error) {
  console.error('Changelog generation failed:', error.message);
  process.exit(1);
}
