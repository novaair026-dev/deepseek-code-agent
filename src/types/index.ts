export type ProjectType = "desktop" | "website";

export interface Project {
  id: string;
  name: string;
  description?: string;
  type: ProjectType;
  path: string;
  plan_json?: string;
  plan_status?: "draft" | "confirmed" | "rejected";
  created_at: string;
  updated_at: string;
}

export interface PlanStage {
  name: string;
  description: string;
  tasks: string[];
}

export interface DevelopmentPlan {
  summary: string;
  project_type: ProjectType;
  stages: PlanStage[];
  estimated_files: string[];
  dependencies: string[];
  run_commands: string[];
}

export type AgentResponse =
  | { type: "clarification"; questions: string[] }
  | { type: "plan"; plan: DevelopmentPlan }
  | { type: "message"; content: string }
  | { type: "progress"; step: string; detail: string; tool_result?: ToolResult }
  | { type: "done"; summary: string; next_steps: string[] }
  | { type: "command_confirmation"; command: string; description: string }
  | { type: "error"; message: string };

export interface ToolResult {
  success: boolean;
  output: string;
  error?: string;
}

export interface Message {
  id: string;
  role: "system" | "user" | "assistant" | "tool";
  content?: string;
  metadata?: string;
  created_at: string;
}
