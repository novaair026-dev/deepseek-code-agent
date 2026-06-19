interface Props {
  command: string;
  description: string;
  onConfirm: () => void;
  onCancel: () => void;
}

export default function CommandConfirmation({ command, description, onConfirm, onCancel }: Props) {
  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white rounded-xl shadow-xl w-full max-w-lg p-6">
        <h2 className="text-xl font-bold mb-2 text-red-600">高危命令确认</h2>
        <p className="text-gray-600 text-sm mb-4">{description}</p>
        <div className="bg-gray-100 rounded-lg p-3 font-mono text-sm mb-6 overflow-x-auto">
          {command}
        </div>
        <div className="flex gap-2">
          <button
            onClick={onCancel}
            className="flex-1 border border-gray-300 py-2 rounded-lg hover:bg-gray-50"
          >
            取消
          </button>
          <button
            onClick={onConfirm}
            className="flex-1 bg-red-600 text-white py-2 rounded-lg hover:bg-red-700"
          >
            确认执行
          </button>
        </div>
      </div>
    </div>
  );
}
