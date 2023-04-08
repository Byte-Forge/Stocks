use glib::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
};
use std::cell::RefCell;

// The actual data structure that stores our values. This is not accessible
// directly from the outside.
#[derive(Default, Properties)]
#[properties(wrapper_type = super::Symbol)]
pub struct Symbol {
    #[property(get, set, construct_only)]
    symbol: RefCell<String>,
    #[property(get, set)]
    shortname: RefCell<String>,
    #[property(get, set)]
    longname: RefCell<String>,
    #[property(get, set)]
    price: RefCell<f64>,
    #[property(get, set)]
    market_change: RefCell<f64>,
}

// Basic declaration of our type for the GObject type system
#[glib::object_subclass]
impl ObjectSubclass for Symbol {
    const NAME: &'static str = "Symbol";
    type Type = super::Symbol;
}

// The ObjectImpl trait provides the setters/getters for GObject properties.
// Here we need to provide the values that are internally stored back to the
// caller, or store whatever new value the caller is providing.
//
// This maps between the GObject properties and our internal storage of the
// corresponding values of the properties.
impl ObjectImpl for Symbol {
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
