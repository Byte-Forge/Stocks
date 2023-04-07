//! Defines the implementation of our model

use gio::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
    gio
};
use std::cell::RefCell;

use crate::symbol::Symbol;

#[derive(Default, Properties)]
#[properties(wrapper_type = super::Symbol)]
pub struct YahooFinanceModel
{
    #[property(get, set)]
    search_text: RefCell<Option<String>>,
    pub(super) symbols: RefCell<Vec<Symbol>>
}

/// Basic declaration of our type for the GObject type system
#[glib::object_subclass]
impl ObjectSubclass for YahooFinanceModel {
    const NAME: &'static str = "YahooFinanceModel";
    type Type = super::YahooFinanceModel;
    type Interfaces = (gio::ListModel,);
}

impl ObjectImpl for YahooFinanceModel {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }
}

impl ListModelImpl for YahooFinanceModel {
    fn item_type(&self) -> glib::Type {
        Symbol::static_type()
    }
    fn n_items(&self) -> u32 {
        self.symbols.borrow().len() as u32
    }
    fn item(&self, position: u32) -> Option<glib::Object> {
        self.symbols
            .borrow()
            .get(position as usize)
            .map(|o| o.clone().upcast::<glib::Object>())
    }
}
