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

use crate::CurrencyLabel;
use crate::Symbol;
use adw::{prelude::*, subclass::prelude::*};
use gtk::glib::{self, clone, BindingFlags, ParamSpec, Properties, Value};

mod imp {
    use gtk::glib::once_cell::sync::OnceCell;

    use super::*;

    #[derive(Debug, Default, Properties, gtk::CompositeTemplate)]
    #[properties(wrapper_type = super::SymbolTrend)]
    #[template(resource = "/org/byteforge/stocks/symbol_trend.ui")]
    pub struct SymbolTrend {
        #[property(get, set, construct)]
        pub symbol: OnceCell<Symbol>,
        // Template widgets
        #[template_child]
        pub price: TemplateChild<CurrencyLabel>,
        #[template_child]
        pub change: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SymbolTrend {
        const NAME: &'static str = "StocksSymbolTrend";
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

            let symbol = self.obj().symbol();
            symbol
                .bind_property("currency", &self.price.get(), "currency")
                .build();

             symbol
                .bind_property("price", &self.price.get(), "amount")
                .build();

            symbol.connect_market_change_notify(clone!(@weak self as trend => @default-panic,
            move |symbol|{
                trend.change.remove_css_class("error");
                trend.change.remove_css_class("success");
                trend.price.remove_css_class("error");
                trend.price.remove_css_class("success");
                if symbol.market_change()>= 0.0 {
                    trend.change.add_css_class("success");
                    trend.price.add_css_class("success");
                }
                else {
                    trend.change.add_css_class("error");
                    trend.price.add_css_class("error");
                }
            }));
        }
    }
    impl BinImpl for SymbolTrend {}
    impl WidgetImpl for SymbolTrend {
        fn map(&self) {
            self.parent_map();

            let symbol = self.obj().symbol();

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
