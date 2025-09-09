import { invoke } from "@tauri-apps/api/core";

export async function call(sql: string) {
  try {
    const response = await invoke("fetch", {
      sql: sql,
    });
    return response;
  } catch (error) {
    console.error("Error invoking greet:", error);
  }
}
