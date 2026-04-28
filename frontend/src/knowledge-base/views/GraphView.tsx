import { useEffect, useRef, useCallback, useState } from "react";
import { KnowledgeItem } from "../types";
import { buildGraphData, GraphNode, GraphEdge } from "../graphData";

type Vec2 = { x: number; y: number };

export type GraphViewProps = {
  items: KnowledgeItem[];
  onSelectItem: (id: string) => void;
};

const SIM_STOP_THRESHOLD = 0.05;
const MAX_LABEL_LEN = 20;

export function GraphView(props: GraphViewProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const dataRef = useRef<{ nodes: (GraphNode & Vec2 & { vx: number; vy: number })[]; edges: GraphEdge[] }>({ nodes: [], edges: [] });
  const transformRef = useRef({ x: 0, y: 0, scale: 1 });
  const draggingRef = useRef<string | null>(null);
  const mouseRef = useRef({ x: 0, y: 0 });
  const animRef = useRef<number>(0);
  const simTickRef = useRef(0);
  const stableRef = useRef(false);
  const [selectedId, setSelectedId] = useState<string | null>(null);

  const initData = useCallback(() => {
    const data = buildGraphData(props.items);
    const width = canvasRef.current?.width || 800;
    const height = canvasRef.current?.height || 600;
    const nodes = data.nodes.map((n) => ({
      ...n,
      x: Math.random() * width,
      y: Math.random() * height,
      vx: 0,
      vy: 0,
    }));
    dataRef.current = { nodes, edges: data.edges };
    transformRef.current = { x: width / 2, y: height / 2, scale: 1 };
    simTickRef.current = 0;
    stableRef.current = false;
  }, [props.items]);

  useEffect(() => {
    initData();
  }, [initData]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const resize = () => {
      const rect = canvas.parentElement?.getBoundingClientRect();
      if (rect) {
        canvas.width = rect.width;
        canvas.height = rect.height;
      }
    };
    resize();
    window.addEventListener("resize", resize);

    const step = () => {
      const nodes = dataRef.current.nodes;
      if (!stableRef.current && nodes.length > 0) {
        const shouldSim = nodes.length <= 150 || simTickRef.current % 2 === 0;
        if (shouldSim) {
          simulate(nodes, dataRef.current.edges, canvas.width, canvas.height);
          const energy = nodes.reduce((s, n) => s + Math.hypot(n.vx, n.vy), 0) / nodes.length;
          if (energy < SIM_STOP_THRESHOLD && simTickRef.current > 60) {
            stableRef.current = true;
          }
        }
        simTickRef.current++;
      }
      render(ctx, canvas.width, canvas.height, dataRef.current.nodes, dataRef.current.edges, transformRef.current, selectedId);
      animRef.current = requestAnimationFrame(step);
    };
    animRef.current = requestAnimationFrame(step);

    return () => {
      window.removeEventListener("resize", resize);
      cancelAnimationFrame(animRef.current);
    };
  }, [selectedId]);

  const handleWheel = (e: React.WheelEvent) => {
    e.preventDefault();
    const t = transformRef.current;
    const scaleFactor = e.deltaY > 0 ? 0.9 : 1.1;
    t.scale = Math.max(0.2, Math.min(3, t.scale * scaleFactor));
  };

  const handleMouseDown = (e: React.MouseEvent) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const rect = canvas.getBoundingClientRect();
    const x = (e.clientX - rect.left - transformRef.current.x) / transformRef.current.scale;
    const y = (e.clientY - rect.top - transformRef.current.y) / transformRef.current.scale;
    const node = dataRef.current.nodes.find((n) => Math.hypot(n.x - x, n.y - y) < n.radius + 4);
    if (node) {
      draggingRef.current = node.id;
      setSelectedId(node.id);
    }
    mouseRef.current = { x: e.clientX, y: e.clientY };
  };

  const handleMouseMove = (e: React.MouseEvent) => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    if (draggingRef.current) {
      const dx = (e.clientX - mouseRef.current.x) / transformRef.current.scale;
      const dy = (e.clientY - mouseRef.current.y) / transformRef.current.scale;
      const node = dataRef.current.nodes.find((n) => n.id === draggingRef.current);
      if (node) {
        node.x += dx;
        node.y += dy;
        node.vx = 0;
        node.vy = 0;
      }
    }
    mouseRef.current = { x: e.clientX, y: e.clientY };
  };

  const handleMouseUp = () => {
    if (draggingRef.current && selectedId) {
      props.onSelectItem(selectedId);
    }
    draggingRef.current = null;
  };

  return (
    <div className="kb-graph-layout">
      <canvas
        ref={canvasRef}
        className="kb-graph-canvas"
        onWheel={handleWheel}
        onMouseDown={handleMouseDown}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
      />
      <div className="kb-graph-hint">
        滚轮缩放 · 拖拽节点 · 点击打开笔记
      </div>
    </div>
  );
}

