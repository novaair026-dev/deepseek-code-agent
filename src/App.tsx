import { useEffect, useState } from "react";
import { Moon, Sun, Plus, FolderOpen } from "lucide-react";
import { Project, ProjectType } from "./types";
import {
  createProject,
  getApiKeyConfigured,
  listProjects,
} from "./services/tauriCommands";
import { useTheme } from "./hooks/useTheme";
import ApiKeySetup from "./components/ApiKeySetup";
import Chat from "./components/Chat";
import ProjectList from "./components/ProjectList";

function App() {
  const { theme, setTheme } = useTheme();
  const [apiKeyConfigured, setApiKeyConfigured] = useState<boolean | null>(null);
  const [projects, setProjects] = useState<Project[]>([]);
  const [selectedProject, setSelectedProject] = useState<Project | null>(null);
  const [showCreate, setShowCreate] = useState(false);
  const [newProjectName, setNewProjectName] = useState("");
  const [newProjectDesc, setNewProjectDesc] = useState("");
  const [newProjectType, setNewProjectType] = useState<ProjectType>("desktop");

  useEffect(() => {
    checkApiKey();
  }, []);

  async function checkApiKey() {
    try {
      const configured = await getApiKeyConfigured();
      setApiKeyConfigured(configured);
      if (configured) {
        await loadProjects();
      }
    } catch (err) {
      console.error(err);
      setApiKeyConfigured(false);
    }
  }

  async function loadProjects() {
    const list = await listProjects();
    setProjects(list);
    if (list.length > 0 && !selectedProject) {
      setSelectedProject(list[0]);
    }
  }

  async function handleCreateProject(e: React.FormEvent) {
    e.preventDefault();
    if (!newProjectName.trim()) return;
    try {
      const project = await createProject(
        newProjectName.trim(),
        newProjectDesc.trim(),
        newProjectType
      );
      setProjects((prev) => [project, ...prev]);
      setSelectedProject(project);
      setShowCreate(false);
      setNewProjectName("");
      setNewProjectDesc("");
    } catch (err) {
      alert("创建项目失败：" + String(err));
    }
  }

  if (apiKeyConfigured === null) {
    return (
      <div className="h-full flex items-center justify-center bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-950 dark:to-gray-900">
        <div className="flex flex-col items-center gap-4">
          <div className="w-10 h-10 border-4 border-primary-500 border-t-transparent rounded-full animate-spin" />
          <p className="text-gray-500 dark:text-gray-400">正在初始化...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex h-full bg-gradient-to-br from-gray-50 to-gray-100 dark:from-gray-950 dark:to-gray-900 transition-colors duration-300">
      {apiKeyConfigured === false && (
        <ApiKeySetup onConfigured={() => checkApiKey()} />
      )}

      <ProjectList
        projects={projects}
        selectedId={selectedProject?.id}
        onSelect={setSelectedProject}
        onCreate={() => setShowCreate(true)}
      />

      <div className="flex-1 flex flex-col min-w-0 p-4">
        <div className="flex items-center justify-between mb-4 px-2">
          <div>
            <h1 className="text-2xl font-bold bg-gradient-to-r from-primary-600 to-purple-600 bg-clip-text text-transparent">
              DeepSeek Code Agent
            </h1>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              让每个人都能开发自己的应用
            </p>
          </div>
          <button
            onClick={() => setTheme(theme === "dark" ? "light" : "dark")}
            className="p-2 rounded-xl bg-white dark:bg-gray-800 shadow-sm border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
            title="切换主题"
          >
            {theme === "dark" ? <Sun size={20} /> : <Moon size={20} />}
          </button>
        </div>

        <div className="flex-1 rounded-2xl overflow-hidden shadow-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 transition-colors duration-300">
          {selectedProject ? (
            <Chat key={selectedProject.id} project={selectedProject} />
          ) : (
            <div className="h-full flex flex-col items-center justify-center text-gray-400 dark:text-gray-500 p-8">
              <div className="w-20 h-20 rounded-2xl bg-primary-50 dark:bg-primary-900/20 flex items-center justify-center mb-6">
                <FolderOpen size={40} className="text-primary-500" />
              </div>
              <h2 className="text-xl font-semibold text-gray-700 dark:text-gray-200 mb-2">
                还没有选择项目
              </h2>
              <p className="text-center mb-6 max-w-sm">
                从左侧选择一个项目开始，或者创建一个新项目，用自然语言描述你想要开发的应用。
              </p>
              <button
                onClick={() => setShowCreate(true)}
                className="flex items-center gap-2 bg-primary-600 hover:bg-primary-700 text-white px-6 py-3 rounded-xl font-medium transition-all shadow-lg shadow-primary-500/25 hover:shadow-primary-500/40"
              >
                <Plus size={20} />
                新建项目
              </button>
            </div>
          )}
        </div>
      </div>

      {showCreate && (
        <div className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 animate-fade-in">
          <div className="bg-white dark:bg-gray-900 rounded-2xl shadow-2xl w-full max-w-md p-6 border border-gray-200 dark:border-gray-800 animate-slide-up">
            <h2 className="text-xl font-bold mb-1 text-gray-900 dark:text-white">新建项目</h2>
            <p className="text-sm text-gray-500 dark:text-gray-400 mb-5">
              描述你的想法，Agent 会帮你把它变成现实。
            </p>
            <form onSubmit={handleCreateProject} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  项目名称
                </label>
                <input
                  value={newProjectName}
                  onChange={(e) => setNewProjectName(e.target.value)}
                  className="w-full bg-gray-50 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-xl px-4 py-2.5 text-gray-900 dark:text-white placeholder-gray-400 focus:border-primary-500 transition-colors"
                  placeholder="例如：个人博客网站"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  项目描述
                </label>
                <input
                  value={newProjectDesc}
                  onChange={(e) => setNewProjectDesc(e.target.value)}
                  className="w-full bg-gray-50 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-xl px-4 py-2.5 text-gray-900 dark:text-white placeholder-gray-400 focus:border-primary-500 transition-colors"
                  placeholder="简单描述项目用途"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                  项目类型
                </label>
                <select
                  value={newProjectType}
                  onChange={(e) => setNewProjectType(e.target.value as ProjectType)}
                  className="w-full bg-gray-50 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-xl px-4 py-2.5 text-gray-900 dark:text-white focus:border-primary-500 transition-colors"
                >
                  <option value="desktop">桌面应用（Rust + Tauri）</option>
                  <option value="website">网站（Rust + Tauri + Axum + Vue3）</option>
                </select>
              </div>
              <div className="flex gap-3 pt-2">
                <button
                  type="button"
                  onClick={() => setShowCreate(false)}
                  className="flex-1 border border-gray-300 dark:border-gray-700 text-gray-700 dark:text-gray-300 py-2.5 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
                >
                  取消
                </button>
                <button
                  type="submit"
                  className="flex-1 bg-primary-600 hover:bg-primary-700 text-white py-2.5 rounded-xl font-medium transition-all shadow-lg shadow-primary-500/25"
                >
                  创建
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
