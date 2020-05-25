extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

mod styles;

fn build_ui(application: &gtk::Application) {
  let window = gtk::ApplicationWindow::new(application);

  window.set_title("Metrognome");
  window.set_border_width(10);
  window.set_position(gtk::WindowPosition::Center);
  window.set_default_size(800, 400);

  let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
  
  let button = gtk::Button::new_with_label("Tap tempo");
  gtk::WidgetExt::set_widget_name(&button, "button1");

  vbox.add(&button);
  window.add(&vbox);
  window.show_all();
}

pub fn init() {
    let application = gtk::Application::new(
      Some("com.gnome.metrognome"), 
      Default::default()
    ).expect("Failed to initialize GTK application");

    application.connect_activate(|app| {
      styles::apply_styles();
      build_ui(app);
    });

    application.run(&[]);
}