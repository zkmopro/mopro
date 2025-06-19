import React, { useRef, useLayoutEffect, useState, useMemo } from "react";
import styles from "./IsMoproForMe.module.css";

const nodes = [
    {
        id: "start",
        label: "START\nWhat's your current situation?",
        type: "start",
    },
    {
        id: "existing",
        label: "Do you have an existing ZK project?",
        type: "decision",
    },
    { id: "whatBuilding", label: "What are you building?", type: "decision" },
    { id: "whatHave", label: "What do you currently have?", type: "decision" },
    { id: "zkApp", label: "ZK App", type: "option" },
    { id: "mobileApp", label: "Mobile App", type: "option" },
    { id: "zkProtocol", label: "ZK Protocol", type: "option" },
    { id: "webJSApp", label: "Web/JS App", type: "option" },
    {
        id: "upgradeProving",
        label: "I have a mobile native ZK App \nand I want to upgrade the underlying proving system",
        type: "option",
    },
];

const edges = [
    { from: "start", to: "existing" },
    { from: "existing", to: "whatBuilding", label: "No" },
    { from: "existing", to: "whatHave", label: "Yes" },
    { from: "whatBuilding", to: "zkApp" },
    { from: "whatBuilding", to: "mobileApp" },
    { from: "whatHave", to: "zkProtocol" },
    { from: "whatHave", to: "webJSApp" },
];

type NodeType = {
    id: string;
    label: string;
    type: string;
    desc?: string;
    tags?: string[];
};

const Node: React.FC<{
    node: NodeType;
    nodeRef?: React.RefObject<HTMLDivElement>;
}> = ({ node, nodeRef }) => {
    const [hovered, setHovered] = useState(false);
    const isZkApp = node.id === "zkApp";
    const isMobileApp = node.id === "mobileApp";
    const isZkProtocol = node.id === "zkProtocol";
    const isWebJSApp = node.id === "webJSApp";
    const isUpgradeProving = node.id === "upgradeProving";
    return (
        <div
            ref={nodeRef}
            className={`${styles.graphNode} ${styles[node.type]}`}
            onMouseEnter={() =>
                (isZkApp ||
                    isMobileApp ||
                    isZkProtocol ||
                    isWebJSApp ||
                    isUpgradeProving) &&
                setHovered(true)
            }
            onMouseLeave={() =>
                (isZkApp ||
                    isMobileApp ||
                    isZkProtocol ||
                    isWebJSApp ||
                    isUpgradeProving) &&
                setHovered(false)
            }
            style={{ position: "relative" }}
        >
            <div className={styles.nodeLabel}>{node.label}</div>
            {(isZkApp ||
                isMobileApp ||
                isZkProtocol ||
                isWebJSApp ||
                isUpgradeProving) && (
                <div className={styles.learnMore}>More</div>
            )}
            {isZkApp && hovered && (
                <div className={`${styles.hoverPopup} ${styles.zkAppPopup}`}>
                    <div className={styles.popupDesc}>
                        Use Mopro to jumpstart your ZK mobile development - it
                        provides native bindings and ready-to-use templates.
                    </div>
                    <a
                        href="/docs/getting-started"
                        className={styles.getStartedBtn}
                    >
                        Getting Started
                    </a>
                </div>
            )}
            {isMobileApp && hovered && (
                <div className={`${styles.hoverPopup} ${styles.mobileAppPopup}`}>
                    <div className={styles.popupDesc}>
                        Add privacy-preserving features without deep
                        cryptography expertise. Includes sample circuits and
                        easy-to-use APIs.
                    </div>
                    <a href="/docs/projects" className={styles.getStartedBtn}>
                        Find SDKs
                    </a>
                </div>
            )}
            {isZkProtocol && hovered && (
                <div className={styles.hoverPopup}>
                    <div className={styles.popupDesc}>
                        Package your protocol with mobile-ready SDKs and
                        generate native bindings for developers
                    </div>
                    <a
                        href="/docs/getting-started"
                        className={styles.getStartedBtn}
                    >
                        Getting Started
                    </a>
                </div>
            )}
            {isWebJSApp && hovered && (
                <div className={`${styles.hoverPopup} ${styles.webJSAppPopup}`}>
                    <div className={styles.popupDesc}>
                        Bringing existing ZK logic to mobile by building
                        circuits and APIs to mobile libs
                    </div>
                    <a
                        href="/docs/getting-started"
                        className={styles.getStartedBtn}
                    >
                        Getting Started
                    </a>
                </div>
            )}
            {isUpgradeProving && hovered && (
                <div className={styles.hoverPopup}>
                    <div className={styles.popupDesc}>
                        Mopro can help you maintain and upgrade your stack — by
                        generating bindings, simplifying updates to your proof
                        system
                    </div>
                    <a
                        href="/docs/adapters/overview"
                        className={styles.getStartedBtn}
                    >
                        Switch Adapters
                    </a>
                </div>
            )}
            {node.desc && <div className={styles.nodeDesc}>{node.desc}</div>}
            {node.tags && (
                <div className={styles.tags}>
                    {node.tags.map((tag: string) => (
                        <span key={tag} className={styles.tag}>
                            {tag}
                        </span>
                    ))}
                </div>
            )}
        </div>
    );
};

