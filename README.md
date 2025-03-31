# tauri-plugin-virtual-keyboard

Gives control over the virtual keyboard on mobile platforms this can be used to prevent the webview from scrolling when the keyboard is open on iOS
and also receive events when the keyboard is shown or hidden.

Android support is not yet implemented.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✗         |
| Windows  | ✗         |
| macOS    | ✗         |
| Android  | ✗         |
| iOS      | ✓         |

## Usage

Defaults to disabling scrolling in the webview and then receiving events when the keyboard is shown or hidden.

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_virtual_keyboard::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Listen to the window events from the webview.

```typescript
type KeyboardEvent = CustomEventInit<{
  height: number;
  duration: number;
}>;

// Add this to your index.html
window.addEventListener('keyboardWillShow', (event: KeyboardEvent) => {
  console.log('Keyboard will show: ', event.detail);
  // Use height information as needed
});

window.addEventListener('keyboardWillHide', () => {
  console.log('Keyboard will hide';
  // Handle keyboard hiding
});
```
