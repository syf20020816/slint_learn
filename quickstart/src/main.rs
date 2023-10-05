use slint::PlatformError;

slint::include_modules!();

fn main() -> Result<(), PlatformError> {
    let app = App::new().unwrap();
    let app_handler = app.as_weak();
    app.on_clicked(move || {
        let tmp_handler = app_handler.unwrap();
        let mut content = tmp_handler.get_content();
        let _ = content.push_str(" world");
        let _ = tmp_handler.set_content(content);
    });

    app.run()
}
