import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import Layout from "@theme/Layout";
import HomepageFeatures from "@site/src/components/HomepageFeatures";
import Comparison from "@site/src/components/Comparison";
import Impact from "@site/src/components/Impact";
import styles from "./index.module.css";
import Challenges from "@site/src/components/Challenges";
import Quote from "@site/src/components/Quote";
import IsMoproForMe from "@site/src/components/IsMoproForMe";
import { useState } from "react";

function Homepage() {
    const [copied, setCopied] = useState(false);

    const handleCopy = () => {
        navigator.clipboard.writeText("cargo install mopro-cli");
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
    };

    return (
        <section>
            <div className={styles.introContainer}>
                <div className={styles.whyMoproContainer}>
                    <h2 className={styles.whyMoproHeading}>
                        Mopro is a toolkit for ZK app development on mobile.
                    </h2>
                    <div className={styles.whyMoproContent}>
                        <h3 className={styles.whyMoproSubheading}>
                            🔧 Why Mopro?
                        </h3>
                        <p className={styles.whyMoproText}>
                            MoPro makes it radically easier to integrate
                            zero-knowledge proofs (ZKPs) for client-side
                            proving, providing clean abstractions across
                            multiple platforms.
                        </p>
                        <div className={styles.cliSection}>
                            <div className={styles.cliCodeContainer}>
                                <pre className={styles.cliCode}>
                                    <code>cargo install mopro-cli</code>
                                </pre>
                                <button 
                                    className={styles.copyButton} 
                                    onClick={handleCopy}
                                    title="Copy to clipboard"
                                >
                                    {copied ? (
                                        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                            <path d="M13.3333 4L6 11.3333L2.66667 8" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                                        </svg>
                                    ) : (
                                        <svg width="16" height="16" viewBox="0 0 16 16" fill="none" xmlns="http://www.w3.org/2000/svg">
                                            <path d="M13.3333 6H7.33333C6.59695 6 6 6.59695 6 7.33333V13.3333C6 14.0697 6.59695 14.6667 7.33333 14.6667H13.3333C14.0697 14.6667 14.6667 14.0697 14.6667 13.3333V7.33333C14.6667 6.59695 14.0697 6 13.3333 6Z" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
                                            <path d="M3.33333 10H2.66667C2.31305 10 1.97391 9.85953 1.72386 9.60948C1.47381 9.35943 1.33334 9.02029 1.33334 8.66667V2.66667C1.33334 2.31305 1.47381 1.97391 1.72386 1.72386C1.97391 1.47381 2.31305 1.33334 2.66667 1.33334H8.66667C9.02029 1.33334 9.35943 1.47381 9.60948 1.72386C9.85953 1.97391 10 2.31305 10 2.66667V3.33334" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
                                        </svg>
                                    )}
                                </button>
                            </div>
                            <a href="/docs/getting-started" className={styles.getStartedButton}>
                                Getting Started
                            </a>
                        </div>
                    </div>
                </div>
                <img
                    src="img/mobile.svg"
                    alt="abstract mobile illustration"
                    className={styles.phoneImg}
                />
            </div>
        </section>
    );
}

export default function Home(): JSX.Element {
    const { siteConfig } = useDocusaurusContext();
    return (
        <Layout
            title={`Hello from ${siteConfig.title}`}
            description="Mobile proving made simple."
        >
            <main className={styles.mainContainer}>
                <Homepage />
                <div className={styles.separator}>
                    <img src="img/separator.svg" alt="separating line" />
                </div>
                <IsMoproForMe />
                <div className={styles.separator}>
                    <img src="img/separator.svg" alt="separating line" />
                </div>
                <HomepageFeatures />
                <div className={styles.separator}>
                    <img src="img/separator.svg" alt="separating line" />
                </div>
                <Comparison />
                <div className={styles.separator}>
                    <img src="img/separator.svg" alt="separating line" />
                </div>
                <Impact />
                <div className={styles.separator}>
                    <img src="img/separator.svg" alt="separating line" />
                </div>
                <Challenges />
                <div className={styles.separator}>
                    <img src="img/separator.svg" alt="separating line" />
                </div>
                <Quote />
            </main>
        </Layout>
    );
}
