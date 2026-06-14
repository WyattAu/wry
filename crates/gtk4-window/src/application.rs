use gtk4::prelude::*;
use gtk4::Application;

pub struct App {
    application: Application,
}

impl App {
    pub fn new(app_id: &str) -> Self {
        let application = Application::builder()
            .application_id(app_id)
            .build();

        Self { application }
    }

    pub fn run(&self) {
        self.application.run();
    }

    pub fn gtk_app(&self) -> &Application {
        &self.application
    }
}
