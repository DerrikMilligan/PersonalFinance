#[tauri::command]
async fn hello_command(name: String) -> String {
  format!("Hello {name}").into()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
