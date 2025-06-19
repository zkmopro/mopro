import React from 'react';
import styles from './styles.module.css';

const Quote: React.FC = () => {
  return (
    <div className={styles.quotesSection}>
      <h2 className={styles.quotesHeading}>🚀 What Developers Are Saying</h2>
      <div className={styles.quotesContainer}>
        <div className={styles.quoteContainer}>
          <blockquote className={styles.quoteText}>
            "It used to take 15 days to integrate ZK proving on Android. With MoPro, I had it running in hours — without touching C++."
          </blockquote>
          <p className={styles.quoteAuthor}>—Anon Aadhaar, April 2025</p>
        </div>

        <div className={styles.quoteContainer}>
          <blockquote className={styles.quoteText}>
            "We went from a cpp/ folder full of brittle native code to a clean, smooth setup. MoPro automated most of it."
          </blockquote>
          <p className={styles.quoteAuthor}>—Anonymous Developer</p>
        </div>
      </div>
    </div>
  );
};

export default Quote;
