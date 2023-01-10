use std::path::PathBuf;
use std::fs::File;
use serde::{ser::Serializer, Serialize};
use tauri::{
  command,
  plugin::{Builder, TauriPlugin},
  Runtime, Window,
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Zip(#[from] zip_extract::ZipExtractError),
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}


#[command]
async fn extract<R: Runtime>(
  _window: Window<R>,
  src_zip: &str,
  out_dir: &str,
) -> Result<String> {
  let target_dir = PathBuf::from(out_dir);
  let file = File::open(zip_path)?;
  zip_extract::extract(file, &target_dir, true)?;
  Ok("Extracted".to_string())
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("extract")
    .invoke_handler(tauri::generate_handler![extract])
    .build()
}
