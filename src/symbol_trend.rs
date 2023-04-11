/* window.rs
 *
 * Copyright 2023 Stephan Vedder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::Symbol;
use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, BindingFlags, ParamSpec, Properties, Value};
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::SymbolTrend)]
    #[template(resource = "/org/byteforge/stocks/symbol_trend.ui")]
    pub struct SymbolTrend {
        #[property(get, set, construct_only)]
        symbol: RefCell<Option<Symbol>>,
        // Template widgets
        #[template_child]
        pub price: TemplateChild<gtk::Label>,
        #[template_child]
        pub change: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SymbolTrend {
        const NAME: &'static str = "SymbolTrend";
        type Type = super::SymbolTrend;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SymbolTrend {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
        }
    }
    impl BinImpl for SymbolTrend {}
    impl WidgetImpl for SymbolTrend {
        fn map(&self) {
            self.parent_map();

            let symbol = self.obj().symbol().unwrap();

            symbol
                .bind_property("price", &self.price.get(), "label")
                .transform_to(|_, price: f64| {
                    println!("Price: {:.2}", price);
                    Some(format!("{:.2}", price))
                })
                .flags(BindingFlags::DEFAULT | BindingFlags::SYNC_CREATE)
                .build();

            symbol
                .bind_property("market_change", &self.change.get(), "label")
                .transform_to(|_, change: f64| {
                    println!("Change: {:.2}%", change);
                    Some(format!("{:.2}%", change))
                })
                .flags(BindingFlags::DEFAULT | BindingFlags::SYNC_CREATE)
                .build();
        }
    }
}

glib::wrapper! {
    pub struct SymbolTrend(ObjectSubclass<imp::SymbolTrend>)
        @extends gtk::Widget, adw::Bin;
}

impl SymbolTrend {
    pub fn new(symbol: &Symbol) -> Self {
        glib::Object::builder().property("symbol", symbol).build()
    }
}
