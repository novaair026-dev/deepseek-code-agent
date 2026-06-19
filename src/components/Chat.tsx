import { useEffect, useRef, useState } from "react";
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
    <div className="flex flex-col h-full bg-white">
      <div className="border-b border-gray-200 px-4 py-3 flex items-center justify-between">
        <div>
          <h2 className="font-bold">{project.name}</h2>
          <p className="text-xs text-gray-500">
            {project.type === "desktop" ? "桌面应用" : "网站"} · {project.path}
          </p>
        </div>
        {project.plan_status === "confirmed" && (
          <span className="text-xs bg-green-100 text-green-700 px-2 py-1 rounded">
            计划已确认
          </span>
        )}
      </div>

      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {messages.map((m) => (
          <div
            key={m.id}
            className={`flex ${
              m.role === "user" ? "justify-end" : "justify-start"
            }`}
          >
            <div
              className={`max-w-[80%] rounded-xl px-4 py-2 text-sm whitespace-pre-wrap ${
                m.role === "user"
                  ? "bg-blue-600 text-white"
                  : m.role === "tool"
                  ? "bg-gray-100 text-gray-800 border border-gray-200"
                  : "bg-gray-50 text-gray-800 border border-gray-200"
              }`}
            >
              {m.content}
              {m.toolResult && !m.toolResult.success && (
                <p className="text-red-500 text-xs mt-1">
                  {m.toolResult.error}
                </p>
              )}
            </div>
          </div>
        ))}
        {loading && (
          <div className="text-gray-400 text-sm">Agent 正在思考...</div>
        )}
        <div ref={bottomRef} />
      </div>

      <div className="border-t border-gray-200 p-4">
        <div className="flex gap-2">
          <input
            value={input}
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && handleSend(input)}
            disabled={loading || !!pendingResponse}
            placeholder="描述你想要开发的应用或网站..."
            className="flex-1 border border-gray-300 rounded-lg px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100"
          />
          <button
            onClick={() => handleSend(input)}
            disabled={loading || !!pendingResponse || !input.trim()}
            className="bg-blue-600 text-white px-4 py-2 rounded-lg hover:bg-blue-700 disabled:opacity-50"
          >
            发送
          </button>
        </div>
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