const IsMoproForMe = () => {
    // Create refs only once
    const nodeRefs = useMemo(
        () =>
            Object.fromEntries(
                nodes.map((n) => [n.id, React.createRef<HTMLDivElement>()])
            ),
        []
    );
    const [lines, setLines] = useState<any[]>([]);
    const graphWrapperRef = useRef<HTMLDivElement>(null);

    useLayoutEffect(() => {
        const updateLines = () => {
            const containerRect =
                graphWrapperRef.current?.getBoundingClientRect();
            if (!containerRect) return;
            const newLines = edges
                .map((edge) => {
                    const fromRef = nodeRefs[edge.from];
                    const toRef = nodeRefs[edge.to];
                    if (!fromRef.current || !toRef.current) return null;
                    const fromRect = fromRef.current.getBoundingClientRect();
                    const toRect = toRef.current.getBoundingClientRect();
                    const startX =
                        fromRect.left + fromRect.width / 2 - containerRect.left;
                    const startY = fromRect.bottom - containerRect.top;
                    const endX =
                        toRect.left + toRect.width / 2 - containerRect.left;
                    const endY = toRect.top - containerRect.top;
                    return { startX, startY, endX, endY, label: edge.label };
                })
                .filter(Boolean);
            setLines(newLines);
        };
        updateLines();
        window.addEventListener("resize", updateLines);
        return () => window.removeEventListener("resize", updateLines);
    }, []);

    return (
        <div className={styles.container}>
            <h1 className={styles.title}>Is MoPro for me?</h1>
            <p className={styles.subtitle}>
                Decision Tree - Find Your Perfect Use Case
            </p>
            <div
                className={styles.graphWrapper}
                ref={graphWrapperRef}
                style={{ position: "relative" }}
            >
                {/* SVG overlay for edges */}
                <svg
                    className={styles.graphSvg}
                    style={{
                        position: "absolute",
                        top: 0,
                        left: 0,
                        width: "100%",
                        height: "100%",
                        pointerEvents: "none",
                    }}
                >
                    {lines.map((line, i) => {
                        // Calculate a right-angle (elbow) path: vertical down from start, then horizontal to end
                        const midY = (line.startY + line.endY) / 2;
                        const path = `M ${line.startX} ${line.startY} L ${line.startX} ${midY} L ${line.endX} ${midY} L ${line.endX} ${line.endY}`;
                        return (
                            <g key={i}>
                                <path
                                    d={path}
                                    stroke="#FFB347"
                                    strokeWidth={2}
                                    fill="none"
                                    markerEnd="url(#arrowhead)"
                                />
                                {line.label && (
                                    <text
                                        x={(line.startX + line.endX) / 2}
                                        y={midY - 6}
                                        fontSize="12"
                                        fill="#fff"
                                        textAnchor="middle"
                                        fontWeight="bold"
                                    >
                                        {line.label}
                                    </text>
                                )}
                            </g>
                        );
                    })}
                    <defs>
                        <marker
                            id="arrowhead"
                            markerWidth="7"
                            markerHeight="7"
                            refX="7"
                            refY="3.5"
                            orient="auto"
                            markerUnits="strokeWidth"
                        >
                            <polygon points="0 0, 7 3.5, 0 7" fill="#FFB347" />
                        </marker>
                    </defs>
                </svg>
                <div className={styles.graphGrid}>
                    <Node
                        node={nodes.find((n) => n.id === "start")!}
                        nodeRef={nodeRefs["start"]}
                    />
                    <Node
                        node={nodes.find((n) => n.id === "existing")!}
                        nodeRef={nodeRefs["existing"]}
                    />
                    <div className={styles.graphRow}>
                        <div className={styles.graphCol}>
                            <Node
                                node={
                                    nodes.find((n) => n.id === "whatBuilding")!
                                }
                                nodeRef={nodeRefs["whatBuilding"]}
                            />
                            <div className={styles.graphRow}>
                                <Node
                                    node={nodes.find((n) => n.id === "zkApp")!}
                                    nodeRef={nodeRefs["zkApp"]}
                                />
                                <Node
                                    node={
                                        nodes.find((n) => n.id === "mobileApp")!
                                    }
                                    nodeRef={nodeRefs["mobileApp"]}
                                />
                            </div>
                        </div>
                        <div className={styles.graphCol}>
                            <Node
                                node={nodes.find((n) => n.id === "whatHave")!}
                                nodeRef={nodeRefs["whatHave"]}
                            />
                            <div className={styles.graphRow}>
                                <Node
                                    node={
                                        nodes.find(
                                            (n) => n.id === "zkProtocol"
                                        )!
                                    }
                                    nodeRef={nodeRefs["zkProtocol"]}
                                />
                                <Node
                                    node={
                                        nodes.find((n) => n.id === "webJSApp")!
                                    }
                                    nodeRef={nodeRefs["webJSApp"]}
                                />
                            </div>
                        </div>
                    </div>
                    <Node
                        node={nodes.find((n) => n.id === "upgradeProving")!}
                        nodeRef={nodeRefs["upgradeProving"]}
                    />
                </div>
            </div>
        </div>
    );
};

export default IsMoproForMe;
