//! Defines the implementation of our model

use crate::symbol::Symbol;
use crate::currency::Currency;
use gio::subclass::prelude::*;
use gtk::{
    gio,
    glib::{self, clone, MainContext, ParamSpec, Properties, Value, PRIORITY_DEFAULT},
    prelude::*,
};
use std::cell::RefCell;
use std::sync::Arc;
use stocks_api;

#[derive(Properties)]
#[properties(wrapper_type = super::YahooFinanceModel)]
pub struct YahooFinanceModel {
    #[property(get, set)]
    search_text: RefCell<String>,
    pub(super) symbols: RefCell<Vec<Symbol>>,
    pub(super) provider: Arc<stocks_api::YahooFinanceAPI>,
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
            provider: Arc::new(stocks_api::YahooFinanceAPI::new()),
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

            let (sender, receiver) = MainContext::channel::<stocks_api::Quote>(PRIORITY_DEFAULT);
            let provider = self.provider.clone();
            let ticker = symbol.symbol();

            tokio::spawn(async move {
                let tickers = provider.get_quote(&ticker).await;
                sender
                    .send(tickers.expect("Failed to get latest quotes"))
                    .expect("Failed to send to channel");
            });

            receiver.attach(
                None,
                clone!(@weak symbol => @default-return Continue(false),
                            move |response| {
                                let quote = response;
                                symbol.set_price(quote.regular_market_price);
                                symbol.set_market_change(quote.regular_market_change);
                                let currency = quote.currency.unwrap().parse::<Currency>().unwrap();
                                symbol.set_currency(currency);
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
        let (sender, receiver) = MainContext::channel::<Vec<stocks_api::Symbol>>(PRIORITY_DEFAULT);

        obj.connect_search_text_notify(move |model| {
            println!("Search text now is {}", model.search_text());

            let sender = sender.clone();
            let provider = model.imp().provider.clone();
            let search_text = model.imp().search_text.borrow().clone();

            tokio::spawn(async move {
                let tickers = provider.search_symbols(&search_text).await;
                sender
                    .send(tickers.expect("Failed to search symbols"))
                    .expect("Failed to send to channel");
            });
        });

        // The main loop executes the closure as soon as it receives the message
        receiver.attach(
            None,
            clone!(@weak self as obj => @default-return Continue(false),
                        move |symbols| {
                            obj.obj().clear();
                            for item in symbols {
                                println!("{}", item.symbol);
                                    let symbol = Symbol::new(item.symbol.as_str());
                                if let Some(long_name) = item.long_name {
                                   symbol.set_longname(long_name);
                                }
                                if let Some(short_name) = item.short_name {
                                   symbol.set_shortname(short_name);
                                }
                                obj.obj().append(&symbol)
                            }
                            obj.update_symbols();
                            Continue(true)
                        }
            ),
        );
    }
}
