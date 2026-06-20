import { useState } from "react";
import { HelpCircle, MessageCircle, X } from "lucide-react";

interface Props {
  questions: string[];
  onSubmit: (answers: Record<string, string>) => void;
  onCancel: () => void;
}

export default function Clarification({ questions, onSubmit, onCancel }: Props) {
  const [answers, setAnswers] = useState<Record<string, string>>({});

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    onSubmit(answers);
  }

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 animate-fade-in">
      <div className="bg-white dark:bg-gray-900 rounded-2xl shadow-2xl w-full max-w-xl p-6 max-h-[85vh] overflow-y-auto border border-gray-200 dark:border-gray-800 animate-slide-up">
        <div className="flex items-center justify-between mb-5">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-primary-50 dark:bg-primary-900/30 flex items-center justify-center">
              <HelpCircle size={22} className="text-primary-600 dark:text-primary-400" />
            </div>
            <div>
              <h2 className="text-xl font-bold text-gray-900 dark:text-white">需要补充一些细节</h2>
              <p className="text-sm text-gray-500 dark:text-gray-400">回答以下问题，帮助 Agent 更好地理解你的需求</p>
            </div>
          </div>
          <button
            onClick={onCancel}
            className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          >
            <X size={20} />
          </button>
        </div>

        <form onSubmit={handleSubmit} className="space-y-4">
          {questions.map((q, idx) => (
            <div key={idx} className="bg-gray-50 dark:bg-gray-800/50 rounded-xl p-4 border border-gray-100 dark:border-gray-800">
              <label className="flex items-start gap-2 text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                <MessageCircle size={16} className="shrink-0 mt-0.5 text-primary-500" />
                {q}
              </label>
              <input
                type="text"
                value={answers[idx] || ""}
                onChange={(e) => setAnswers({ ...answers, [idx]: e.target.value })}
                className="w-full bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-lg px-3 py-2.5 text-gray-900 dark:text-white placeholder-gray-400 focus:border-primary-500 transition-colors"
                placeholder="请输入你的回答"
              />
            </div>
          ))}
          <div className="flex gap-3 pt-2">
            <button
              type="button"
              onClick={onCancel}
              className="flex-1 border border-gray-300 dark:border-gray-700 text-gray-700 dark:text-gray-300 py-2.5 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
            >
              取消
            </button>
            <button
              type="submit"
              className="flex-1 bg-primary-600 hover:bg-primary-700 text-white py-2.5 rounded-xl font-medium transition-all shadow-lg shadow-primary-500/25"
            >
              提交回答
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
