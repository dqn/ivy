import type { RecentProject } from "../../types/project";
import "./WelcomeScreen.css";

interface Props {
  recentProjects: RecentProject[];
  onNewProject: () => void;
  onOpenProject: () => void;
  onOpenRecentProject: (path: string) => void;
  onRemoveRecentProject: (path: string) => void;
  onOpenFile: () => void;
}

export const WelcomeScreen: React.FC<Props> = ({
  recentProjects,
  onNewProject,
  onOpenProject,
  onOpenRecentProject,
  onRemoveRecentProject,
  onOpenFile,
}) => {
  const formatDate = (timestamp: number) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));

    if (days === 0) {
      return "Today";
    }
    if (days === 1) {
      return "Yesterday";
    }
    if (days < 7) {
      return `${days} days ago`;
    }
    return date.toLocaleDateString();
  };

  return (
    <div className="welcome-screen">
      <div className="welcome-content">
        <div className="welcome-header">
          <h1>ivy Editor</h1>
          <p className="welcome-subtitle">Visual Novel Editor</p>
        </div>

        <div className="welcome-actions">
          <button className="welcome-action primary" onClick={onNewProject}>
            <span className="action-icon">+</span>
            <span className="action-text">
              <span className="action-title">New Project</span>
              <span className="action-description">
                Create a new visual novel project
              </span>
            </span>
          </button>

          <button className="welcome-action" onClick={onOpenProject}>
            <span className="action-icon">📁</span>
            <span className="action-text">
              <span className="action-title">Open Project</span>
              <span className="action-description">
                Open an existing project folder
              </span>
            </span>
          </button>
        </div>

        {recentProjects.length > 0 && (
          <div className="recent-projects">
            <h2>Recent Projects</h2>
            <div className="recent-list">
              {recentProjects.map((project) => (
                <div
                  key={project.path}
                  className="recent-item"
                  onClick={() => onOpenRecentProject(project.path)}
                >
                  <div className="recent-info">
                    <span className="recent-name">{project.name}</span>
                    <span className="recent-path">{project.path}</span>
                  </div>
                  <div className="recent-meta">
                    <span className="recent-date">
                      {formatDate(project.last_opened)}
                    </span>
                    <button
                      className="remove-recent"
                      onClick={(e) => {
                        e.stopPropagation();
                        onRemoveRecentProject(project.path);
                      }}
                      title="Remove from recent"
                    >
                      ×
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        <div className="welcome-divider">
          <span>or</span>
        </div>

        <button className="open-file-link" onClick={onOpenFile}>
          Open single scenario file (legacy mode)
        </button>
      </div>
    </div>
  );
};
