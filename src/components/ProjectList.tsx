import { Project } from "../types";

interface Props {
  projects: Project[];
  selectedId?: string;
  onSelect: (p: Project) => void;
  onCreate: () => void;
}

export default function ProjectList({ projects, selectedId, onSelect, onCreate }: Props) {
  return (
    <div className="w-64 bg-gray-900 text-white flex flex-col h-full">
      <div className="p-4 border-b border-gray-700 flex items-center justify-between">
        <h1 className="font-bold text-lg">DS Code Agent</h1>
      </div>
      <div className="flex-1 overflow-y-auto p-2 space-y-1">
        {projects.map((p) => (
          <button
            key={p.id}
            onClick={() => onSelect(p)}
            className={`w-full text-left px-3 py-2 rounded-lg text-sm transition ${
              p.id === selectedId ? "bg-blue-600" : "hover:bg-gray-800"
            }`}
          >
            <div className="font-medium truncate">{p.name}</div>
            <div className="text-xs text-gray-400 truncate">
              {p.type === "desktop" ? "桌面应用" : "网站"}
            </div>
          </button>
        ))}
        {projects.length === 0 && (
          <p className="text-gray-500 text-sm px-3 py-2">暂无项目</p>
        )}
      </div>
      <div className="p-3 border-t border-gray-700">
        <button
          onClick={onCreate}
          className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 rounded-lg text-sm"
        >
          + 新建项目
        </button>
      </div>
    </div>
  );
}
