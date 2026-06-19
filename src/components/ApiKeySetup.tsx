import { useState } from "react";
import { setApiKey } from "../services/tauriCommands";

interface Props {
  onConfigured: () => void;
}

export default function ApiKeySetup({ onConfigured }: Props) {
  const [apiKey, setApiKeyLocal] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError("");
    if (!apiKey.trim()) {
      setError("请输入 API Key");
      return;
    }
    setLoading(true);
    try {
      await setApiKey(apiKey.trim());
      onConfigured();
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-white rounded-xl shadow-xl w-full max-w-md p-6">
        <h2 className="text-xl font-bold mb-2">配置 DeepSeek API Key</h2>
        <p className="text-gray-600 text-sm mb-4">
          首次使用需要配置你的 DeepSeek API Key。密钥将安全存储在系统钥匙串中。
        </p>
        <form onSubmit={handleSubmit} className="space-y-4">
          <input
            type="password"
            value={apiKey}
            onChange={(e) => setApiKeyLocal(e.target.value)}
            placeholder="sk-xxxxxxxxxxxxxxxxxxxxxxxx"
            className="w-full border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          {error && <p className="text-red-500 text-sm">{error}</p>}
          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-600 text-white py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50"
          >
            {loading ? "保存中..." : "保存并开始使用"}
          </button>
        </form>
      </div>
    </div>
  );
}
