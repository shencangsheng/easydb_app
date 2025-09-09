import { invoke } from "@tauri-apps/api/core";

export async function callGreet() {
  try {
    const response = await invoke("fetch", {
      sql: "select * from read_tsv('/Users/shencangsheng/Downloads/nightly-VariantSummaries.tsv', infer_schema(false))",
    });
    console.log(response);
  } catch (error) {
    console.error("Error invoking greet:", error);
  }
}
