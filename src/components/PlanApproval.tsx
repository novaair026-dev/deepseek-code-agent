import { useState } from "react";
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
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white rounded-xl shadow-xl w-full max-w-2xl p-6 max-h-[85vh] overflow-y-auto">
        <h2 className="text-xl font-bold mb-4">开发计划确认</h2>

        {mode === "view" ? (
          <div className="space-y-4 text-sm">
            <p className="text-gray-700">{plan.summary}</p>
            <div>
              <h3 className="font-semibold">阶段</h3>
              {plan.stages.map((s, i) => (
                <div key={i} className="mt-2 pl-3 border-l-2 border-blue-200">
                  <div className="font-medium">{s.name}</div>
                  <div className="text-gray-600">{s.description}</div>
                  <ul className="list-disc list-inside text-gray-600">
                    {s.tasks.map((t, j) => (
                      <li key={j}>{t}</li>
                    ))}
                  </ul>
                </div>
              ))}
            </div>
            <div>
              <h3 className="font-semibold">预计文件</h3>
              <p className="text-gray-600">{plan.estimated_files.join(", ")}</p>
            </div>
            <div>
              <h3 className="font-semibold">依赖</h3>
              <p className="text-gray-600">{plan.dependencies.join(", ")}</p>
            </div>
            <div>
              <h3 className="font-semibold">运行命令</h3>
              <p className="text-gray-600">{plan.run_commands.join(", ")}</p>
            </div>
          </div>
        ) : (
          <textarea
            value={editJson}
            onChange={(e) => setEditJson(e.target.value)}
            className="w-full h-96 font-mono text-xs border border-gray-300 rounded-lg p-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        )}

        <div className="mt-4">
          <label className="block text-sm font-medium text-gray-700 mb-1">修改意见 / 拒绝理由</label>
          <input
            type="text"
            value={feedback}
            onChange={(e) => setFeedback(e.target.value)}
            className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
            placeholder="如需修改或拒绝，请填写原因"
          />
        </div>

        <div className="flex gap-2 mt-4">
          <button
            onClick={() => setMode(mode === "view" ? "edit" : "view")}
            className="px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50"
          >
            {mode === "view" ? "修改计划" : "返回预览"}
          </button>
          <button
            onClick={handleReject}
            className="px-4 py-2 border border-red-300 text-red-600 rounded-lg hover:bg-red-50"
          >
            拒绝
          </button>
          <button
            onClick={handleConfirm}
            className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
          >
            {mode === "edit" ? "提交修改" : "确认计划"}
          </button>
        </div>
      </div>
    </div>
  );
}
