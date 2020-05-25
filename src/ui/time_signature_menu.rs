extern crate gtk;
extern crate gio;

use gtk::prelude::*;

const UPPER_INTERVAL: [&str; 15] = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16"];
const LOWER_INTERVAL: [&str; 4] = ["1", "2", "4", "8"];

fn selection_box(wrapper: &gtk::Box, values: &[&str]) {
  let select = gtk::ComboBoxText::new();
  for _i in 0..values.len() {
    select.append_text(values[_i]);
  }
  select.set_active(Some(2));
  wrapper.add(&select);
}

pub fn build(container: &gtk::Box) {
  let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 2);
  let divider = gtk::Label::new(Some("/"));
  gtk::WidgetExt::set_widget_name(&divider, "time-signature-divider");
  selection_box(&wrapper, &UPPER_INTERVAL);
  wrapper.add(&divider);
  selection_box(&wrapper, &LOWER_INTERVAL);
  container.add(&wrapper);

}