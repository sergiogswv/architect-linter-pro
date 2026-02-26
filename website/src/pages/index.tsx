import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <div className={styles.heroContent}>
          <div className={styles.badgeContainer}>
            <span className={styles.badge}>v5.0.2</span>
          </div>

          <Heading as="h1" className={styles.heroTitle}>
            {siteConfig.title}
          </Heading>

          <p className={styles.heroSubtitle}>
            {siteConfig.tagline}
          </p>

          <div className={styles.buttons}>
            <Link
              className="button button--secondary button--lg"
              to="/docs/intro">
              Get Started →
            </Link>
            <a
              className="button button--outline button--secondary button--lg"
              href="https://github.com/sergiogswv/architect-linter-pro"
              target="_blank"
              rel="noopener noreferrer">
              GitHub
            </a>
          </div>

          <div className={styles.terminalDemo}>
            <div className={styles.terminal}>
              <div className={styles.terminalHeader}>
                <span className={styles.terminalDot} style={{backgroundColor: '#ff5f56'}}></span>
                <span className={styles.terminalDot} style={{backgroundColor: '#ffbd2e'}}></span>
                <span className={styles.terminalDot} style={{backgroundColor: '#27c93f'}}></span>
              </div>
              <div className={styles.terminalBody}>
                <div>$ architect-linter analyze .</div>
                <div className={styles.outputSuccess}>✓ Scanning 234 files...</div>
                <div>
                  <span className={styles.outputLabel}>Architecture Health Score:</span>
                  <span className={styles.outputHighlight}> 92/100 (A)</span>
                </div>
                <div>
                  <span className={styles.outputLabel}>Violations Found:</span>
                  <span className={styles.outputHighlight}> 3 critical, 5 warnings</span>
                </div>
                <div className={styles.outputSuccess}>✓ Analysis complete in 245ms</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </header>
  );
}

function InstallSection() {
  return (
    <section className={styles.installSection}>
      <div className="container">
        <Heading as="h2" className={styles.sectionTitle}>
          Get Started in Seconds
        </Heading>

        <div className={styles.installCode}>
          <pre>
            <code>cargo install architect-linter-pro</code>
          </pre>
          <button
            className={styles.copyButton}
            onClick={() => {
              navigator.clipboard.writeText('cargo install architect-linter-pro');
            }}>
            Copy
          </button>
        </div>

        <p className={styles.installNote}>
          Or use npm, Homebrew, or Scoop. Check <Link to="/docs/installation">installation docs</Link> for details.
        </p>
      </div>
    </section>
  );
}

function BenchmarkSection() {
  const benchmarks = [
    { name: 'Small Project (10 files)', time: '23ms', score: 'A' },
    { name: 'Medium Project (100 files)', time: '87ms', score: 'B+' },
    { name: 'Large Project (1000 files)', time: '342ms', score: 'A-' },
    { name: 'Enterprise (10K+ files)', time: '2.1s', score: 'B+' },
  ];

  return (
    <section className={styles.benchmarkSection}>
      <div className="container">
        <Heading as="h2" className={styles.sectionTitle}>
          Lightning-Fast Performance
        </Heading>

        <div className={styles.benchmarkTable}>
          <table>
            <thead>
              <tr>
                <th>Project Size</th>
                <th>Analysis Time</th>
                <th>Health Score</th>
              </tr>
            </thead>
            <tbody>
              {benchmarks.map((item, idx) => (
                <tr key={idx}>
                  <td>{item.name}</td>
                  <td className={styles.benchmarkTime}>{item.time}</td>
                  <td className={styles.benchmarkScore}>{item.score}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        <p className={styles.benchmarkNote}>
          Built with Rust, Tree-sitter, and Rayon parallel processing for maximum speed.
        </p>
      </div>
    </section>
  );
}

function CtaSection() {
  return (
    <section className={styles.ctaSection}>
      <div className="container">
        <div className={styles.ctaContent}>
          <Heading as="h2">Ready to Improve Your Architecture?</Heading>
          <p>Start linting your architecture today and get actionable insights.</p>
          <Link
            className="button button--secondary button--lg"
            to="/docs/intro">
            Start Now →
          </Link>
        </div>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={siteConfig.title}
      description="Enterprise-grade architecture linting for modern developers">
      <HomepageHeader />
      <main>
        <InstallSection />
        <HomepageFeatures />
        <BenchmarkSection />
        <CtaSection />
      </main>
    </Layout>
  );
}
