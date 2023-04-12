/* currency_label.rs
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
use adw::{prelude::*, subclass::prelude::*};
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
    subclass::prelude::*,
};
use std::cell::{Cell, RefCell};

use crate::currency::Currency;

mod imp {
    use super::*;
    #[derive(Default, Properties, Debug)]
    #[properties(wrapper_type = super::CurrencyLabel)]
    pub struct CurrencyLabel {
        #[property(get, set)]
        amount: Cell<f64>,
        #[property(get, set, builder(Currency::USD))]
        currency: Cell<Currency>,
        #[property(get)]
        label: RefCell<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CurrencyLabel {
        const NAME: &'static str = "CurrencyLabel";
        type ParentType = adw::Bin;
        type Type = super::CurrencyLabel;
    }

    impl ObjectImpl for CurrencyLabel {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }
        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "currency" => {
                    let currency = value
                        .get()
                        .expect("The value needs to be of type `Currency`.");
                    self.currency.replace(currency);
                    self.update_label();
                },
                "amount" => {
                    let amount = value.get().expect("The value needs to be of type `f64`.");
                    self.amount.replace(amount);
                    self.update_label();
                },
                _ => unimplemented!(),
            }
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();

            self.obj().set_child(Some(&self.obj().label()));
        }
    }

    impl BinImpl for CurrencyLabel {}
    impl WidgetImpl for CurrencyLabel {}

    impl CurrencyLabel {
        fn update_label(&self) {
            let mut result = format!("{:.2}", self.obj().amount());
            match self.obj().currency() {
                Currency::USD => result.push_str("$"),
                Currency::EUR => result.push_str("€"),
                _ => unimplemented!(),
            }
            self.obj().label().set_text(&result);
        }
    }
}

glib::wrapper! {
  pub struct CurrencyLabel(ObjectSubclass<imp::CurrencyLabel>)
      @extends gtk::Widget, adw::Bin;
}

impl CurrencyLabel {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
