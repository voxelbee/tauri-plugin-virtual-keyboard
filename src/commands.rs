// use tauri::{AppHandle, command, Runtime};

// use crate::models::*;
// use crate::Result;
// use crate::VirtualKeyboardExt;

// #[command]
// pub(crate) async fn set_keyboard_style<R: Runtime>(
//     app: AppHandle<R>,
//     payload: SetKeyboardStyleRequest,
// ) -> Result<()> {
//     #[cfg(mobile)]
//     return app.virtual_keyboard().set_keyboard_style(payload);
//     #[cfg(desktop)]
//     return Ok(());
// }
