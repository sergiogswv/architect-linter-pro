import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  emoji: string;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Blazing Fast',
    emoji: '🦀',
    description: (
      <>
        Built with Rust and Tree-sitter for lightning-fast analysis. Parallel processing via Rayon
        means scanning thousands of files in seconds, not minutes.
      </>
    ),
  },
  {
    title: 'Multi-Language',
    emoji: '🔍',
    description: (
      <>
        Analyze TypeScript, JavaScript, Python, and PHP projects. Extensible architecture supports
        adding more languages with grammar-based parsing.
      </>
    ),
  },
  {
    title: 'AI Auto-Fix',
    emoji: '🤖',
    description: (
      <>
        Intelligent violation suggestions powered by Claude, Gemini, OpenAI, Groq, or Ollama.
        Get actionable recommendations to improve your architecture.
      </>
    ),
  },
  {
    title: 'Health Score',
    emoji: '📊',
    description: (
      <>
        Get a 0-100 health score with A-F grading system. Track architecture quality over time and
        set improvement goals.
      </>
    ),
  },
  {
    title: 'Watch Mode',
    emoji: '👁️',
    description: (
      <>
        Continuous analysis as you code with 300ms debounce. Get instant feedback on architectural
        violations while you develop.
      </>
    ),
  },
  {
    title: 'Dynamic Rules',
    emoji: '🏗️',
    description: (
      <>
        Built-in patterns for Clean Architecture, MVC, Hexagonal, and NestJS. Define custom rules
        to match your project's architecture.
      </>
    ),
  },
];

function Feature({title, emoji, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <div className={styles.featureEmoji}>{emoji}</div>
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <Heading as="h2" className={styles.featuresTitle}>
          Everything You Need
        </Heading>
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
