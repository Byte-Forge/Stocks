mod imp;

use gtk::glib;

use crate::symbol::Symbol;

glib::wrapper! {
    pub struct SymbolListBoxRow(ObjectSubclass<imp::SymbolListBoxRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl SymbolListBoxRow {
    pub fn new(symbol: &Symbol) -> Self {
        glib::Object::builder().property("symbol", &symbol).build()
    }
}
