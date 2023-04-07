mod imp;

use gtk::subclass::prelude::*;

use crate::symbol::Symbol;
use gtk::{gio, glib, prelude::*};

// Public part of the Model type.
glib::wrapper! {
    pub struct YahooFinanceModel(ObjectSubclass<imp::YahooFinanceModel>) @implements gio::ListModel;
}

// Constructor for new instances. This simply calls glib::Object::new()
impl YahooFinanceModel {
    pub fn new() -> YahooFinanceModel {
        glib::Object::new()
    }

    pub fn append(&self, obj: &Symbol) {
        let imp = self.imp();
        let index = {
            // Borrow the data only once and ensure the borrow guard is dropped
            // before we emit the items_changed signal because the view
            // could call get_item / get_n_item from the signal handler to update its state
            let mut data = imp.symbols.borrow_mut();
            data.push(obj.clone());
            data.len() - 1
        };
        // Emits a signal that 1 item was added, 0 removed at the position index
        self.items_changed(index as u32, 0, 1);
    }

    pub fn remove(&self, index: u32) {
        let imp = self.imp();
        imp.symbols.borrow_mut().remove(index as usize);
        // Emits a signal that 1 item was removed, 0 added at the position index
        self.items_changed(index, 1, 0);
    }
}

impl Default for YahooFinanceModel {
    fn default() -> Self {
        Self::new()
    }
}
