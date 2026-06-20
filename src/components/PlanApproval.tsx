import { useState } from "react";
import { FileText, CheckCircle, XCircle, Edit3, Clock, Package, Play } from "lucide-react";
import { DevelopmentPlan } from "../types";

interface Props {
  plan: DevelopmentPlan;
  onAction: (
    action: "confirm" | "reject" | "modify",
    modifiedPlan?: DevelopmentPlan,
    feedback?: string
  ) => void;
}

export default function PlanApproval({ plan, onAction }: Props) {
  const [mode, setMode] = useState<"view" | "edit">("view");
  const [editJson, setEditJson] = useState(JSON.stringify(plan, null, 2));
  const [feedback, setFeedback] = useState("");

  function handleConfirm() {
    if (mode === "edit") {
      try {
        const modified = JSON.parse(editJson) as DevelopmentPlan;
        onAction("modify", modified);
      } catch (e) {
        alert("JSON 格式错误：" + String(e));
      }
    } else {
      onAction("confirm", plan);
    }
  }

  function handleReject() {
    onAction("reject", undefined, feedback);
  }

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 animate-fade-in">
      <div className="bg-white dark:bg-gray-900 rounded-2xl shadow-2xl w-full max-w-3xl p-6 max-h-[90vh] overflow-y-auto border border-gray-200 dark:border-gray-800 animate-slide-up">
        <div className="flex items-center gap-3 mb-5">
          <div className="w-10 h-10 rounded-xl bg-primary-50 dark:bg-primary-900/30 flex items-center justify-center">
            <FileText size={22} className="text-primary-600 dark:text-primary-400" />
          </div>
          <div>
            <h2 className="text-xl font-bold text-gray-900 dark:text-white">开发计划确认</h2>
            <p className="text-sm text-gray-500 dark:text-gray-400">请查看 Agent 制定的开发计划，你可以选择确认、修改或拒绝</p>
          </div>
        </div>

        {mode === "view" ? (
          <div className="space-y-5 text-sm">
            <div className="bg-primary-50 dark:bg-primary-900/20 border border-primary-100 dark:border-primary-800 rounded-xl p-4">
              <h3 className="font-semibold text-primary-900 dark:text-primary-100 mb-1">项目概述</h3>
              <p className="text-gray-700 dark:text-gray-300">{plan.summary}</p>
            </div>

            <div>
              <h3 className="font-semibold text-gray-900 dark:text-white mb-3 flex items-center gap-2">
                <Clock size={16} />
                开发阶段
              </h3>
              <div className="space-y-3">
                {plan.stages.map((s, i) => (
                  <div key={i} className="bg-gray-50 dark:bg-gray-800/50 rounded-xl p-4 border border-gray-100 dark:border-gray-800">
                    <div className="flex items-center gap-2 mb-2">
                      <span className="w-6 h-6 rounded-full bg-primary-100 dark:bg-primary-900/40 text-primary-700 dark:text-primary-300 text-xs flex items-center justify-center font-bold">
                        {i + 1}
                      </span>
                      <span className="font-medium text-gray-900 dark:text-white">{s.name}</span>
                    </div>
                    <p className="text-gray-600 dark:text-gray-400 mb-2 ml-8">{s.description}</p>
                    <ul className="list-disc list-inside text-gray-600 dark:text-gray-400 ml-8 space-y-0.5">
                      {s.tasks.map((t, j) => (
                        <li key={j}>{t}</li>
                      ))}
                    </ul>
                  </div>
                ))}
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div className="bg-gray-50 dark:bg-gray-800/50 rounded-xl p-4 border border-gray-100 dark:border-gray-800">
                <h4 className="font-medium text-gray-900 dark:text-white mb-2 flex items-center gap-2">
                  <FileText size={14} />
                  预计文件
                </h4>
                <p className="text-gray-600 dark:text-gray-400 text-xs leading-relaxed">
                  {plan.estimated_files.join(", ")}
                </p>
              </div>
              <div className="bg-gray-50 dark:bg-gray-800/50 rounded-xl p-4 border border-gray-100 dark:border-gray-800">
                <h4 className="font-medium text-gray-900 dark:text-white mb-2 flex items-center gap-2">
                  <Package size={14} />
                  依赖
                </h4>
                <p className="text-gray-600 dark:text-gray-400 text-xs leading-relaxed">
                  {plan.dependencies.join(", ")}
                </p>
              </div>
              <div className="bg-gray-50 dark:bg-gray-800/50 rounded-xl p-4 border border-gray-100 dark:border-gray-800">
                <h4 className="font-medium text-gray-900 dark:text-white mb-2 flex items-center gap-2">
                  <Play size={14} />
                  运行命令
                </h4>
                <p className="text-gray-600 dark:text-gray-400 text-xs leading-relaxed">
                  {plan.run_commands.join(", ")}
                </p>
              </div>
            </div>
          </div>
        ) : (
          <div className="space-y-2">
            <p className="text-sm text-gray-500 dark:text-gray-400">
              直接在下方编辑 JSON 格式的开发计划：
            </p>
            <textarea
              value={editJson}
              onChange={(e) => setEditJson(e.target.value)}
              className="w-full h-96 font-mono text-xs bg-gray-50 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-xl p-4 text-gray-900 dark:text-white focus:border-primary-500 transition-colors"
            />
          </div>
        )}

        <div className="mt-5">
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">
            {mode === "view" ? "修改意见 / 拒绝理由" : "修改说明"}
          </label>
          <input
            type="text"
            value={feedback}
            onChange={(e) => setFeedback(e.target.value)}
            className="w-full bg-gray-50 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-xl px-4 py-2.5 text-sm text-gray-900 dark:text-white placeholder-gray-400 focus:border-primary-500 transition-colors"
            placeholder={mode === "view" ? "如需修改或拒绝，请填写原因" : "请简要说明修改内容"}
          />
        </div>

        <div className="flex gap-3 mt-5">
          <button
            onClick={() => setMode(mode === "view" ? "edit" : "view")}
            className="flex items-center gap-1.5 px-4 py-2.5 border border-gray-300 dark:border-gray-700 text-gray-700 dark:text-gray-300 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
          >
            <Edit3 size={16} />
            {mode === "view" ? "修改计划" : "返回预览"}
          </button>
          <button
            onClick={handleReject}
            className="flex items-center gap-1.5 px-4 py-2.5 border border-red-300 dark:border-red-800 text-red-600 dark:text-red-400 rounded-xl hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors"
          >
            <XCircle size={16} />
            拒绝
          </button>
          <button
            onClick={handleConfirm}
            className="flex-1 flex items-center justify-center gap-1.5 px-4 py-2.5 bg-primary-600 hover:bg-primary-700 text-white rounded-xl font-medium transition-all shadow-lg shadow-primary-500/25"
          >
            <CheckCircle size={18} />
            {mode === "edit" ? "提交修改" : "确认计划"}
          </button>
        </div>
      </div>
    </div>
  );
}
