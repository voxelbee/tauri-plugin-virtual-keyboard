use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

// use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_virtual_keyboard);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<VirtualKeyboard<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.virtual-keyboard", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_virtual_keyboard)?;
  Ok(VirtualKeyboard(handle))
}

/// Access to the virtual-keyboard APIs.
pub struct VirtualKeyboard<R: Runtime>(PluginHandle<R>);

// impl<R: Runtime> VirtualKeyboard<R> {
//   pub fn set_keyboard_style(&self, payload: SetKeyboardStyleRequest) -> crate::Result<()> {
//     self
//       .0
//       .run_mobile_plugin("setKeyboardStyle", payload)
//       .map_err(Into::into)
//   }
// }
