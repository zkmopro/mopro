import React, { useState, useRef, useCallback, useEffect } from 'react';
import styles from './styles.module.css';
import { 
  architectureComponents, 
  architectureLayers, 
  connections, 
  roleComponents,
  ComponentData 
} from './architectureData';

interface ZoomState {
  scale: number;
  translateX: number;
  translateY: number;
}

interface HoveredComponent {
  component: ComponentData;
  x: number;
  y: number;
}

const ArchitectureDiagram: React.FC = () => {
  const [zoomState, setZoomState] = useState<ZoomState>({
    scale: 0.8,
    translateX: 50,
    translateY: 20
  });
  const [hoveredComponent, setHoveredComponent] = useState<HoveredComponent | null>(null);
  const [isDragging, setIsDragging] = useState(false);
  const [dragStart, setDragStart] = useState({ x: 0, y: 0 });
  const svgRef = useRef<SVGSVGElement>(null);
  const hideTooltipTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Theme-aware color mapping function
  const getThemedColor = useCallback((originalColor: string | undefined): string => {
    // Handle undefined/null colors with defaults
    if (!originalColor) {
      return 'var(--arch-component-text)'; // Default to component text color
    }
    
    const colorMap: { [key: string]: string } = {
      // Layer colors
      '#E1F5FE': 'var(--arch-roles-bg)',
      '#FFCDD2': 'var(--arch-red-bg)',
      '#FFF3E0': 'var(--arch-orange-bg)',
      '#E8F5E8': 'var(--arch-teal-bg)', // For circuits layer
      '#F3E5F5': 'var(--arch-purple-bg)', // For middleware/proving layers
      // Text colors  
      '#0277BD': 'var(--arch-roles-text)',
      '#D32F2F': 'var(--arch-red-text)', 
      '#F57C00': 'var(--arch-orange-text)',
      '#4CAF50': 'var(--arch-teal-text)',
      '#9C27B0': 'var(--arch-purple-text)',
      // Component colors
      '#FFFFFF': 'var(--arch-component-bg)',
      '#000000': 'var(--arch-component-text)',
      '#000': 'var(--arch-component-text)', // Handle short hex
      // Role label colors
      '#BBDEFB': 'var(--arch-role-label-bg)',
      '#1976D2': 'var(--arch-role-label-text)',
    };
    
    return colorMap[originalColor] || originalColor;
  }, []);

  const handleWheel = useCallback((e: React.WheelEvent) => {
    e.preventDefault();
    const delta = e.deltaY * -0.001;
    const newScale = Math.min(Math.max(0.3, zoomState.scale + delta), 2);
    
    setZoomState((prev: ZoomState) => ({
      ...prev,
      scale: newScale
    }));
  }, [zoomState.scale]);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    setIsDragging(true);
    setDragStart({ x: e.clientX - zoomState.translateX, y: e.clientY - zoomState.translateY });
  }, [zoomState.translateX, zoomState.translateY]);

  const handleMouseMove = useCallback((e: React.MouseEvent) => {
    if (isDragging) {
      setZoomState((prev: ZoomState) => ({
        ...prev,
        translateX: e.clientX - dragStart.x,
        translateY: e.clientY - dragStart.y
      }));
    }
  }, [isDragging, dragStart]);

  const handleMouseUp = useCallback(() => {
    setIsDragging(false);
  }, []);

  const handleComponentHover = useCallback((component: ComponentData, e: React.MouseEvent<SVGGElement>) => {
    // Clear any pending hide timeout
    if (hideTooltipTimeoutRef.current) {
      clearTimeout(hideTooltipTimeoutRef.current);
      hideTooltipTimeoutRef.current = null;
    }
    
    const rect = svgRef.current?.getBoundingClientRect();
    if (rect) {
      setHoveredComponent({
        component,
        x: e.clientX - rect.left,
        y: e.clientY - rect.top
      });
    }
  }, []);

  const handleComponentLeave = useCallback(() => {
    // Delay hiding the tooltip to allow hovering over it
    hideTooltipTimeoutRef.current = setTimeout(() => {
      setHoveredComponent(null);
    }, 300); // 300ms delay
  }, []);

  const handleTooltipEnter = useCallback(() => {
    // Cancel hiding when hovering over tooltip
    if (hideTooltipTimeoutRef.current) {
      clearTimeout(hideTooltipTimeoutRef.current);
      hideTooltipTimeoutRef.current = null;
    }
  }, []);

  const handleTooltipLeave = useCallback(() => {
    // Hide immediately when leaving tooltip
    setHoveredComponent(null);
  }, []);

  const resetZoom = useCallback(() => {
    setZoomState({
      scale: 0.8,
      translateX: 50,
      translateY: 20
    });
  }, []);

  const zoomIn = useCallback(() => {
    setZoomState((prev: ZoomState) => ({
      ...prev,
      scale: Math.min(2, prev.scale + 0.2)
    }));
  }, []);

  const zoomOut = useCallback(() => {
    setZoomState((prev: ZoomState) => ({
      ...prev,
      scale: Math.max(0.3, prev.scale - 0.2)
    }));
  }, []);

  // Create connection paths
  const createConnectionPath = (fromId: string, toId: string) => {
    // Search in both architectureComponents and roleComponents
    const allComponents = [...architectureComponents, ...roleComponents.map(role => ({
      id: role.id,
      position: role.position,
      width: 110,
      height: 25
    }))];
    
    const fromComponent = allComponents.find(c => c.id === fromId);
    const toComponent = allComponents.find(c => c.id === toId);
    
    if (!fromComponent || !toComponent) return '';
    
    // Calculate border-centered connection points
    let fromX, fromY, toX, toY;
    
    // Check if this is a role-related connection (from roles or brain labels)
    const isRoleConnection = fromId.includes('brain') || 
                           ['user', 'app-dev', 'tooling-infra-dev', 'circuits-dev'].includes(fromId);
    
    if (isRoleConnection) {
      // For role connections, always start from bottom center
      fromX = fromComponent.position.x + fromComponent.width / 2;
      fromY = fromComponent.position.y + fromComponent.height;
      toX = toComponent.position.x + toComponent.width / 2;
      toY = toComponent.position.y;
    } else {
      // For non-role connections, use the smart detection logic
      const fromCenterX = fromComponent.position.x + fromComponent.width / 2;
      const fromCenterY = fromComponent.position.y + fromComponent.height / 2;
      const toCenterX = toComponent.position.x + toComponent.width / 2;
      const toCenterY = toComponent.position.y + toComponent.height / 2;
      
      if (Math.abs(fromCenterY - toCenterY) > Math.abs(fromCenterX - toCenterX)) {
        // Vertical connection
        if (fromCenterY < toCenterY) {
          // From component is above
          fromX = fromCenterX;
          fromY = fromComponent.position.y + fromComponent.height;
          toX = toCenterX;
          toY = toComponent.position.y;
        } else {
          // From component is below
          fromX = fromCenterX;
          fromY = fromComponent.position.y;
          toX = toCenterX;
          toY = toComponent.position.y + toComponent.height;
        }
      } else {
        // Horizontal connection
        if (fromCenterX < toCenterX) {
          // From component is to the left
          fromX = fromComponent.position.x + fromComponent.width;
          fromY = fromCenterY;
          toX = toComponent.position.x;
          toY = toCenterY;
        } else {
          // From component is to the right
          fromX = fromComponent.position.x;
          fromY = fromCenterY;
          toX = toComponent.position.x + toComponent.width;
          toY = toCenterY;
        }
      }
    }
    
    // Create straight line path
    return `M ${fromX} ${fromY} L ${toX} ${toY}`;
  };

  useEffect(() => {
    const handleGlobalMouseUp = () => setIsDragging(false);
    document.addEventListener('mouseup', handleGlobalMouseUp);
    return () => document.removeEventListener('mouseup', handleGlobalMouseUp);
  }, []);

  useEffect(() => {
    // Cleanup timeout on unmount
    return () => {
      if (hideTooltipTimeoutRef.current) {
        clearTimeout(hideTooltipTimeoutRef.current);
      }
    };
  }, []);

  return (
    <div className={styles.container}>
      <div className={styles.controls}>
        <button onClick={zoomIn} className={styles.controlButton} title="Zoom In">
          +
        </button>
        <button onClick={zoomOut} className={styles.controlButton} title="Zoom Out">
          −
        </button>
        <button onClick={resetZoom} className={styles.controlButton} title="Reset View">
          ⌂
        </button>
      </div>
      
      <div className={styles.diagramContainer}>
        <svg
          ref={svgRef}
          className={styles.svg}
          onWheel={handleWheel}
          onMouseDown={handleMouseDown}
          onMouseMove={handleMouseMove}
          onMouseUp={handleMouseUp}
          style={{ cursor: isDragging ? 'grabbing' : 'grab' }}
        >
          <g
            transform={`translate(${zoomState.translateX}, ${zoomState.translateY}) scale(${zoomState.scale})`}
          >
            {/* Background layers */}
            {architectureLayers.map((layer) => (
              <g key={layer.id}>
                <rect
                  x={layer.position.x}
                  y={layer.position.y}
                  width={layer.width}
                  height={layer.height}
                  fill={getThemedColor(layer.color)}
                  stroke="rgba(128, 128, 128, 0.3)"
                  strokeWidth="1"
                  rx="8"
                />
                <text
                  x={layer.position.x + layer.width / 2}
                  y={layer.id === 'roles' ? layer.position.y + 15 : layer.position.y + 25}
                  fill={getThemedColor(layer.textColor)}
                  fontSize="14"
                  fontWeight="bold"
                  textAnchor="middle"
                >
                  {layer.name}
                </text>
              </g>
            ))}

            {/* Role components */}
            {roleComponents.map((role) => {
              const isDevRole = ['app-dev', 'tooling-infra-dev', 'circuits-dev'].includes(role.id);
              return (
                <g key={role.id}>
                  <rect
                    x={role.position.x}
                    y={role.position.y}
                    width={110}
                    height={25}
                    fill={getThemedColor(role.color)}
                    stroke={role.id === 'no-brain-label' ? getThemedColor("#D32F2F") : (isDevRole ? "var(--arch-component-text)" : getThemedColor("#1976D2"))}
                    strokeWidth="1"
                    rx={isDevRole ? "4" : "12"}
                  />
                  <text
                    x={role.position.x + 55}
                    y={role.position.y + 17}
                    fill={getThemedColor(role.textColor)}
                    fontSize={role.id === 'no-brain-label' ? "11" : "12"}
                    fontWeight="bold"
                    textAnchor="middle"
                  >
                    {role.name}
                  </text>
                </g>
              );
            })}

            {/* Connection lines */}
            <g className={styles.connections}>
              {connections.map((connection, index) => (
                <path
                  key={index}
                  d={createConnectionPath(connection.from, connection.to)}
                  stroke="var(--arch-component-text)"
                  strokeWidth="2"
                  fill="none"
                  strokeDasharray="5,5"
                  opacity="0.5"
                />
              ))}
            </g>

            {/* Components */}
            {architectureComponents.filter(component => component.category !== 'connection-point').map((component) => (
              <g
                key={component.id}
                onMouseEnter={(e) => handleComponentHover(component, e)}
                onMouseLeave={handleComponentLeave}
                style={{ cursor: 'pointer' }}
              >
                <rect
                  x={component.position.x}
                  y={component.position.y}
                  width={component.width}
                  height={component.height}
                  fill={getThemedColor(component.color)}
                  stroke="var(--arch-component-text)"
                  strokeWidth="1"
                  rx="4"
                  className={styles.component}
                />
                {component.displayName ? (
                  // Multi-line text
                  component.displayName.map((line, index) => (
                    <text
                      key={index}
                      x={component.position.x + component.width / 2}
                      y={component.position.y + component.height / 2 - (component.displayName!.length - 1) * 6 + index * 12}
                      fill={getThemedColor(component.textColor || '#000')}
                      fontSize="11"
                      textAnchor="middle"
                      dominantBaseline="middle"
                      fontWeight="bold"
                      style={{ pointerEvents: 'none' }}
                    >
                      {line}
                    </text>
                  ))
                ) : (
                  // Single line text
                  <text
                    x={component.position.x + component.width / 2}
                    y={component.position.y + component.height / 2}
                    fill={getThemedColor(component.textColor || '#000')}
                    fontSize="11"
                    textAnchor="middle"
                    dominantBaseline="middle"
                    fontWeight='bold'
                    style={{ pointerEvents: 'none' }}
                  >
                    {component.name}
                  </text>
                )}
              </g>
            ))}

            {/* Arrow marker definition */}
            <defs>
              <marker
                id="arrowhead"
                markerWidth="10"
                markerHeight="7"
                refX="9"
                refY="3.5"
                orient="auto"
              >
                <polygon
                  points="0 0, 10 3.5, 0 7"
                  fill="#666"
                />
              </marker>
            </defs>
          </g>
        </svg>

        {/* Tooltip */}
        {hoveredComponent && (
          <div
            className={styles.tooltip}
            style={{
              left: hoveredComponent.x + 5,
              top: hoveredComponent.y + 5
            }}
            onMouseEnter={handleTooltipEnter}
            onMouseLeave={handleTooltipLeave}
          >
            <h4>{hoveredComponent.component.name}</h4>
            <p>{hoveredComponent.component.description}</p>
            {hoveredComponent.component.links && hoveredComponent.component.links.length > 0 && (
              <div className={styles.links}>
                {hoveredComponent.component.links.map((link: string, index: number) => (
                  <a
                    key={index}
                    href={link}
                    target="_blank"
                    rel="noopener noreferrer"
                    className={styles.link}
                  >
                    Learn more →
                  </a>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

    </div>
  );
};

export default ArchitectureDiagram;