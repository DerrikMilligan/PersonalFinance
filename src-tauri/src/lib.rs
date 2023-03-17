use reqwest;

// A custom error type that represents all possible in our command
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to make the request: {0}")]
    Reqwest(#[from] reqwest::Error),
}

// we must also implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

#[tauri::command]
async fn hello_command(name: String) -> Result<String, Error> {
  let body = reqwest::get("https://randomuser.me/api/").await?.text().await?;
  Ok(body.into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![hello_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
