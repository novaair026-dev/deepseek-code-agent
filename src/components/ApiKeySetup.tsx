import { useState } from "react";
import { KeyRound, Eye, EyeOff, Shield } from "lucide-react";
import { setApiKey } from "../services/tauriCommands";

interface Props {
  onConfigured: () => void;
}

export default function ApiKeySetup({ onConfigured }: Props) {
  const [apiKey, setApiKeyLocal] = useState("");
  const [showKey, setShowKey] = useState(false);
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
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center z-50 animate-fade-in">
      <div className="bg-white dark:bg-gray-900 rounded-2xl shadow-2xl w-full max-w-md p-8 border border-gray-200 dark:border-gray-800 animate-slide-up">
        <div className="w-16 h-16 rounded-2xl bg-gradient-to-br from-primary-500 to-purple-600 flex items-center justify-center mx-auto mb-5 shadow-lg shadow-primary-500/25">
          <KeyRound size={32} className="text-white" />
        </div>

        <h2 className="text-2xl font-bold text-center text-gray-900 dark:text-white mb-2">
          配置 DeepSeek API Key
        </h2>
        <p className="text-gray-500 dark:text-gray-400 text-center text-sm mb-6">
          首次使用需要配置你的 DeepSeek API Key。密钥将安全存储在系统钥匙串中。
        </p>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="relative">
            <input
              type={showKey ? "text" : "password"}
              value={apiKey}
              onChange={(e) => setApiKeyLocal(e.target.value)}
              placeholder="sk-xxxxxxxxxxxxxxxxxxxxxxxx"
              className="w-full bg-gray-50 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-xl pl-4 pr-12 py-3 text-gray-900 dark:text-white placeholder-gray-400 focus:border-primary-500 transition-colors"
            />
            <button
              type="button"
              onClick={() => setShowKey(!showKey)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300"
            >
              {showKey ? <EyeOff size={18} /> : <Eye size={18} />}
            </button>
          </div>

          {error && (
            <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-xl px-4 py-3 text-red-600 dark:text-red-400 text-sm">
              {error}
            </div>
          )}

          <div className="flex items-start gap-2 text-xs text-gray-500 dark:text-gray-400 bg-gray-50 dark:bg-gray-800/50 rounded-xl p-3">
            <Shield size={16} className="shrink-0 mt-0.5" />
            <p>你的 API Key 只会保存在本机系统钥匙串中，不会上传到任何服务器或在项目中以明文存储。</p>
          </div>

          <button
            type="submit"
            disabled={loading}
            className="w-full bg-primary-600 hover:bg-primary-700 disabled:opacity-50 text-white py-3 rounded-xl font-medium transition-all shadow-lg shadow-primary-500/25 hover:shadow-primary-500/40"
          >
            {loading ? "保存中..." : "保存并开始使用"}
          </button>
        </form>
      </div>
    </div>
  );
}
