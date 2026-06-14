use gtk::prelude::*;
use wry::{WebViewBuilder, WebViewBuilderExtUnix};

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.example.WaylandTest")
        .build();

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Wayland Test")
            .default_width(800)
            .default_height(600)
            .build();

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        window.add(&vbox);

        let webview = WebViewBuilder::new()
            .with_url("https://example.com")
            .build_gtk(&vbox)
            .expect("Failed to create webview");

        webview.show_all();
        window.show_all();
    });

    app.run();
}
