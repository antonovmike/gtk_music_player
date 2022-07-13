extern crate gdk_pixbuf;
extern crate id3;

use gtk::{ContainerExt, ToolButton, Toolbar};

pub struct MusicToolbar {
    pub open_button: ToolButton,
    pub stop_button: ToolButton,
    toolbar: Toolbar,
}

impl MusicToolbar {
    pub fn new() -> Self {
        let toolbar = Toolbar::new();

        let open_button = ToolButton::new_from_stock("gtk-open");
        toolbar.add(&open_button);
        let next_button = ToolButton::new_from_stock("gtk-next");
        toolbar.add(&next_button);
        let play_button = ToolButton::new_from_stock("gtk-play");
        toolbar.add(&play_button);
        let previous_button = ToolButton::new_from_stock("gtk-previous");
        toolbar.add(&previous_button);
        let remove_button = ToolButton::new_from_stock("gtk-remove");
        toolbar.add(&remove_button);
        let stop_button = ToolButton::new_from_stock("gtk-stop");
        toolbar.add(&stop_button);
        let quit_button = ToolButton::new_from_stock("gtk-quit");

        toolbar.add(&quit_button);

        MusicToolbar {
            open_button,
            stop_button,
            toolbar,
        }
    }
    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}
