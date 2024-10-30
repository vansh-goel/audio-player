mod ui;

extern crate gio;
extern crate gstreamer as gst;
extern crate gtk;

use gio::prelude::*; // This brings ApplicationExt into scope
use gtk::prelude::*; // This brings other needed GTK traits into scope

fn main() {
    // Initialize GStreamer
    if let Err(e) = gst::init() {
        eprintln!("Failed to initialize GStreamer: {}", e);
        return;
    }

    // Create a new application
    let application = gtk::Application::new(Some("com.example.audio_player"), Default::default())
        .expect("Initialization failed...");

    // Connect the activate signal
    application.connect_activate(|app| {
        ui::build_ui(app);
    });

    // Run the application
    application.run(&std::env::args().collect::<Vec<_>>());
}
