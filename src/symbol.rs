use glib::subclass::prelude::*;
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
};
use std::cell::RefCell;

mod imp {
    use super::*;

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

    #[glib::object_subclass]
    impl ObjectSubclass for Symbol {
        const NAME: &'static str = "Symbol";
        type Type = super::Symbol;
    }

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
}

glib::wrapper! {
    pub struct Symbol(ObjectSubclass<imp::Symbol>);
}

impl Symbol {
    pub fn new(symbol: &str) -> Symbol {
        glib::Object::builder().property("symbol", symbol).build()
    }
}
