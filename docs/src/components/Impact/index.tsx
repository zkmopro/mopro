import React from 'react';
import styles from './styles.module.css';

const Impact: React.FC = () => {
  return (
    <div className={styles.impactSection}>
      <h2 className={styles.title}>📈 Impact & Value</h2>

      <div className={styles.section}>
        <h3>Short-Term</h3>
        <ul className={styles.list}>
          <li className={styles.listItem}>
            <strong>Reduction in the size of native library bindings</strong>
            <br />
            In this <a href="https://github.com/orgs/zkmopro/projects/1/views/1?pane=issue&itemId=105992137&issue=zkmopro%7Cmopro%7C395">example for zkEmail</a>, Mopro reduced the iOS binary size from ~154 MB to 143 MB ~7.1% improvement
          </li>
          <li className={styles.listItem}>
            <strong>⏱ Time-to-Integration</strong>
            <br />
            Most developers can generate a working proof on a mobile device in under 1.5 <strong>hours</strong>.
          </li>
          <li className={styles.listItem}>
            <strong>📱 Real-World Adoption</strong>
            <br />
            3+ mobile apps in production using MoPro expected by <strong>Q3 2025</strong>. (Anon Aadhaar, zkEmail, EZKL, Semaphore)
          </li>
          <li className={styles.listItem}>
            <strong>💡 Dev Effort Saved</strong>
            <br />
            Integrators report saving ~14 days <strong>in dev time/code</strong> vs. traditional approaches.
          </li>
        </ul>
      </div>

      <div className={styles.section}>
        <h3>Long-Term</h3>
        <ul className={styles.list}>
          <li className={styles.listItem}>
            <strong>🌐 GPU acceleration</strong>
            <br />
            Leverage client-side native <strong>GPUs</strong> and other <strong>proving systems</strong> to enhance on-device proof generation, <a href="https://github.com/zkmopro/gpu-acceleration">read our research here</a>.
          </li>
          <li className={styles.listItem}>
            <strong>🧠 Future Proof System Support</strong>
            <br />
            MoPro will continue to add support for the most performant client-side proving system based on PSE's client-side proving <a href="https://hackmd.io/@clientsideproving/zkIDBenchmarks">research</a> team
          </li>
          <li className={styles.listItem}>
            <strong>🐞 Debugging and testing tools</strong>
          </li>
        </ul>
      </div>
    </div>
  );
};

export default Impact;
