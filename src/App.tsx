import { useEffect, useState } from "react";
import { Project, ProjectType } from "./types";
import {
  createProject,
  getApiKeyConfigured,
  listProjects,
} from "./services/tauriCommands";
import ApiKeySetup from "./components/ApiKeySetup";
import Chat from "./components/Chat";
import ProjectList from "./components/ProjectList";

function App() {
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
      <div className="h-full flex items-center justify-center text-gray-500">
        加载中...
      </div>
    );
  }

  return (
    <div className="flex h-full">
      {apiKeyConfigured === false && (
        <ApiKeySetup onConfigured={() => checkApiKey()} />
      )}

      <ProjectList
        projects={projects}
        selectedId={selectedProject?.id}
        onSelect={setSelectedProject}
        onCreate={() => setShowCreate(true)}
      />

      <div className="flex-1 flex flex-col min-w-0">
        {selectedProject ? (
          <Chat key={selectedProject.id} project={selectedProject} />
        ) : (
          <div className="flex-1 flex flex-col items-center justify-center text-gray-400">
            <p className="mb-4">选择一个项目或创建新项目</p>
            <button
              onClick={() => setShowCreate(true)}
              className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700"
            >
              新建项目
            </button>
          </div>
        )}
      </div>

      {showCreate && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-white rounded-xl shadow-xl w-full max-w-md p-6">
            <h2 className="text-xl font-bold mb-4">新建项目</h2>
            <form onSubmit={handleCreateProject} className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  项目名称
                </label>
                <input
                  value={newProjectName}
                  onChange={(e) => setNewProjectName(e.target.value)}
                  className="w-full border border-gray-300 rounded-lg px-3 py-2"
                  placeholder="例如：个人博客网站"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  项目描述
                </label>
                <input
                  value={newProjectDesc}
                  onChange={(e) => setNewProjectDesc(e.target.value)}
                  className="w-full border border-gray-300 rounded-lg px-3 py-2"
                  placeholder="简单描述项目用途"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  项目类型
                </label>
                <select
                  value={newProjectType}
                  onChange={(e) => setNewProjectType(e.target.value as ProjectType)}
                  className="w-full border border-gray-300 rounded-lg px-3 py-2"
                >
                  <option value="desktop">桌面应用（Rust + Tauri）</option>
                  <option value="website">网站（Rust + Tauri + Axum）</option>
                </select>
              </div>
              <div className="flex gap-2 pt-2">
                <button
                  type="button"
                  onClick={() => setShowCreate(false)}
                  className="flex-1 border border-gray-300 py-2 rounded-lg hover:bg-gray-50"
                >
                  取消
                </button>
                <button
                  type="submit"
                  className="flex-1 bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700"
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
