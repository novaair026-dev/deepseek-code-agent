import { AlertTriangle, Terminal, X, Play } from "lucide-react";

interface Props {
  command: string;
  description: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export default function CommandConfirmation({ command, description, onConfirm, onCancel }: Props) {
  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 animate-fade-in">
      <div className="bg-white dark:bg-gray-900 rounded-2xl shadow-2xl w-full max-w-lg p-6 border border-red-200 dark:border-red-900/30 animate-slide-up">
        <div className="flex items-center gap-3 mb-4">
          <div className="w-12 h-12 rounded-full bg-red-100 dark:bg-red-900/30 flex items-center justify-center">
            <AlertTriangle size={24} className="text-red-600 dark:text-red-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-red-600 dark:text-red-400">高危命令确认</h2>
            <p className="text-sm text-gray-500 dark:text-gray-400">执行前请确认此操作是安全的</p>
          </div>
        </div>

        <p className="text-gray-700 dark:text-gray-300 text-sm mb-4">{description}</p>

        <div className="bg-gray-900 rounded-xl p-4 mb-6 overflow-x-auto border border-gray-800">
          <div className="flex items-center gap-2 mb-2">
            <Terminal size={14} className="text-gray-500" />
            <span className="text-xs text-gray-500 uppercase tracking-wider">Command</span>
          </div>
          <code className="font-mono text-sm text-green-400 whitespace-pre">{command}</code>
        </div>

        <div className="flex gap-3">
          <button
            onClick={onCancel}
            className="flex-1 flex items-center justify-center gap-2 border border-gray-300 dark:border-gray-700 text-gray-700 dark:text-gray-300 py-2.5 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
          >
            <X size={18} />
            取消
          </button>
          <button
            onClick={onConfirm}
            className="flex-1 flex items-center justify-center gap-2 bg-red-600 hover:bg-red-700 text-white py-2.5 rounded-xl font-medium transition-all shadow-lg shadow-red-500/25"
          >
            <Play size={18} />
            确认执行
          </button>
        </div>
      </div>
    </div>
  );
}
