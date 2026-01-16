import { useTranslation } from "react-i18next";
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
  const { t } = useTranslation();

  const formatDate = (timestamp: number) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const days = Math.floor(diff / (1000 * 60 * 60 * 24));

    if (days === 0) {
      return t("welcome.today");
    }
    if (days === 1) {
      return t("welcome.yesterday");
    }
    if (days < 7) {
      return t("welcome.daysAgo", { count: days });
    }
    return date.toLocaleDateString();
  };

  return (
    <div className="welcome-screen">
      <div className="welcome-content">
        <div className="welcome-header">
          <h1>{t("welcome.title")}</h1>
          <p className="welcome-subtitle">{t("welcome.subtitle")}</p>
        </div>

        <div className="welcome-actions">
          <button className="welcome-action primary" onClick={onNewProject}>
            <span className="action-icon">+</span>
            <span className="action-text">
              <span className="action-title">{t("welcome.newProject")}</span>
              <span className="action-description">
                {t("welcome.newProjectDescription")}
              </span>
            </span>
          </button>

          <button className="welcome-action" onClick={onOpenProject}>
            <span className="action-icon">📁</span>
            <span className="action-text">
              <span className="action-title">{t("welcome.openProject")}</span>
              <span className="action-description">
                {t("welcome.openProjectDescription")}
              </span>
            </span>
          </button>
        </div>

        {recentProjects.length > 0 && (
          <div className="recent-projects">
            <h2>{t("welcome.recentProjects")}</h2>
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
                      title={t("welcome.removeFromRecent")}
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
          <span>{t("common.or")}</span>
        </div>

        <button className="open-file-link" onClick={onOpenFile}>
          {t("welcome.openSingleFile")}
        </button>
      </div>
    </div>
  );
};
