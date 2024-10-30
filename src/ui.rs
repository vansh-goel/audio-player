use glib::clone;
use gst::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog};
use std::rc::Rc; // Added import for Rc

pub fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("Rust Audio Player");
    window.set_default_size(300, 100);
    let play_button = Button::with_label("Play");
    let pause_button = Button::with_label("Pause");
    let stop_button = Button::with_label("Stop");
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    vbox.pack_start(&play_button, true, true, 0);
    vbox.pack_start(&pause_button, true, true, 0);
    vbox.pack_start(&stop_button, true, true, 0);
    window.add(&vbox);
    window.show_all();
    let pipeline = gst::Pipeline::new(None);
    let playbin = Rc::new(gst::ElementFactory::make("playbin", None).unwrap()); // Wrap playbin in Rc
    pipeline.add(&*playbin).unwrap(); // Dereference Rc to get the Element
    play_button.connect_clicked(clone!(@strong playbin => move |_| {
        let dialog = FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            FileChooserAction::Open,
        );
        dialog.add_button("_Open", gtk::ResponseType::Accept);
        dialog.add_button("_Cancel", gtk::ResponseType::Cancel);
        if dialog.run() == gtk::ResponseType::Accept.into() {
            if let Some(file) = dialog.get_filename() {
                let uri = format!("file://{}", file.to_str().unwrap());
                playbin.set_property("uri", &uri).unwrap();
                playbin.set_state(gst::State::Playing).unwrap();
            }
        }
        dialog.close();
    }));
    pause_button.connect_clicked(
        clone!(@strong playbin => move |_| { // Use @strong instead of @weak
            playbin.set_state(gst::State::Paused).unwrap();
        }),
    );
    stop_button.connect_clicked(
        clone!(@strong playbin => move |_| { // Use @strong instead of @weak
            playbin.set_state(gst::State::Null).unwrap();
        }),
    );
}
