use tauri::command;

#[command]
pub fn greet(name: &str) -> String {
    println!("Hello {}!", name);
    format!("Hello, {}!", name)
}