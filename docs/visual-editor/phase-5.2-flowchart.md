# Phase 5.2: Flowchart View

Visualize scenario structure with an interactive flowchart.

## Goal

Users can see the scenario flow at a glance and navigate by clicking nodes.

## Prerequisites

- Phase 5.1 completed (Tauri project, IPC commands)

## Tasks

### 1. Tauri IPC Command

```rust
// src/commands/flowchart.rs
use crate::flowchart::{build_flowchart, Flowchart, FlowchartNode, FlowchartEdge, NodeType, EdgeType};
use crate::scenario::Scenario;
use serde::Serialize;

#[derive(Serialize)]
pub struct FlowchartData {
    pub nodes: Vec<NodeData>,
    pub edges: Vec<EdgeData>,
}

#[derive(Serialize)]
pub struct NodeData {
    pub id: String,
    pub node_type: String,
    pub label: String,
    pub script_index: usize,
    pub preview: Option<String>,
}

#[derive(Serialize)]
pub struct EdgeData {
    pub id: String,
    pub source: String,
    pub target: String,
    pub edge_type: String,
    pub label: Option<String>,
}

#[tauri::command]
pub fn get_flowchart(scenario: Scenario) -> FlowchartData {
    let flowchart = build_flowchart(&scenario);

    let nodes = flowchart.nodes.iter().map(|node| {
        let (node_type, label) = match &node.node_type {
            NodeType::Start => ("start".to_string(), "Start".to_string()),
            NodeType::Label { name } => ("label".to_string(), name.clone()),
            NodeType::Choice { options } => ("choice".to_string(), format!("{} choices", options.len())),
            NodeType::Conditional { var, .. } => ("conditional".to_string(), format!("if {}", var)),
            NodeType::End => ("end".to_string(), "End".to_string()),
        };

        NodeData {
            id: format!("node-{}", node.id.0),
            node_type,
            label,
            script_index: node.script_index,
            preview: node.preview.clone(),
        }
    }).collect();

    let edges = flowchart.edges.iter().enumerate().map(|(i, edge)| {
        let edge_type = match edge.edge_type {
            EdgeType::Sequential => "sequential",
            EdgeType::Jump => "jump",
            EdgeType::Choice(_) => "choice",
            EdgeType::Conditional => "conditional",
        };

        EdgeData {
            id: format!("edge-{}", i),
            source: format!("node-{}", edge.from.0),
            target: format!("node-{}", edge.to.0),
            edge_type: edge_type.to_string(),
            label: edge.label.clone(),
        }
    }).collect();

    FlowchartData { nodes, edges }
}
```

### 2. Dependencies

```bash
cd editors/ivy-editor/ui
npm install @xyflow/react dagre @types/dagre
```

### 3. Auto Layout with Dagre

```typescript
// ui/src/utils/layoutFlowchart.ts
import dagre from 'dagre';
import type { Node, Edge } from '@xyflow/react';

interface LayoutOptions {
  direction: 'TB' | 'LR';
  nodeWidth: number;
  nodeHeight: number;
  rankSep: number;
  nodeSep: number;
}

const defaultOptions: LayoutOptions = {
  direction: 'TB',
  nodeWidth: 180,
  nodeHeight: 60,
  rankSep: 80,
  nodeSep: 40,
};

export function layoutFlowchart(
  nodes: Node[],
  edges: Edge[],
  options: Partial<LayoutOptions> = {}
): { nodes: Node[]; edges: Edge[] } {
  const opts = { ...defaultOptions, ...options };

  const dagreGraph = new dagre.graphlib.Graph();
  dagreGraph.setDefaultEdgeLabel(() => ({}));
  dagreGraph.setGraph({
    rankdir: opts.direction,
    ranksep: opts.rankSep,
    nodesep: opts.nodeSep,
  });

  nodes.forEach((node) => {
    dagreGraph.setNode(node.id, {
      width: opts.nodeWidth,
      height: opts.nodeHeight,
    });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  const layoutedNodes = nodes.map((node) => {
    const dagreNode = dagreGraph.node(node.id);
    return {
      ...node,
      position: {
        x: dagreNode.x - opts.nodeWidth / 2,
        y: dagreNode.y - opts.nodeHeight / 2,
      },
    };
  });

  return { nodes: layoutedNodes, edges };
}
```

