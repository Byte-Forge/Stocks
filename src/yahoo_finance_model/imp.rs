//! Defines the implementation of our model

use crate::symbol::Symbol;
use gio::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, clone, MainContext, ParamSpec, Properties, Value, PRIORITY_DEFAULT},
    prelude::*,
};
use std::cell::RefCell;
use std::sync::Arc;
use yahoo_finance_api as yahoo;

#[derive(Properties)]
#[properties(wrapper_type = super::YahooFinanceModel)]
pub struct YahooFinanceModel {
    #[property(get, set)]
    search_text: RefCell<String>,
    pub(super) symbols: RefCell<Vec<Symbol>>,
    pub(super) provider: Arc<yahoo::YahooConnector>,
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
            provider: Arc::new(yahoo::YahooConnector::new()),
        }
    }
}

impl ObjectImpl for YahooFinanceModel {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_search();
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

impl YahooFinanceModel {
    fn update_symbols(&self) {
        for i in 0..self.n_items() {
            let symbol = self.item(i).unwrap().downcast::<Symbol>().unwrap();

            let (sender, receiver) = MainContext::channel::<yahoo::YResponse>(PRIORITY_DEFAULT);
            let provider = self.provider.clone();
            let ticker = symbol.symbol();

            tokio::spawn(async move {
                let tickers = provider.get_latest_quotes(&ticker, "1d").await;
                sender
                    .send(tickers.expect("Failed to get latest quotes"))
                    .expect("Failed to send to channel");
            });

            receiver.attach(
                None,
                clone!(@weak symbol => @default-return Continue(false),
                            move |response| {
                                let quote = response.last_quote().unwrap();
                                symbol.set_price(quote.close);
                                Continue(true)
                            }
                ),
            );
        }
        /*for symbol in self.symbols.borrow().get()
        {
            self.provider.get_latest_quotes(&symbol.symbol(), "1d");
        }*/
    }

    fn setup_search(&self) {
        let obj = self.obj();
        let (sender, receiver) = MainContext::channel::<yahoo::YSearchResult>(PRIORITY_DEFAULT);

        obj.connect_search_text_notify(move |model| {
            println!("Search text now is {}", model.search_text());

            let sender = sender.clone();
            let provider = model.imp().provider.clone();
            let search_text = model.imp().search_text.borrow().clone();

            tokio::spawn(async move {
                let tickers = provider.search_ticker(&search_text).await;
                sender
                    .send(tickers.expect("Failed to search tickers"))
                    .expect("Failed to send to channel");
            });
        });

        // The main loop executes the closure as soon as it receives the message
        receiver.attach(
            None,
            clone!(@weak self as obj => @default-return Continue(false),
                        move |results| {
                            obj.obj().clear();
                            for item in results.quotes {
                                println!("{}", item.symbol);
                                let symbol = Symbol::new(item.symbol.as_str());
                                symbol.set_longname(item.long_name);
                                symbol.set_shortname(item.short_name);
                                obj.obj().append(&symbol)
                            }
                            obj.update_symbols();
                            Continue(true)
                        }
            ),
        );
    }
}
