extern crate gdk_pixbuf;
extern crate id3;

use crate::gtk::ImageExt;
use crate::Playlist;
use gtk::Image;
use gtk::{ContainerExt, ToolButton, Toolbar, WidgetExt};
// use gtk::{SeparatorToolItem, ToolButstonExt};

// use super::App;

// const PLAY_STOCK: &str = "gtk-media-play";
// const PAUSE_STOCK: &str = "gtk-media-pause";

pub struct MusicToolbar {
    pub open_button: ToolButton,
    pub next_button: ToolButton,
    pub play_button: ToolButton,
    pub previous_button: ToolButton,
    pub quit_button: ToolButton,
    pub remove_button: ToolButton,
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
            next_button,
            play_button,
            previous_button,
            quit_button,
            remove_button,
            stop_button,
            toolbar,
        }
    }
    pub fn toolbar(&self) -> &Toolbar {
        &self.toolbar
    }
}

fn set_cover(cover: &Image, playlist: &Playlist) {
    cover.set_from_pixbuf(playlist.pixbuf().as_ref());
    cover.show();
}

/*
call this function from the click event handler of the play button:

let playlist = self.playlist.clone();
let cover = self.cover.clone();
self.toolbar.play_button.connect_clicked(move |_| {
    if play_button.get_stock_id() == Some(PLAY_STOCK.to_string()) {
    play_button.set_stock_id(PAUSE_STOCK);
    set_cover(&cover, &playlist);
} else {
    play_button.set_stock_id(PLAY_STOCK);
}});
*/