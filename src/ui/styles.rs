extern crate gdk;
extern crate gtk;
extern crate glib;
extern crate gio;

use gtk::prelude::*;

const STYLE: &str = "
  #window {
    background-color: #112233;
  }

  #body {
    background-color: inherit;
  }

  #button1 {
    background-color: #ffffff;
    color: #000000;
  }

  #time-signature-divider {
    color: #ffffff;
  }
";

pub fn apply_styles() {
  let provider = gtk::CssProvider::new();
  provider.load_from_data(STYLE.as_bytes())
    .expect("Error loading CSS");
  gtk::StyleContext::add_provider_for_screen(
    &gdk::Screen::get_default()
      .expect("Error initializing CSS provider"),
    &provider,
    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
  );
}