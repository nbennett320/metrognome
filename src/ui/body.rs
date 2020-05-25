extern crate gtk;
extern crate gio;

use gtk::prelude::*;

#[path = "./time_signature_menu.rs"] mod time_signature_menu;

pub fn build(container: &gtk::Box) {
  time_signature_menu::build(&container);
  let button = gtk::Button::new_with_label("Tap tempo");
  gtk::WidgetExt::set_widget_name(&button, "button");
  container.add(&button);
}