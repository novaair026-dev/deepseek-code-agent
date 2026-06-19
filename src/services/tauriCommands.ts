import { invoke } from "@tauri-apps/api/core";
import { AgentResponse, DevelopmentPlan, Project, ProjectType } from "../types";

export async function getApiKeyConfigured(): Promise<boolean> {
  const result = await invoke<{ configured: boolean }>("get_api_key_configured");
  return result.configured;
}

export async function setApiKey(apiKey: string): Promise<void> {
  await invoke("set_api_key", { apiKey });
}

export async function createProject(
  name: string,
  description: string,
  type: ProjectType
): Promise<Project> {
  return await invoke<Project>("create_project", {
    payload: { name, description, type },
  });
}

export async function listProjects(): Promise<Project[]> {
  return await invoke<Project[]>("list_projects");
}

export async function getProject(id: string): Promise<Project | null> {
  return await invoke<Project | null>("get_project", { id });
}

export async function deleteProject(id: string): Promise<void> {
  await invoke("delete_project", { id });
}

export async function sendMessage(
  projectId: string,
  content: string,
  sessionId?: string,
  referencedFiles?: string[]
): Promise<{ session_id: string; response: AgentResponse }> {
  return await invoke("send_message", {
    req: { project_id: projectId, content, session_id: sessionId, referenced_files: referencedFiles },
  });
}

export async function approvePlan(
  projectId: string,
  action: "confirm" | "reject" | "modify",
  plan?: DevelopmentPlan,
  feedback?: string
): Promise<AgentResponse> {
  return await invoke("approve_plan", {
    req: { project_id: projectId, action, modified_plan: plan, feedback },
  });
}

export async function confirmCommand(
  projectId: string,
  sessionId: string,
  command: string
): Promise<AgentResponse> {
  return await invoke("confirm_command", { projectId, sessionId, command });
}
