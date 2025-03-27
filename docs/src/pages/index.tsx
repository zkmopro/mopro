import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import styles from './index.module.css';

function Homepage() {
  return (
    <section>
      <div className={styles.introContainer}>
        <div className={styles.heading}>Mobile proving made simple</div>
        <p className={ styles.firstP}>MoPro (short for Mobile Prover) is an open-source framework designed to make zero-knowledge (ZK) proofs easily accessible for mobile developers. By providing high-performance Rust bindings for iOS and Android, developer-friendly CLI tools, and cross-platform libraries, MoPro enables seamless integration of privacy-preserving protocols into native mobile applications. With MoPro, you can build apps that leverage ZK proofs for private authentication, anonymous event membership, and more—all while maintaining an efficient and smooth user experience.</p>
        <p className={styles.secondP}>This documentation is your go-to resource for getting started with MoPro. Whether you’re a ZK circuit engineer looking to integrate mobile support, a mobile developer exploring privacy-preserving technologies, or a researcher experimenting with efficient proof generation, you'll find guides, API references, and example projects to help you along the way. Let's build the future of privacy-preserving mobile apps together!</p>
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
