import { useEffect, useRef, useState } from "react";
import { Send, Bot, User, Terminal, CheckCircle, AlertCircle, Loader2 } from "lucide-react";
import { AgentResponse, Project, ToolResult } from "../types";
import {
  approvePlan,
  confirmCommand,
  sendMessage,
} from "../services/tauriCommands";
import Clarification from "./Clarification";
import PlanApproval from "./PlanApproval";
import CommandConfirmation from "./CommandConfirmation";

interface ChatMessage {
  id: string;
  role: "user" | "assistant" | "tool";
  content: string;
  toolResult?: ToolResult;
}

interface Props {
  project: Project;
}

export default function Chat({ project }: Props) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const [sessionId, setSessionId] = useState<string | undefined>(undefined);
  const [pendingResponse, setPendingResponse] = useState<AgentResponse | null>(null);
  const [pendingCommand, setPendingCommand] = useState<{ command: string; description: string } | null>(null);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  function addMessage(msg: ChatMessage) {
    setMessages((prev) => [...prev, msg]);
  }

  async function handleSend(content: string, isClarification = false) {
    if (!content.trim()) return;
    setInput("");
    setLoading(true);
    setPendingResponse(null);

    if (!isClarification) {
      addMessage({ id: Date.now().toString(), role: "user", content });
    }

    try {
      const { session_id, response } = await sendMessage(
        project.id,
        content,
        sessionId
      );
      setSessionId(session_id);
      handleAgentResponse(response);
    } catch (err) {
      addMessage({
        id: Date.now().toString(),
        role: "assistant",
        content: "出错了：" + String(err),
      });
    } finally {
      setLoading(false);
    }
  }

  function handleAgentResponse(response: AgentResponse) {
    switch (response.type) {
      case "message":
      case "done":
      case "error":
        addMessage({
          id: Date.now().toString(),
          role: "assistant",
          content:
            response.type === "error"
              ? response.message
              : response.type === "done"
              ? response.summary
              : response.content,
        });
        break;
      case "clarification":
      case "plan":
        setPendingResponse(response);
        break;
      case "progress":
        addMessage({
          id: Date.now().toString(),
          role: "tool",
          content: `${response.step}\n${response.detail}`,
          toolResult: response.tool_result,
        });
        break;
      case "command_confirmation":
        setPendingCommand({
          command: response.command,
          description: response.description,
        });
        break;
    }
  }

  async function handleClarificationSubmit(answers: Record<string, string>) {
    if (!pendingResponse || pendingResponse.type !== "clarification") return;
    const questions = pendingResponse.questions;
    const combined = questions
      .map((q, i) => `Q: ${q}\nA: ${answers[i] || "未回答"}`)
      .join("\n\n");
    setPendingResponse(null);
    await handleSend(combined, true);
  }

  async function handlePlanAction(
    action: "confirm" | "reject" | "modify",
    plan?: any,
    feedback?: string
  ) {
    if (!pendingResponse || pendingResponse.type !== "plan") return;
    setLoading(true);
    setPendingResponse(null);
    try {
      const response = await approvePlan(
        project.id,
        action,
        plan,
        feedback
      );
      handleAgentResponse(response);
    } catch (err) {
      addMessage({
        id: Date.now().toString(),
        role: "assistant",
        content: "计划处理失败：" + String(err),
      });
    } finally {
      setLoading(false);
    }
  }

  async function handleCommandConfirm() {
    if (!pendingCommand || !sessionId) return;
    setLoading(true);
    try {
      const response = await confirmCommand(
        project.id,
        sessionId,
        pendingCommand.command
      );
      setPendingCommand(null);
      handleAgentResponse(response);
    } catch (err) {
      addMessage({
        id: Date.now().toString(),
        role: "assistant",
        content: "命令执行失败：" + String(err),
      });
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="flex flex-col h-full bg-white dark:bg-gray-900 transition-colors duration-300">
      <div className="border-b border-gray-200 dark:border-gray-800 px-5 py-4 flex items-center justify-between bg-gray-50/50 dark:bg-gray-900/50 backdrop-blur">
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-primary-500 to-purple-600 flex items-center justify-center text-white font-bold shadow-lg shadow-primary-500/25">
            {project.name.charAt(0).toUpperCase()}
          </div>
          <div>
            <h2 className="font-bold text-gray-900 dark:text-white">{project.name}</h2>
            <p className="text-xs text-gray-500 dark:text-gray-400 font-mono truncate max-w-md">
              {project.path}
            </p>
          </div>
        </div>
        {project.plan_status === "confirmed" && (
          <span className="flex items-center gap-1.5 text-xs bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 px-3 py-1.5 rounded-full font-medium border border-green-200 dark:border-green-800">
            <CheckCircle size={14} />
            计划已确认
          </span>
        )}
      </div>

      <div className="flex-1 overflow-y-auto p-5 space-y-5">
        {messages.length === 0 && (
          <div className="h-full flex flex-col items-center justify-center text-center px-8">
            <div className="w-16 h-16 rounded-2xl bg-primary-50 dark:bg-primary-900/20 flex items-center justify-center mb-4">
              <Bot size={32} className="text-primary-500" />
            </div>
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-2">
              开始描述你的需求
            </h3>
            <p className="text-gray-500 dark:text-gray-400 text-sm max-w-md leading-relaxed">
              例如："帮我做一个待办事项管理工具，可以添加任务、标记完成、按状态筛选。" Agent 会帮你澄清细节并制定开发计划。
            </p>
          </div>
        )}

        {messages.map((m) => (
          <div
            key={m.id}
            className={`flex ${
              m.role === "user" ? "justify-end" : "justify-start"
            } animate-slide-up`}
          >
            <div className={`flex gap-3 max-w-[85%] ${m.role === "user" ? "flex-row-reverse" : ""}`}>
              <div
                className={`w-8 h-8 rounded-full flex items-center justify-center shrink-0 ${
                  m.role === "user"
                    ? "bg-primary-600 text-white"
                    : m.role === "tool"
                    ? "bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300"
                    : "bg-gradient-to-br from-purple-500 to-primary-600 text-white"
                }`}
              >
                {m.role === "user" ? <User size={16} /> : m.role === "tool" ? <Terminal size={16} /> : <Bot size={16} />}
              </div>
              <div
                className={`rounded-2xl px-4 py-3 text-sm whitespace-pre-wrap shadow-sm ${
                  m.role === "user"
                    ? "bg-primary-600 text-white rounded-br-md"
                    : m.role === "tool"
                    ? "bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200 border border-gray-200 dark:border-gray-700 rounded-bl-md"
                    : "bg-white dark:bg-gray-800 text-gray-800 dark:text-gray-200 border border-gray-200 dark:border-gray-700 rounded-bl-md"
                }`}
              >
                {m.content}
                {m.toolResult && !m.toolResult.success && (
                  <div className="flex items-start gap-1.5 mt-2 text-red-500 dark:text-red-400 text-xs">
                    <AlertCircle size={14} className="shrink-0 mt-0.5" />
                    {m.toolResult.error}
                  </div>
                )}
              </div>
            </div>
          </div>
        ))}

        {loading && (
          <div className="flex items-center gap-2 text-gray-500 dark:text-gray-400 text-sm animate-fade-in">
            <Loader2 size={16} className="animate-spin" />
            Agent 正在思考...
          </div>
        )}
        <div ref={bottomRef} />
      </div>

      <div className="border-t border-gray-200 dark:border-gray-800 p-4 bg-gray-50/80 dark:bg-gray-900/80 backdrop-blur">
        <div className="flex gap-3 items-end bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-2xl p-2 shadow-sm">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault();
                handleSend(input);
              }
            }}
            disabled={loading || !!pendingResponse}
            placeholder="描述你想要开发的应用或网站..."
            rows={1}
            className="flex-1 bg-transparent px-3 py-2 text-gray-900 dark:text-white placeholder-gray-400 resize-none focus:outline-none max-h-32 disabled:opacity-50"
          />
          <button
            onClick={() => handleSend(input)}
            disabled={loading || !!pendingResponse || !input.trim()}
            className="bg-primary-600 hover:bg-primary-700 disabled:opacity-40 text-white p-2.5 rounded-xl transition-all shadow-lg shadow-primary-500/25"
          >
            <Send size={18} />
          </button>
        </div>
        <p className="text-xs text-gray-400 dark:text-gray-500 mt-2 text-center">
          按 Enter 发送，Shift + Enter 换行
        </p>
      </div>

      {pendingResponse?.type === "clarification" && (
        <Clarification
          questions={pendingResponse.questions}
          onSubmit={handleClarificationSubmit}
          onCancel={() => setPendingResponse(null)}
        />
      )}

      {pendingResponse?.type === "plan" && (
        <PlanApproval
          plan={pendingResponse.plan}
          onAction={handlePlanAction}
        />
      )}

      {pendingCommand && (
        <CommandConfirmation
          command={pendingCommand.command}
          description={pendingCommand.description}
          onConfirm={handleCommandConfirm}
          onCancel={() => setPendingCommand(null)}
        />
      )}
    </div>
  );
}
