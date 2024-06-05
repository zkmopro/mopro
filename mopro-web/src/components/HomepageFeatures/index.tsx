import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Ease of use',
    Svg: require('@site/static/img/gold_arch.svg').default,
    description: (
      <>
        mopro simplifies the complexity of integrating zk-proofs into mobile apps, making it accessible even for developers new to mobile development.
      </>
    ),
  },
  {
    title: 'Performance',
    Svg: require('@site/static/img/red_angle.svg').default,
    description: (
      <>
        By optimizing for mobile GPUs, mopro significantly enhances the speed and capacity for handling large circuits, surpassing traditional browser-based solutions.
      </>
    ),
  },
  {
    title: 'Security',
    Svg: require('@site/static/img/blue_angle.svg').default,
    description: (
      <>
        Users can generate claims securely on their devices, ensuring data privacy and integrity.
      </>
    ),
  },
  {
    title: 'X-platform compatibility',
    Svg: require('@site/static/img/yellow_drop.svg').default,
    description: (
      <>
        Whether you're developing for iOS, Android, Windows, or Mac, mopro supports you with a unified toolkit that works seamlessly across all platforms.
      </>
    ),
  },
  {
    title: 'Scalability',
    Svg: require('@site/static/img/gold_leaf.svg').default,
    description: (
      <>
        Designed to scale with your needs, mopro supports a wide range of proving systems, facilitating the development of versatile, high-performance applications.
      </>
    ),
  },
];

function Feature({ title, Svg, description }: FeatureItem) {
  return (
    <div className={styles.feature}>
      <Svg className={styles.featureSvg} role="img" />
      <div className=''>
        <div className={styles.featureHeading}>{title}</div>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <div>
      <div className={styles.featuresContainer}>
        {FeatureList.map((props, idx) => (
          <Feature key={idx} {...props} />
        ))}
      </div>

      <div className={styles.separator}>
        <img src='img/separator.svg' alt='separating line'/>
      </div>
    </div>
  );
}