### 4. Custom Node Components

```typescript
// ui/src/components/FlowchartView/nodes/StartNode.tsx
import { FC } from 'react';
import { Handle, Position, NodeProps } from '@xyflow/react';

interface StartNodeData {
  label: string;
}

export const StartNode: FC<NodeProps<StartNodeData>> = ({ data }) => {
  return (
    <div className="flowchart-node start-node">
      <div className="node-content">{data.label}</div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
```

```typescript
// ui/src/components/FlowchartView/nodes/LabelNode.tsx
import { FC } from 'react';
import { Handle, Position, NodeProps } from '@xyflow/react';

interface LabelNodeData {
  label: string;
  preview?: string;
  scriptIndex: number;
}

export const LabelNode: FC<NodeProps<LabelNodeData>> = ({ data }) => {
  return (
    <div className="flowchart-node label-node">
      <Handle type="target" position={Position.Top} />
      <div className="node-label">{data.label}</div>
      {data.preview && <div className="node-preview">{data.preview}</div>}
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
```

```typescript
// ui/src/components/FlowchartView/nodes/ChoiceNode.tsx
import { FC } from 'react';
import { Handle, Position, NodeProps } from '@xyflow/react';

interface ChoiceNodeData {
  label: string;
  options: string[];
}

export const ChoiceNode: FC<NodeProps<ChoiceNodeData>> = ({ data }) => {
  return (
    <div className="flowchart-node choice-node">
      <Handle type="target" position={Position.Top} />
      <div className="node-label">{data.label}</div>
      <div className="choice-options">
        {data.options.map((opt, i) => (
          <div key={i} className="choice-option">{opt}</div>
        ))}
      </div>
      <Handle type="source" position={Position.Bottom} />
    </div>
  );
};
```

```typescript
// ui/src/components/FlowchartView/nodes/EndNode.tsx
import { FC } from 'react';
import { Handle, Position, NodeProps } from '@xyflow/react';

interface EndNodeData {
  label: string;
}

export const EndNode: FC<NodeProps<EndNodeData>> = ({ data }) => {
  return (
    <div className="flowchart-node end-node">
      <Handle type="target" position={Position.Top} />
      <div className="node-content">{data.label}</div>
    </div>
  );
};
```

### 5. FlowchartView Component

```typescript
// ui/src/components/FlowchartView/index.tsx
import { FC, useCallback, useEffect, useMemo, useState } from 'react';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  useNodesState,
  useEdgesState,
  Node,
  Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import { invoke } from '@tauri-apps/api/core';
import type { Scenario } from '../../types/scenario';
import { layoutFlowchart } from '../../utils/layoutFlowchart';
import { StartNode } from './nodes/StartNode';
import { LabelNode } from './nodes/LabelNode';
import { ChoiceNode } from './nodes/ChoiceNode';
import { EndNode } from './nodes/EndNode';

interface FlowchartData {
  nodes: {
    id: string;
    node_type: string;
    label: string;
    script_index: number;
    preview?: string;
  }[];
  edges: {
    id: string;
    source: string;
    target: string;
    edge_type: string;
    label?: string;
  }[];
}

interface Props {
  scenario: Scenario;
  onNodeClick: (scriptIndex: number) => void;
}

const nodeTypes = {
  start: StartNode,
  label: LabelNode,
  choice: ChoiceNode,
  conditional: LabelNode,
  end: EndNode,
};

const edgeStyles = {
  sequential: { stroke: '#666', strokeWidth: 2 },
  jump: { stroke: '#f59e0b', strokeWidth: 2, strokeDasharray: '5,5' },
  choice: { stroke: '#3b82f6', strokeWidth: 2 },
  conditional: { stroke: '#10b981', strokeWidth: 2, strokeDasharray: '3,3' },
};

export const FlowchartView: FC<Props> = ({ scenario, onNodeClick }) => {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  useEffect(() => {
    const loadFlowchart = async () => {
      const data = await invoke<FlowchartData>('get_flowchart', { scenario });

      const rawNodes: Node[] = data.nodes.map((node) => ({
        id: node.id,
        type: node.node_type,
        position: { x: 0, y: 0 },
        data: {
          label: node.label,
          preview: node.preview,
          scriptIndex: node.script_index,
        },
      }));

      const rawEdges: Edge[] = data.edges.map((edge) => ({
        id: edge.id,
        source: edge.source,
        target: edge.target,
        label: edge.label,
        style: edgeStyles[edge.edge_type as keyof typeof edgeStyles],
        animated: edge.edge_type === 'jump',
      }));

      const { nodes: layoutedNodes, edges: layoutedEdges } = layoutFlowchart(
        rawNodes,
        rawEdges
      );

      setNodes(layoutedNodes);
      setEdges(layoutedEdges);
    };

    loadFlowchart();
  }, [scenario, setNodes, setEdges]);

  const handleNodeClick = useCallback(
    (_: React.MouseEvent, node: Node) => {
      const scriptIndex = node.data.scriptIndex as number;
      if (scriptIndex !== undefined) {
        onNodeClick(scriptIndex);
      }
    },
    [onNodeClick]
  );

  return (
    <div className="flowchart-view">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onNodeClick={handleNodeClick}
        nodeTypes={nodeTypes}
        fitView
      >
        <Background />
        <Controls />
        <MiniMap />
      </ReactFlow>
    </div>
  );
};
```

