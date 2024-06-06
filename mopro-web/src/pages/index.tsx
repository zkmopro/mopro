import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import styles from './index.module.css';

function Homepage() {
  return (
    <section>
      <div className={styles.introContainer}>
        <div className={styles.heading}>Mobile proving made simple</div>
        <p className={ styles.firstP}>With mopro, developers can overcome the limitations of existing tools like snarkjs, which struggles with small circuit sizes and slow performance in browsers. Our solution leverages the growing power of mobile GPUs to deliver a fast, scalable, and secure proving experience directly on client-side applications</p>
        <p className={styles.secondP}>mopro, short for Mobile Prover, redefines the landscape of mobile app development by introducing an innovative toolkit designed for zero-knowledge (zk) proofs. With mopro, developers can overcome the limitations of existing tools like snarkjs, which struggles with small circuit sizes and slow performance in browsers. Our solution leverages the growing power of mobile GPUs to deliver a fast, scalable, and secure proving experience directly on client-side applications.</p>
        <img src='img/mobile.svg' alt='abstract mobile illustration' className={styles.phoneImg}/>
      </div>
    </section>
  );
}

export default function Home(): JSX.Element {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`Hello from ${siteConfig.title}`}
      description="Mobile proving made simple.">    
      <main className={styles.mainContainer}>
        <Homepage />
        <div className={styles.separator}>
          <img src='img/separator.svg' alt='separating line'/>
        </div>
        <HomepageFeatures />
        <div className={styles.separator}>
          <img src='img/separator.svg' alt='separating line'/>
        </div>
      </main>
    </Layout>
  );
}
