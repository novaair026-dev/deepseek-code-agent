import { useState } from "react";

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
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white rounded-xl shadow-xl w-full max-w-lg p-6 max-h-[80vh] overflow-y-auto">
        <h2 className="text-xl font-bold mb-4">需要补充一些细节</h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          {questions.map((q, idx) => (
            <div key={idx}>
              <label className="block text-sm font-medium text-gray-700 mb-1">{q}</label>
              <input
                type="text"
                value={answers[idx] || ""}
                onChange={(e) => setAnswers({ ...answers, [idx]: e.target.value })}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="请输入你的回答"
              />
            </div>
          ))}
          <div className="flex gap-2 pt-2">
            <button
              type="button"
              onClick={onCancel}
              className="flex-1 border border-gray-300 py-2 rounded-lg hover:bg-gray-50"
            >
              取消
            </button>
            <button
              type="submit"
              className="flex-1 bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700"
            >
              提交
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
