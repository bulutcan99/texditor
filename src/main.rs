use druid::{AppLauncher, LocalizedString, WindowDesc};
use ui::{ui_builder, Delegate};

pub mod command;
pub mod state;
pub mod ui;
fn main() {
    let main_window = WindowDesc::new(ui_builder())
        .title(LocalizedString::new("open-save-demo").with_placeholder("Opening/Saving Demo"));
    let data = "Type here.".to_owned();
    AppLauncher::with_window(main_window)
        .delegate(Delegate)
        .log_to_console()
        .launch(data)
        .expect("launch failed");
}
