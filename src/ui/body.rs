extern crate gtk;
extern crate gio;

use gtk::prelude::*;

pub fn build(container: &gtk::Box) {
  
  let button = gtk::Button::new_with_label("Tap tempo");
  gtk::WidgetExt::set_widget_name(&button, "button");
  container.add(&button);
}