### 6. Styles

```css
/* ui/src/components/FlowchartView/styles.css */
.flowchart-view {
  width: 100%;
  height: 100%;
  min-height: 400px;
}

.flowchart-node {
  padding: 10px 15px;
  border-radius: 8px;
  font-size: 12px;
  min-width: 120px;
  text-align: center;
}

.start-node {
  background: #22c55e;
  color: white;
  border-radius: 50%;
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.label-node {
  background: #3b82f6;
  color: white;
  border: 2px solid #1d4ed8;
}

.choice-node {
  background: #f59e0b;
  color: white;
  border: 2px solid #d97706;
}

.choice-node .choice-options {
  margin-top: 5px;
  font-size: 10px;
}

.choice-node .choice-option {
  background: rgba(255, 255, 255, 0.2);
  padding: 2px 5px;
  margin: 2px 0;
  border-radius: 3px;
}

.end-node {
  background: #ef4444;
  color: white;
  border-radius: 50%;
  width: 60px;
  height: 60px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.node-label {
  font-weight: bold;
}

.node-preview {
  font-size: 10px;
  opacity: 0.8;
  margin-top: 4px;
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
```

### 7. Integration with App

```typescript
// ui/src/App.tsx (updated)
import { useState } from 'react';
import { FlowchartView } from './components/FlowchartView';

function App() {
  const { scenario, selectedIndex, selectCommand, /* ... */ } = useScenario();
  const [view, setView] = useState<'list' | 'flowchart'>('list');

  return (
    <div className="app">
      <header className="toolbar">
        {/* ... existing buttons */}
        <div className="view-toggle">
          <button
            className={view === 'list' ? 'active' : ''}
            onClick={() => setView('list')}
          >
            List
          </button>
          <button
            className={view === 'flowchart' ? 'active' : ''}
            onClick={() => setView('flowchart')}
          >
            Flowchart
          </button>
        </div>
      </header>

      <main className="main-content">
        <aside className="sidebar">
          {view === 'list' && scenario && (
            <CommandList /* ... */ />
          )}
          {view === 'flowchart' && scenario && (
            <FlowchartView
              scenario={scenario}
              onNodeClick={selectCommand}
            />
          )}
        </aside>
        {/* ... rest of layout */}
      </main>
    </div>
  );
}
```

## Node Type Mapping

| Rust `NodeType` | React Flow Type | Color | Shape |
|-----------------|-----------------|-------|-------|
| `Start` | `start` | Green | Circle |
| `Label` | `label` | Blue | Rounded rect |
| `Choice` | `choice` | Orange | Rounded rect |
| `Conditional` | `conditional` | Green | Diamond-like |
| `End` | `end` | Red | Circle |

## Edge Type Mapping

| Rust `EdgeType` | Style | Color |
|-----------------|-------|-------|
| `Sequential` | Solid | Gray |
| `Jump` | Dashed, animated | Orange |
| `Choice` | Solid | Blue |
| `Conditional` | Dashed | Green |

## Verification

1. Open scenario with labels and choices
2. Verify flowchart displays correctly
3. Click node → editor jumps to that command
4. Zoom/pan works
5. MiniMap shows overview
