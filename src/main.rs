use druid::{
    widget::Label,
    AppLauncher,
    Widget,
    WindowDesc
};

fn build_ui () -> impl Widget<()> {
    Label::new(":)")
}

fn convert_picker (mode: &str, file: &str) -> () {
    match mode {
        "MD to HTML"  => (),
        "MD to PDF"   => (),
        "HTML to PDF" => (),
        _             => (),
    };
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .window_size((600.0, 400.0))
        .title("TextTraveller");
    let initial_data = ();

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application.");
}
