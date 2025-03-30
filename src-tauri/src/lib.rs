

#[cfg_attr(mobile, tauri::mobile_entry_point)]

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JavaScript!");
}

pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
