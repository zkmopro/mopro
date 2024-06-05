// import clsx from 'clsx';
// import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
// import Heading from '@theme/Heading';

import styles from './index.module.css';

function Homepage() {
  return (
    <main className={styles.mainContainer}>
      <section>
        <div className={styles.introContainer}>
          <div className={styles.heading}>Mobile proving made simple</div>
          <p className={styles.introParagraph}>With mopro, developers can overcome the limitations of existing tools like snarkjs, which struggles with small circuit sizes and slow performance in browsers. Our solution leverages the growing power of mobile GPUs to deliver a fast, scalable, and secure proving experience directly on client-side applications</p>
        </div>
        <div className={styles.introContainer}>
          <p className={styles.introParagraph}>mopro, short for Mobile Prover, redefines the landscape of mobile app development by introducing an innovative toolkit designed for zero-knowledge (zk) proofs. With mopro, developers can overcome the limitations of existing tools like snarkjs, which struggles with small circuit sizes and slow performance in browsers. Our solution leverages the growing power of mobile GPUs to deliver a fast, scalable, and secure proving experience directly on client-side applications.</p>
          <img src='img/mobile.svg' alt='abstract mobile illustration'/>
        </div>
      </section>

      <div className={styles.separator}>
        <img src='img/separator.svg' alt='separating line'/>
      </div>

      <section className={styles.featuesContainer}>
        <div className={styles.heading}>Developer Capabilities</div>
        <div>

        </div>
      </section>
    </main>
  );
}

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`Hello from ${siteConfig.title}`}
      description="Mobile proving made simple.">
      <Homepage />
      <main>
        <HomepageFeatures />
      </main>
    </Layout>
  );
}
