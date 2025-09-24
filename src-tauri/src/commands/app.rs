use crate::context::schema::AppResult;
use tauri::AppHandle;

#[tauri::command]
pub async fn restart_app(app_handle: AppHandle) -> AppResult<()> {
    // 延迟一小段时间确保响应发送完成
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    // 重启应用
    app_handle.restart();

    // 这行代码永远不会执行，因为restart()会终止应用
    // 但我们需要返回Ok(())来满足函数签名
    Ok(())
}
