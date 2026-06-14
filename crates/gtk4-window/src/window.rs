use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

pub struct Window {
    application: Application,
    window: ApplicationWindow,
}

impl Window {
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Ferro")
            .default_width(1200)
            .default_height(800)
            .build();

        Self {
            application: app.clone(),
            window,
        }
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(Some(title));
    }

    pub fn set_default_size(&self, width: i32, height: i32) {
        self.window.set_default_size(width, height);
    }

    pub fn show(&self) {
        self.window.show();
    }

    pub fn present(&self) {
        self.window.present();
    }

    pub fn gtk_window(&self) -> &ApplicationWindow {
        &self.window
    }
}
