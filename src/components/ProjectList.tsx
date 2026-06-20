import { Monitor, Globe, Plus } from "lucide-react";
import { Project } from "../types";

interface Props {
  projects: Project[];
  selectedId?: string;
  onSelect: (p: Project) => void;
  onCreate: () => void;
}

export default function ProjectList({ projects, selectedId, onSelect, onCreate }: Props) {
  return (
    <div className="w-72 bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border-r border-gray-200 dark:border-gray-800 flex flex-col h-full transition-colors duration-300">
      <div className="p-5 border-b border-gray-200 dark:border-gray-800">
        <h1 className="font-bold text-xl bg-gradient-to-r from-primary-600 to-purple-600 bg-clip-text text-transparent">
          DS Code Agent
        </h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-0.5">AI 编程助手</p>
      </div>

      <div className="flex-1 overflow-y-auto p-3 space-y-1">
        {projects.map((p) => (
          <button
            key={p.id}
            onClick={() => onSelect(p)}
            className={`w-full text-left px-3 py-3 rounded-xl text-sm transition-all duration-200 group ${
              p.id === selectedId
                ? "bg-primary-50 dark:bg-primary-900/30 border border-primary-200 dark:border-primary-800 shadow-sm"
                : "hover:bg-gray-100 dark:hover:bg-gray-800 border border-transparent"
            }`}
          >
            <div className="flex items-center gap-3">
              <div
                className={`w-9 h-9 rounded-lg flex items-center justify-center shrink-0 ${
                  p.id === selectedId
                    ? "bg-primary-100 dark:bg-primary-800 text-primary-600 dark:text-primary-300"
                    : "bg-gray-100 dark:bg-gray-800 text-gray-500 dark:text-gray-400 group-hover:bg-white dark:group-hover:bg-gray-700"
                }`}
              >
                {p.type === "desktop" ? <Monitor size={18} /> : <Globe size={18} />}
              </div>
              <div className="min-w-0">
                <div
                  className={`font-medium truncate ${
                    p.id === selectedId
                      ? "text-primary-900 dark:text-primary-100"
                      : "text-gray-900 dark:text-gray-100"
                  }`}
                >
                  {p.name}
                </div>
                <div className="text-xs text-gray-500 dark:text-gray-400 truncate">
                  {p.type === "desktop" ? "桌面应用" : "网站"}
                </div>
              </div>
            </div>
          </button>
        ))}
        {projects.length === 0 && (
          <div className="text-center py-8 px-4">
            <div className="w-12 h-12 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center mx-auto mb-3">
              <Plus size={20} className="text-gray-400" />
            </div>
            <p className="text-gray-500 dark:text-gray-400 text-sm">点击下方按钮创建第一个项目</p>
          </div>
        )}
      </div>

      <div className="p-4 border-t border-gray-200 dark:border-gray-800">
        <button
          onClick={onCreate}
          className="w-full flex items-center justify-center gap-2 bg-primary-600 hover:bg-primary-700 text-white py-2.5 rounded-xl text-sm font-medium transition-all shadow-lg shadow-primary-500/20 hover:shadow-primary-500/30"
        >
          <Plus size={18} />
          新建项目
        </button>
      </div>
    </div>
  );
}
