//! Defines the implementation of our model

use crate::symbol::Symbol;
use gio::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, ParamSpec, Properties, Value, clone, MainContext, PRIORITY_DEFAULT},
    prelude::*,
};
use std::cell::RefCell;
use yahoo_finance_api as yahoo;

#[derive(Properties)]
#[properties(wrapper_type = super::YahooFinanceModel)]
pub struct YahooFinanceModel {
    #[property(get, set)]
    search_text: RefCell<String>,
    pub(super) symbols: RefCell<Vec<Symbol>>,
    pub(super) provider: yahoo::YahooConnector,
}

/// Basic declaration of our type for the GObject type system
#[glib::object_subclass]
impl ObjectSubclass for YahooFinanceModel {
    const NAME: &'static str = "YahooFinanceModel";
    type Type = super::YahooFinanceModel;
    type Interfaces = (gio::ListModel,);

    fn new() -> Self {
        Self {
            search_text: RefCell::new(String::new()),
            symbols: RefCell::new(Vec::new()),
            provider: yahoo::YahooConnector::new(),
        }
    }
}

impl ObjectImpl for YahooFinanceModel {
    fn constructed(&self) {
        self.parent_constructed();

        //let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);
        let obj = self.obj();

        obj.connect_search_text_notify(move |model| {
            println!("Search text now is {}", model.search_text());

            let provider = &model.imp().provider;
            let search_text = model.imp().search_text.borrow().clone();

            // The main loop executes the asynchronous block#
            tokio::spawn(async move {
                let tickers = provider
                .search_ticker(&search_text)
                .await;
                for item in tickers.unwrap().quotes {
                    println!("{}", item.symbol)
                }
            });
        });
    }

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
