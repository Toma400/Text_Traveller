mod converters;
use druid::{
    widget::{
        Container,
        Label,
        Split
    },
    AppLauncher,
    Color,
    Widget,
    WindowDesc
};

fn build_ui () -> impl Widget<()> {
    Split::columns(
        Container::new(
            Label::new(":)"),
        )
        .border(Color::grey(0.6), 2.0),
        Container::new(
            Label::new(":)"),
        )
        .border(Color::grey(0.6), 2.0),
    )
}

fn main() {
    let main_window = WindowDesc::new(build_ui)
        .window_size((600.0, 400.0))
        .title("TextTraveller");
    let initial_data = ();

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application!");
}
