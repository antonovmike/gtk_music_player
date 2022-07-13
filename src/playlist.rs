use std::path::Path;
use crate::gtk::ListStoreExt;
use crate::gtk::ToValue;
use crate::gtk::ListStoreExtManual;
use gdk_pixbuf::PixbufLoader;
use gdk_pixbuf::{InterpType, Pixbuf};
use gtk::{
    CellLayoutExt, CellRendererPixbuf, CellRendererText, ListStore,
    StaticType, TreeIter, TreeView,
    TreeViewColumn, TreeViewColumnExt, TreeViewExt, Type, WidgetExt,
};
use id3::Tag;

// The *_COLUMN constant represents the column we'll show in the playlist
// PIXBUF_COLUMN , is a bit special: it will be a hidden column holding the cover of
// a bigger size so that we can show this image in the cover widget we created earlier
const THUMBNAIL_COLUMN: u32 = 0;
const TITLE_COLUMN: u32 = 1;
const ARTIST_COLUMN: u32 = 2;
const ALBUM_COLUMN: u32 = 3;
const GENRE_COLUMN: u32 = 4;
const YEAR_COLUMN: u32 = 5;
const TRACK_COLUMN: u32 = 6;
const PATH_COLUMN: u32 = 7;
const PIXBUF_COLUMN: u32 = 8;
const IMAGE_SIZE: i32 = 256;
const THUMBNAIL_SIZE: i32 = 64;
const INTERP_HYPER:  InterpType = 3;

pub struct Playlist {
    model: ListStore,
    treeview: TreeView,
}

use self::Visibility::*;
#[derive(PartialEq)]
enum Visibility {
    Invisible,
    Visible,
}

impl Playlist {
    pub fn new() -> Self {
        let model = ListStore::new(&[
            Pixbuf::static_type(), // thumbnail image
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Type::String,
            Pixbuf::static_type(), // bigger image; shown for the currently playing song
        ]);
        let treeview = TreeView::new_with_model(&model);
        treeview.set_hexpand(true);
        treeview.set_vexpand(true);
        Self::create_columns(&treeview);
        Playlist { model, treeview }
    }
    fn create_columns(treeview: &TreeView) {
        Self::add_pixbuf_column(treeview, THUMBNAIL_COLUMN as i32,
        Visible);
        Self::add_text_column(treeview, "Title", TITLE_COLUMN as i32);
        Self::add_text_column(treeview, "Artist", ARTIST_COLUMN as i32);
        Self::add_text_column(treeview, "Album", ALBUM_COLUMN as i32);
        Self::add_text_column(treeview, "Genre", GENRE_COLUMN as i32);
        Self::add_text_column(treeview, "Year", YEAR_COLUMN as i32);
        Self::add_text_column(treeview, "Track", TRACK_COLUMN as i32);
        Self::add_pixbuf_column(treeview, PIXBUF_COLUMN as i32, Invisible);
    }
    fn add_text_column(treeview: &TreeView, title: &str, column: i32) {
        let view_column = TreeViewColumn::new();
        view_column.set_title(title);
        let cell = CellRendererText::new();
        view_column.set_expand(true);
        view_column.pack_start(&cell, true);
        view_column.add_attribute(&cell, "text", column);
        treeview.append_column(&view_column);
    }
    fn add_pixbuf_column(treeview: &TreeView, column: i32, visibility:
        Visibility) {
        let view_column = TreeViewColumn::new();
        if visibility == Visible {
            let cell = CellRendererPixbuf::new();
            view_column.pack_start(&cell, true);
            view_column.add_attribute(&cell, "pixbuf", column);
        }
        treeview.append_column(&view_column);
    }
    pub fn view(&self) -> &TreeView {
        &self.treeview
    }

    // Open mp3
    fn set_pixbuf(&self, row: &TreeIter, tag: &Tag) {
        if let Some(picture) = tag.pictures().next() {
        let pixbuf_loader = PixbufLoader::new();
        pixbuf_loader.set_size(IMAGE_SIZE, IMAGE_SIZE);
        pixbuf_loader.loader_write(&picture.data).unwrap();if let Some(pixbuf) = pixbuf_loader.get_pixbuf() {
        let thumbnail = pixbuf.scale_simple(THUMBNAIL_SIZE,
        THUMBNAIL_SIZE, INTERP_HYPER).unwrap();
        self.model.set_value(row, THUMBNAIL_COLUMN,
        &thumbnail.to_value());
        self.model.set_value(row, PIXBUF_COLUMN,
        &pixbuf.to_value());
        }
        pixbuf_loader.close().unwrap();
        }
    }

    // ID3 metadata
    // convert filename to string
    // show if there is no song's title
    // read metadata
    pub fn add(&self, path: &Path) {
        let filename = path.file_stem().unwrap_or_default().to_str().unwrap_or_default();
        let row = self.model.append();
        if let Ok(tag) = Tag::read_from_path(path) {
            let title = tag.title().unwrap_or(filename);
            let artist = tag.artist().unwrap_or("(no artist)");
            let album = tag.album().unwrap_or("(no album)");
            let genre = tag.genre().unwrap_or("(no genre)");
            let year = tag.year().map(|year|
                year.to_string()).unwrap_or("(no year)".to_string());
            let track = tag.track().map(|track|
                track.to_string()).unwrap_or("??".to_string());
            let total_tracks = tag.total_tracks().map(|total_tracks|
                total_tracks.to_string()).unwrap_or("??".to_string());
            let track_value = format!("{} / {}", track, total_tracks);
            self.set_pixbuf(&row, &tag);
            // create new row in model, call set_pixbuf()
            self.model.set_value(&row, TITLE_COLUMN, &title.to_value());
            self.model.set_value(&row, ARTIST_COLUMN, &artist.to_value());
            self.model.set_value(&row, ALBUM_COLUMN, &album.to_value());
            self.model.set_value(&row, GENRE_COLUMN, &genre.to_value());
            self.model.set_value(&row, YEAR_COLUMN, &year.to_value());
            self.model.set_value(&row, TRACK_COLUMN, &track_value.to_value());
        }
        else {
            self.model.set_value(&row, TITLE_COLUMN, &filename.to_value());
            }
        let path = path.to_str().unwrap_or_default();
        self.model.set_value(&row, PATH_COLUMN, &path.to_value());
    }
}
