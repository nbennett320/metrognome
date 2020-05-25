extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

fn menu_item(wrapper: &gtk::Box, values: &[u8]) {
  let menu = gtk::Menu::new();
  let mut items = [gtk::MenuItem, values.len()]
  for i in 0..values.len() {
    let item = gtk::MenuItemBuilder::new();
    let item_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    item_box.start_pack(1, false, false)
    
    let item_label = gtk::Label::new(Some(values[i]));
  }
}

pub fn build(container: &gtk::Box) {
  let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 2);

}