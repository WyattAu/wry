use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.ferro.gtk4test")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("GTK4 Wayland Test")
            .default_width(800)
            .default_height(600)
            .build();

        window.show();
        println!("GTK4 window created and shown!");
    });

    app.run();
}
