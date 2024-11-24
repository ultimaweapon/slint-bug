use self::ui::{MainWindow, Page};
use slint::ComponentHandle;

fn main() {
    let page = Page::About; // This line should not compile because Page is not exported.

    MainWindow::new().unwrap().run().unwrap();
}

mod ui {
    slint::include_modules!();
}
