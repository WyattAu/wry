pub mod application;
pub mod window;

pub use application::App;
pub use window::Window;

#[cfg(test)]
mod tests {
    use super::{App, Window};
    use gtk4::prelude::*;

    #[test]
    fn test_window_creation() {
        gtk4::init().expect("Failed to initialize GTK");

        let app = App::new("com.ferro.test");
        let gtk_app = app.gtk_app();

        let window = Window::new(gtk_app);

        window.set_title("Test Window");
        window.set_default_size(800, 600);

        assert!(!window.gtk_window().is_visible());
    }
}
