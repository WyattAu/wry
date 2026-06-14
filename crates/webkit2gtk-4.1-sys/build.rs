fn main() {
    pkg_config::Config::new()
        .atleast_version("2.40")
        .probe("webkit2gtk-4.1")
        .unwrap();
    
    pkg_config::Config::new()
        .atleast_version("2.40")
        .probe("javascriptcoregtk-4.1")
        .unwrap();
}
