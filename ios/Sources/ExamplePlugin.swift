import Tauri
import UIKit
import WebKit

// Extension to handle keyboard notifications
extension WKWebView {
    @objc func keyboardWillShow(_ notification: Notification) {
        var keyboardHeight: CGFloat = 0
        
        if let rect = notification.userInfo?[UIResponder.keyboardFrameEndUserInfoKey] as? CGRect {
            keyboardHeight = rect.size.height
        }

        // Get the duration of the animation
        let duration = notification.userInfo?[UIResponder.keyboardAnimationDurationUserInfoKey] as? Double ?? 0.0

        // Post a message to the web content
        let script = "window.dispatchEvent(new CustomEvent('keyboardWillShow', { detail: { height: \(keyboardHeight), duration: \(duration) } }));"
        self.evaluateJavaScript(script, completionHandler: nil)
    }
    
    @objc func keyboardWillHide(_ notification: Notification) {
        // Get the duration of the animation  
        let duration = notification.userInfo?[UIResponder.keyboardAnimationDurationUserInfoKey] as? Double ?? 0.0

        // Post a message to the web content
        let script = "window.dispatchEvent(new CustomEvent('keyboardWillHide', { detail: { duration: \(duration) } }));"
        self.evaluateJavaScript(script, completionHandler: nil)
    }
    
    func setupKeyboardObservers() {
        NotificationCenter.default.addObserver(self, selector: #selector(keyboardWillShow), name: UIResponder.keyboardWillShowNotification, object: nil)
        NotificationCenter.default.addObserver(self, selector: #selector(keyboardWillHide), name: UIResponder.keyboardWillHideNotification, object: nil)
    }
    
    func removeKeyboardObservers() {
        NotificationCenter.default.removeObserver(self, name: UIResponder.keyboardDidShowNotification, object: nil)
        NotificationCenter.default.removeObserver(self, name: UIResponder.keyboardDidHideNotification, object: nil)
        NotificationCenter.default.removeObserver(self, name: UIResponder.keyboardWillChangeFrameNotification, object: nil)
        NotificationCenter.default.removeObserver(self, name: UIResponder.keyboardDidChangeFrameNotification, object: nil)
        NotificationCenter.default.removeObserver(self, name: UIResponder.keyboardWillShowNotification, object: nil)
        NotificationCenter.default.removeObserver(self, name: UIResponder.keyboardWillHideNotification, object: nil)
    }
}

class SetKeyboardStyleArgs: Decodable {
  let style: String?
}

class ExamplePlugin: Plugin {
  @objc open override func load(webview: WKWebView) {
    // Setup keyboard observers directly on the webview
    webview.removeKeyboardObservers()
    webview.setupKeyboardObservers()
    
    webview.scrollView.contentInsetAdjustmentBehavior = .never
    webview.scrollView.isScrollEnabled = false
    webview.scrollView.keyboardDismissMode = .interactive
    webview.isOpaque = false
  }
}

@_cdecl("init_plugin_virtual_keyboard")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
