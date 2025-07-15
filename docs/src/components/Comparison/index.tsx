import React from 'react';
import styles from './styles.module.css';

const comparisonData = [
  {
    feature: 'Integration Time',
    mopro: 'Under 2 hours',
    legacy: '10–15 days',
  },
  {
    feature: 'Native Code Required',
    mopro: 'Minimal (mostly abstracted)',
    legacy: 'High',
  },
  {
    feature: 'Code Complexity',
    mopro: 'Single toolchain (Rust → Kotlin/Swift)',
    legacy: 'Multiple binaries, manual bindings',
  },
  {
    feature: 'Dev UX',
    mopro: 'Clean CLI, modern tooling',
    legacy: 'Fragmented and error-prone',
  },
  {
    feature: 'Cross-Platform',
    mopro: 'Yes (Android & iOS)',
    legacy: 'Requires separate native work',
  },
  {
    feature: 'Proof System Upgradability',
    mopro: 'Minimal effort 2-3 hours',
    legacy: 'Requires entire code re-write',
  },
];

export default function Comparison(): JSX.Element {
  return (
    <section className={styles.comparisonSection}>
      <h2 className={styles.heading}>MoPro vs. Legacy Setups</h2>
      <div className={styles.tableContainer}>
        <table className={styles.comparisonTable}>
          <thead>
            <tr>
              <th>Comparison</th>
              <th>MoPro</th>
              <th>Legacy</th>
            </tr>
          </thead>
          <tbody>
            {comparisonData.map((row, idx) => (
              <tr key={idx}>
                <td>{row.feature}</td>
                <td className={styles.moproCell}>{row.mopro}</td>
                <td className={styles.legacyCell}>{row.legacy}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </section>
  );
}
