use glib::clone;
use gst::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, FileChooserAction, FileChooserDialog, Scale};
use std::rc::Rc;

extern crate gdk;

pub fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("Rust Audio Player");
    window.set_default_size(600, 300); // Increased size for better layout

    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(
            b"
    window {
        background-color: #2E1A47;
    }
    button {
        background-color: #6B5B9A;
        color: white;
        border: none;
        border-radius: 8px;
        padding: 12px 20px;
        margin: 10px 0;
        font-size: 16px;
        transition: background-color 0.3s ease;
    }
    button:hover {
        background-color: #5A4A8A;
        box-shadow: 0px 5px 15px rgba(0, 0, 0, 0.2);
    }
    button:active {
        background-color: #4A3A7A;
        box-shadow: none;
    }
    scale {
        margin: 20px 0;
        padding: 0 10px;
    }
    scale trough {
        background-color: #374151;
    }
    scale slider {
        background-color: #60A5FA;
    }
    label {
        color: white;
        font-size: 18px;
        margin-top: 15px;
    }
    ",
        )
        .unwrap();

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Unable to obtain default screen"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );
    let play_button = Button::with_label("▶"); // Play icon
    let select_file_button = Button::with_label("📂"); // File icon
    let pause_button = Button::with_label("⏸️"); // Pause icon
    let stop_button = Button::with_label("⏹️"); // Stop icon

    let adjustment = gtk::Adjustment::new(100.0, 0.0, 200.0, 1.0, 0.0, 0.0); // Create Adjustment for 0% to 200%
    let volume_scale = Scale::new(gtk::Orientation::Horizontal, Some(&adjustment)); // Pass Adjustment
    volume_scale.set_range(0.0, 200.0); // Set volume range to 0% - 200%
    volume_scale.set_value(100.0); // Default volume level to 100%

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Change the layout of buttons to vertical
    let vbox_buttons = gtk::Box::new(gtk::Orientation::Vertical, 5); // Create a vertical box for buttons
    vbox_buttons.pack_start(&select_file_button, true, true, 0); // Add select file button
    vbox_buttons.pack_start(&play_button, true, true, 0);
    vbox_buttons.pack_start(&pause_button, true, true, 0);
    vbox_buttons.pack_start(&stop_button, true, true, 0);

    vbox.pack_start(&vbox_buttons, true, true, 0); // Add the vertical box to the main vertical box
    vbox.pack_start(&volume_scale, true, true, 0); // Add volume control below the buttons
    window.add(&vbox);
    window.show_all();

    let pipeline = gst::Pipeline::new(None);
    let playbin = Rc::new(gst::ElementFactory::make("playbin", None).unwrap()); // Wrap playbin in Rc
    pipeline.add(&*playbin).unwrap(); // Dereference Rc to get the Element

    select_file_button.connect_clicked(clone!(@strong playbin => move |_| {
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
                // Load metadata image here
                // Example: image_visualizer.set_from_file(file); // Adjust as needed
            }
        }
        dialog.close();
    }));

    play_button.connect_clicked(clone!(@strong playbin => move |_| {
        playbin.set_state(gst::State::Playing).unwrap();
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

    volume_scale.connect_value_changed(clone!(@strong playbin => move |scale| {
        let volume = scale.get_value() / 100.0;
        playbin.set_property("volume", volume).unwrap();
    }));
}
