use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

// pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::VirtualKeyboard;
#[cfg(mobile)]
use mobile::VirtualKeyboard;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the virtual-keyboard APIs.
pub trait VirtualKeyboardExt<R: Runtime> {
  fn virtual_keyboard(&self) -> &VirtualKeyboard<R>;
}

impl<R: Runtime, T: Manager<R>> crate::VirtualKeyboardExt<R> for T {
  fn virtual_keyboard(&self) -> &VirtualKeyboard<R> {
    self.state::<VirtualKeyboard<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("virtual-keyboard")
    // .invoke_handler(tauri::generate_handler![commands::set_keyboard_style])
    .setup(|app, api| {
      #[cfg(mobile)]
      let virtual_keyboard = mobile::init(app, api)?;
      #[cfg(desktop)]
      let virtual_keyboard = desktop::init(app, api)?;
      app.manage(virtual_keyboard);
      Ok(())
    })
    .build()
}
