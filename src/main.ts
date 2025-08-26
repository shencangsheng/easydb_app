import { invoke } from "@tauri-apps/api/core";

export async function callGreet() {
  try {
    const response = await invoke("greet", { name: "World" });
    console.log(response); // 输出: Hello, World!
  } catch (error) {
    console.error("Error invoking greet:", error);
  }
}
