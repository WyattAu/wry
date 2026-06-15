// examples/gtk4_integration_test.rs
//
// Tests the GTK4-window feature of wry on Wayland.
// Note: build_with_gtk4_window() creates a GTK3 window internally because
// WebKitGTK is GTK3-based, and blocks via gtk::main() until the window is closed.
//
// Run with: cargo run --example gtk4_integration_test --features gtk4-window

use wry::{dpi::LogicalSize, Rect, WebViewBuilder, WebViewBuilderExtUnix};

fn main() {
    println!("Creating GTK4 window with WebKitGTK webview...");

    // build_with_gtk4_window() blocks until the GTK window is closed
    let _webview = WebViewBuilder::new()
        .with_url("https://example.com")
        .with_bounds(Rect {
            position: wry::dpi::LogicalPosition::new(100.0, 100.0).into(),
            size: LogicalSize::new(1200.0, 800.0).into(),
        })
        .build_with_gtk4_window()
        .expect("Failed to create webview");

    println!("Webview created and GTK main loop ended.");
}
