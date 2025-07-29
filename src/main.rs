use tray_item::{IconSource, TrayItem};

mod mouse_handler;

fn main() {
    mouse_handler::move_silently();

    let mut tray = TrayItem::new("Digital Vigilance", IconSource::Resource("")).unwrap();
    tray.add_label("Set refresh interval").unwrap();

    tray.add_menu_item("3 min", || {
        println!("todo");
    })
    .unwrap();

    let inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();
}
