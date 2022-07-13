extern crate gio;
extern crate gtk;
extern crate gtk_sys;

use crate::gtk::{DialogExt, FileChooserExt, FileFilterExt, ToolButtonExt,};
use gtk::WindowType::Toplevel;
use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::Inhibit;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{Adjustment, Image, Scale, ScaleExt};
use gtk::{Application, ApplicationWindow, Window, GtkWindowExt, WidgetExt};
use gtk::{ContainerExt, SeparatorToolItem, ToolButton, Toolbar};
use gtk::{FileChooserAction, FileChooserDialog, FileFilter};
use std::env;
use std::path::PathBuf;
use toolbar::MusicToolbar;
use std::rc::Rc;
use gtk_sys::{GTK_RESPONSE_ACCEPT, GTK_RESPONSE_CANCEL};

use crate::playlist::Playlist;

mod playlist;
mod toolbar;

const PLAY_STOCK: &str = "gtk-media-play";
const PAUSE_STOCK: &str = "gtk-media-play";
const RESPONSE_ACCEPT: i32 = GTK_RESPONSE_ACCEPT as i32;
const RESPONSE_CANCEL: i32 = GTK_RESPONSE_CANCEL as i32;

struct App {
    adjustment: Adjustment,
    cover: Image,
    playlist: Rc<Playlist>,
    toolbar: MusicToolbar,
    // window: ApplicationWindow, // change to next line
    window: Window,
}

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

        // TOOLBAR
        let vbox = gtk::Box::new(Vertical, 0); // V: toolbar ontop, H: toolbar on the right
        window.add(&vbox);
        let toolbar = MusicToolbar::new();
        vbox.add(toolbar.toolbar());
        let playlist = Rc::new(Playlist::new());
        vbox.add(playlist.view()); // makes playlist visible. WORKS!!!
        let cover = Image::new();

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

impl App {
    fn new(application: Application) -> Self {
        let window = ApplicationWindow::new(&application);
        let toolbar = MusicToolbar::new();
        let adjustment = Adjustment::new(0.0, 0.0, 10.0, 0.0, 0.0, 0.0);
        let scale = Scale::new(Horizontal, &adjustment);
        let vbox = gtk::Box::new(Vertical, 0);
        let cover = Image::new();
        let playlist = Rc::new(Playlist::new()); // makes playlist visible. DOES NOT WORK without vbox.add(playlist.view())
        let window = Window::new(Toplevel); // Root, Toplevel, Child, Temp, Foreign, Offscreen, Subsurface,

        scale.set_draw_value(false);

        vbox.add(&scale);
        vbox.add(playlist.view());

        window.set_title("Music Player (impl App)");
        window.add(&vbox);
        window.add(toolbar.toolbar());
        window.show_all();

        let app = App {
            adjustment,
            cover,
            playlist,
            toolbar,
            window,
        };

        app.connect_events();
        app.connect_toolbar_events();
        app
    }
    pub fn connect_toolbar_events(&self) {
        let window = self.window.clone();
        self.toolbar.quit_button.connect_clicked(move |_| {
            window.destroy();
        });
        let play_button = self.toolbar.play_button.clone();
        self.toolbar.play_button.connect_clicked(move |_| {
            if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
                play_button.set_stock_id(PAUSE_STOCK);
            } else {
                play_button.set_stock_id(PLAY_STOCK);
            }
        });
    }

    fn show_open_dialog(parent: &ApplicationWindow) -> Option<PathBuf> {
        let mut file = None;
        let dialog = FileChooserDialog::new(Some("Select an MP3 audio file"), Some(parent), FileChooserAction::Open);
        let filter = FileFilter::new();
        filter.add_mime_type("audio/mp3");
        filter.set_name("MP3 audio file");
        dialog.add_filter(&filter);
        dialog.add_button("Cancel", RESPONSE_CANCEL);
        dialog.add_button("Accept", RESPONSE_ACCEPT);
        let result = dialog.run();
        if result == RESPONSE_ACCEPT {
        file = dialog.get_filename();
        }
        dialog.destroy();
        file
    }
    
    fn connect_events(&self) {}         // FIX IT
}