import { useState, useEffect, useRef } from "react";

interface ParticlePickerProps {
  value: string | undefined;
  intensity: number | undefined;
  onChange: (value: string | undefined) => void;
  onIntensityChange: (value: number | undefined) => void;
}

interface ParticleType {
  id: string;
  label: string;
  color: string;
  shape: "circle" | "line" | "petal" | "star" | "leaf";
  speed: number;
  count: number;
}

const PARTICLE_TYPES: ParticleType[] = [
  { id: "snow", label: "Snow", color: "#ffffff", shape: "circle", speed: 1, count: 30 },
  { id: "rain", label: "Rain", color: "#6eb5ff", shape: "line", speed: 3, count: 40 },
  { id: "sakura", label: "Sakura", color: "#ffb7c5", shape: "petal", speed: 0.8, count: 20 },
  { id: "sparkle", label: "Sparkle", color: "#ffd700", shape: "star", speed: 0.5, count: 15 },
  { id: "leaves", label: "Leaves", color: "#8bc34a", shape: "leaf", speed: 1.2, count: 25 },
];

interface Particle {
  x: number;
  y: number;
  size: number;
  speed: number;
  opacity: number;
  rotation: number;
  rotationSpeed: number;
}

function createParticle(canvasWidth: number, canvasHeight: number, type: ParticleType): Particle {
  return {
    x: Math.random() * canvasWidth,
    y: Math.random() * canvasHeight - canvasHeight,
    size: 2 + Math.random() * 4,
    speed: type.speed * (0.5 + Math.random()),
    opacity: 0.5 + Math.random() * 0.5,
    rotation: Math.random() * 360,
    rotationSpeed: (Math.random() - 0.5) * 2,
  };
}

function drawParticle(
  ctx: CanvasRenderingContext2D,
  particle: Particle,
  type: ParticleType
) {
  ctx.save();
  ctx.globalAlpha = particle.opacity;
  ctx.fillStyle = type.color;
  ctx.strokeStyle = type.color;
  ctx.translate(particle.x, particle.y);
  ctx.rotate((particle.rotation * Math.PI) / 180);

  switch (type.shape) {
    case "circle":
      ctx.beginPath();
      ctx.arc(0, 0, particle.size, 0, Math.PI * 2);
      ctx.fill();
      break;
    case "line":
      ctx.lineWidth = 1;
      ctx.beginPath();
      ctx.moveTo(0, -particle.size * 2);
      ctx.lineTo(0, particle.size * 2);
      ctx.stroke();
      break;
    case "petal":
      ctx.beginPath();
      ctx.ellipse(0, 0, particle.size, particle.size * 0.6, 0, 0, Math.PI * 2);
      ctx.fill();
      break;
    case "star":
      drawStar(ctx, 0, 0, 5, particle.size, particle.size * 0.5);
      ctx.fill();
      break;
    case "leaf":
      ctx.beginPath();
      ctx.ellipse(0, 0, particle.size * 0.4, particle.size, 0, 0, Math.PI * 2);
      ctx.fill();
      break;
  }

  ctx.restore();
}

function drawStar(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  spikes: number,
  outerRadius: number,
  innerRadius: number
) {
  let rot = (Math.PI / 2) * 3;
  const step = Math.PI / spikes;

  ctx.beginPath();
  ctx.moveTo(cx, cy - outerRadius);

  for (let i = 0; i < spikes; i++) {
    ctx.lineTo(cx + Math.cos(rot) * outerRadius, cy + Math.sin(rot) * outerRadius);
    rot += step;
    ctx.lineTo(cx + Math.cos(rot) * innerRadius, cy + Math.sin(rot) * innerRadius);
    rot += step;
  }

  ctx.lineTo(cx, cy - outerRadius);
  ctx.closePath();
}

export const ParticlePicker: React.FC<ParticlePickerProps> = ({
  value,
  intensity,
  onChange,
  onIntensityChange,
}) => {
  const [hoveredType, setHoveredType] = useState<string | null>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const particlesRef = useRef<Particle[]>([]);
  const animationRef = useRef<number>(0);

  const activeType = hoveredType || value;
  const particleType = PARTICLE_TYPES.find((t) => t.id === activeType);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !particleType) return;

    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const width = canvas.width;
    const height = canvas.height;
    const intensityMultiplier = (intensity ?? 100) / 100;
    const particleCount = Math.floor(particleType.count * intensityMultiplier);

    // Initialize particles
    particlesRef.current = Array.from({ length: particleCount }, () =>
      createParticle(width, height, particleType)
    );
    // Spread initial positions
    particlesRef.current.forEach((p) => {
      p.y = Math.random() * height;
    });

    const animate = () => {
      ctx.clearRect(0, 0, width, height);

      particlesRef.current.forEach((particle) => {
        // Update position
        particle.y += particle.speed;
        particle.rotation += particle.rotationSpeed;

        // Add horizontal drift for some particle types
        if (particleType.shape === "petal" || particleType.shape === "leaf") {
          particle.x += Math.sin(particle.y * 0.02) * 0.5;
        }

        // Reset particle when it goes off screen
        if (particle.y > height + particle.size) {
          particle.y = -particle.size;
          particle.x = Math.random() * width;
        }

        drawParticle(ctx, particle, particleType);
      });

      animationRef.current = requestAnimationFrame(animate);
    };

    animate();

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [particleType, intensity]);

  return (
    <div className="particle-picker">
      <label>Particles</label>

      {/* Preview Canvas */}
      <div className="particle-preview-container">
        {activeType ? (
          <canvas
            ref={canvasRef}
            width={280}
            height={100}
            className="particle-canvas"
          />
        ) : (
          <div className="particle-preview-placeholder">
            Hover to preview
          </div>
        )}
      </div>

      {/* Type Selector */}
      <div className="particle-type-grid">
        <button
          type="button"
          className={`particle-type-button ${!value ? "selected" : ""}`}
          onClick={() => onChange(undefined)}
        >
          <span className="particle-icon">-</span>
          <span className="particle-label">None</span>
        </button>
        {PARTICLE_TYPES.map((type) => (
          <button
            key={type.id}
            type="button"
            className={`particle-type-button ${value === type.id ? "selected" : ""}`}
            onClick={() => onChange(type.id)}
            onMouseEnter={() => setHoveredType(type.id)}
            onMouseLeave={() => setHoveredType(null)}
          >
            <span
              className="particle-icon"
              style={{ color: type.color }}
            >
              {type.shape === "circle" && "●"}
              {type.shape === "line" && "│"}
              {type.shape === "petal" && "❀"}
              {type.shape === "star" && "★"}
              {type.shape === "leaf" && "🍃"}
            </span>
            <span className="particle-label">{type.label}</span>
          </button>
        ))}
      </div>

      {/* Intensity Slider */}
      {value && (
        <div className="particle-intensity">
          <label>Intensity</label>
          <div className="intensity-control">
            <input
              type="range"
              min="25"
              max="200"
              step="25"
              value={intensity ?? 100}
              onChange={(e) => {
                const val = parseInt(e.target.value, 10);
                onIntensityChange(val === 100 ? undefined : val);
              }}
            />
            <span className="intensity-value">{intensity ?? 100}%</span>
          </div>
        </div>
      )}
    </div>
  );
};