function simulate(
  nodes: (GraphNode & Vec2 & { vx: number; vy: number })[],
  edges: GraphEdge[],
  width: number,
  height: number,
) {
  const centerX = width / 2;
  const centerY = height / 2;

  for (let i = 0; i < nodes.length; i++) {
    for (let j = i + 1; j < nodes.length; j++) {
      const a = nodes[i];
      const b = nodes[j];
      const dx = a.x - b.x;
      const dy = a.y - b.y;
      const dist = Math.hypot(dx, dy) || 1;
      const force = 800 / (dist * dist);
      const fx = (dx / dist) * force;
      const fy = (dy / dist) * force;
      a.vx += fx;
      a.vy += fy;
      b.vx -= fx;
      b.vy -= fy;
    }
  }

  edges.forEach((edge) => {
    const a = nodes.find((n) => n.id === edge.source);
    const b = nodes.find((n) => n.id === edge.target);
    if (!a || !b) return;
    const dx = b.x - a.x;
    const dy = b.y - a.y;
    const dist = Math.hypot(dx, dy) || 1;
    const targetDist = 120;
    const force = ((dist - targetDist) / targetDist) * 0.05 * edge.strength;
    const fx = (dx / dist) * force;
    const fy = (dy / dist) * force;
    a.vx += fx;
    a.vy += fy;
    b.vx -= fx;
    b.vy -= fy;
  });

  nodes.forEach((n) => {
    n.vx += (centerX - n.x) * 0.0005;
    n.vy += (centerY - n.y) * 0.0005;
    n.vx *= 0.85;
    n.vy *= 0.85;
    n.x += n.vx;
    n.y += n.vy;
  });
}

function truncateLabel(label: string, max: number): string {
  if (label.length <= max) return label;
  return label.slice(0, max) + "…";
}

function render(
  ctx: CanvasRenderingContext2D,
  width: number,
  height: number,
  nodes: (GraphNode & Vec2)[],
  edges: GraphEdge[],
  transform: { x: number; y: number; scale: number },
  selectedId: string | null,
) {
  ctx.clearRect(0, 0, width, height);
  ctx.save();
  ctx.translate(transform.x, transform.y);
  ctx.scale(transform.scale, transform.scale);

  const invScale = 1 / transform.scale;
  const viewLeft = -transform.x * invScale;
  const viewTop = -transform.y * invScale;
  const viewRight = (width - transform.x) * invScale;
  const viewBottom = (height - transform.y) * invScale;
  const margin = 50 * invScale;

  const isOutside = (x: number, y: number, r: number) =>
    x + r < viewLeft - margin ||
    x - r > viewRight + margin ||
    y + r < viewTop - margin ||
    y - r > viewBottom + margin;

  edges.forEach((edge) => {
    const a = nodes.find((n) => n.id === edge.source);
    const b = nodes.find((n) => n.id === edge.target);
    if (!a || !b) return;
    if (isOutside(a.x, a.y, 0) && isOutside(b.x, b.y, 0)) return;
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.strokeStyle = "rgba(125,135,156,0.25)";
    ctx.lineWidth = Math.max(0.5, edge.strength * 0.8);
    ctx.stroke();
  });

  const hideLabels = transform.scale < 0.5 || nodes.length > 300;
  nodes.forEach((n) => {
    if (isOutside(n.x, n.y, n.radius)) return;

    ctx.beginPath();
    ctx.arc(n.x, n.y, n.radius, 0, Math.PI * 2);
    ctx.fillStyle = n.color;
    ctx.fill();
    if (n.id === selectedId) {
      ctx.strokeStyle = "#fff";
      ctx.lineWidth = 2;
      ctx.stroke();
    }

    if (!hideLabels || n.id === selectedId) {
      ctx.fillStyle = "#c9d1d9";
      ctx.font = "12px sans-serif";
      ctx.textAlign = "center";
      ctx.fillText(truncateLabel(n.label, MAX_LABEL_LEN), n.x, n.y + n.radius + 14);
    }
  });

  ctx.restore();
}
