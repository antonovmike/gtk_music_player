extern crate gio;
extern crate gtk;
extern crate gtk_sys;

use crate::playlist::Playlist;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::Inhibit;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Adjustment, Image, ImageExt, Scale, ScaleExt};
use gtk::{Application, ApplicationWindow, GtkWindowExt, WidgetExt};
use gtk::{ContainerExt, SeparatorToolItem, ToolButton, Toolbar};
use std::env;
use toolbar::MusicToolbar;

mod playlist;
mod toolbar;

const PLAY_STOCK: &str = "gtk-media-play";

fn main() {
    let application = Application::new("com.github.rust-by-example", ApplicationFlags::empty())
        .expect("Application initialization failed");
    // create window, set it's title, show it to the screen
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(&application);
        let toolbar = Toolbar::new();
        let open_button = ToolButton::new_from_stock("gtk-open");

        toolbar.add(&open_button);
        toolbar.add(&SeparatorToolItem::new());
        let previous_button = ToolButton::new_from_stock("gtk-media-previous");
        toolbar.add(&previous_button);
        let play_button = ToolButton::new_from_stock(PLAY_STOCK);
        toolbar.add(&play_button);
        let stop_button = ToolButton::new_from_stock("gtk-media-stop");
        toolbar.add(&stop_button);
        let next_button = ToolButton::new_from_stock("gtk-media-next");
        toolbar.add(&next_button);
        toolbar.add(&SeparatorToolItem::new());
        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);
        toolbar.add(&SeparatorToolItem::new());
        let quit_button = ToolButton::new_from_stock("gtk-quit");
        toolbar.add(&quit_button);

        // TOOLBAR v1
        // window.add(&toolbar);

        // TOOLBAR v2
        let vbox = gtk::Box::new(Vertical, 0); // V: toolbar ontop, H: toolbar on the right
        window.add(&vbox);
        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());
        let playlist = Playlist::new();
        vbox.add(playlist.view()); // makes playlist visible. WORKS!!!
        let cover = Image::new();
        cover.set_from_file("cover.jpg");
        vbox.add(&cover);

        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        scale.set_draw_value(false);
        vbox.add(&scale);

        window.set_title("Music Player (fn main)");
        window.show_all();
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());

    application.connect_startup(startup_handler);
    application.run(&env::args().collect::<Vec<_>>());
}

fn startup_handler(application: &Application) {
    let window = ApplicationWindow::new(&application);
    window.set_title("Music Player (fn startup_handler)");
    window.connect_delete_event(|_, _| Inhibit(false));
    window.show();
}