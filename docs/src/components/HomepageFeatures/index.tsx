import styles from "./styles.module.css";

type FeatureItem = {
    title: string;
    Svg: React.ComponentType<React.ComponentProps<"svg">>;
    description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
    {
        title: "Cross-platform Support",
        Svg: require("@site/static/img/gold_arch.svg").default,
        description: (
            <>
                Automated pipelines for generating bindings and deploying
                libraries across platforms: iOS (Swift), Android (Kotlin),
                web/JS app, react-native, flutter.
            </>
        ),
    },
    {
        title: "Simple CLI & SDK",
        Svg: require("@site/static/img/red_angle.svg").default,
        description: (
            <>
                Developers can generate bindings and integrate circuits in
                minutes via an intuitive command-line interface and
                mobile-friendly SDK.
            </>
        ),
    },
    {
        title: "Circuit Integration",
        Svg: require("@site/static/img/blue_angle.svg").default,
        description: (
            <>
                Support for Circom, Halo2 and Noir. It is also easy to
                integrate with other proving systems.
            </>
        ),
    },
    {
        title: "Extensibility to new proving systems",
        Svg: require("@site/static/img/gold_leaf.svg").default,
        description: (
            <>
                Currently supporting Arkworks, Rapidsnark, Plonk, Hyperplonk,
                Gemini, Barretenberg with more to come!
            </>
        ),
    },
];

const AdditionalFeatureList = [
    {
        title: "WebAssembly (WASM) Support",
        description: (
            <>
                <a href="https://github.com/zkmopro/mopro/tree/main/mopro-wasm">
                    mopro-wasm
                </a>
                : Generates WebAssembly (WASM) with Rayon for browser
                compatibility.
            </>
        ),
    },
    {
        title: "circom-prover",
        description: (
            <>
                <a href="https://github.com/zkmopro/mopro/tree/main/circom-prover">
                    circom-prover
                </a>
                : A Rust prover for{" "}
                <a href="https://github.com/iden3/circom">Circom</a>. Supports
                cross-platform high performance circom proving. It also supports
                BLS12-381 and BN254 curves.
            </>
        ),
    },
    {
        title: "ark-zkey",
        description: (
            <>
                <a href="https://github.com/zkmopro/ark-zkey">ark-zkey</a>:
                Compresses and decompresses zkey files for Arkworks.
            </>
        ),
    },
    {
        title: "witnesscalc_adapter",
        description: (
            <>
                <a href="https://github.com/zkmopro/witnesscalc_adapter">
                    witnesscalc_adapter
                </a>
                : A Rust wrapper for{" "}
                <a href="https://github.com/0xPolygonID/witnesscalc">
                    witnesscalc
                </a>
                .
            </>
        ),
    },
    {
        title: "rust-rapidsnark",
        description: (
            <>
                <a href="https://github.com/zkmopro/rust-rapidsnark">
                    rust-rapidsnark
                </a>
                : A Rust wrapper for{" "}
                <a href="https://github.com/iden3/rapidsnark">Rapidsnark</a>.
            </>
        ),
    },
    {
        title: "noir-rs",
        description: (
            <>
                <a href="https://github.com/zkmopro/noir-rs">noir-rs</a>: A Rust
                wrapper for <a href="https://github.com/noir-lang/noir">Noir</a>
                .
            </>
        ),
    },
];

function Feature({ title, Svg, description }: FeatureItem) {
    return (
        <div className={styles.feature}>
            <div className={styles.featureSvg}>
                <Svg role="img" />
            </div>
            <div className="">
                <div className={styles.featureHeading}>{title}</div>
                <p className={styles.featureText}>{description}</p>
            </div>
        </div>
    );
}

function AdditionalFeature({
    title,
    description,
}: {
    title: string;
    description: JSX.Element;
}) {
    return (
        <div className={styles.additionalFeature}>
            <div className={styles.additionalFeatureHeading}>{title}</div>
            <p className={styles.additionalFeatureText}>{description}</p>
        </div>
    );
}

export default function HomepageFeatures(): JSX.Element {
    return (
        <section>
            <div className={styles.heading}>Core Features</div>
            <div className={styles.featuresContainer}>
                {FeatureList.map((props, idx) => (
                    <Feature key={idx} {...props} />
                ))}
            </div>

            <div className={styles.additionalHeading}>Additional Features</div>
            <div className={styles.additionalFeaturesContainer}>
                {AdditionalFeatureList.map((props, idx) => (
                    <AdditionalFeature key={idx} {...props} />
                ))}
            </div>
        </section>
    );
}
