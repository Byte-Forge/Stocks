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
use gtk::{
    glib::{self, ParamSpec, Properties, Value},
    prelude::*,
    subclass::prelude::*,
};
use std::cell::Cell;

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
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CurrencyLabel {
        const NAME: &'static str = "CurrencyLabel";
        type ParentType = gtk::Widget;
        type Type = super::CurrencyLabel;
    }

    impl ObjectImpl for CurrencyLabel {
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
            self.parent_constructed();
            let obj = self.obj();
        }
    }

    impl WidgetImpl for CurrencyLabel {}
    impl ListBoxRowImpl for CurrencyLabel {}
}

glib::wrapper! {
  pub struct CurrencyLabel(ObjectSubclass<imp::CurrencyLabel>)
      @extends gtk::Widget;
}

impl CurrencyLabel {
    pub fn new() -> Self {
        glib::Object::new()
    }
}
