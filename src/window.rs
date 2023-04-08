/* window.rs
 *
 * Copyright 2023 stephan
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

use adw::subclass::prelude::*;
use adw::{prelude::*, ActionRow};
use gtk::{
    gio,
    glib::{self, clone},
    prelude::*,
};

use crate::Symbol;
use crate::SymbolTrend;
use crate::YahooFinanceModel;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/byteforge/stocks/window.ui")]
    pub struct StocksWindow {
        // Template widgets
        #[template_child]
        pub yahoo_model: TemplateChild<YahooFinanceModel>,
        #[template_child]
        pub stocks_listbox: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub search_listbox: TemplateChild<gtk::ListBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StocksWindow {
        const NAME: &'static str = "StocksWindow";
        type Type = super::StocksWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            // Bind the private callbacks
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl StocksWindow {
        #[template_callback]
        fn handle_row_activated(&self, listbox: &gtk::ListBox, row: &gtk::ListBoxRow) {}

        #[template_callback]
        fn handle_leaflet_back(&self) {}
    }

    impl ObjectImpl for StocksWindow {
        fn constructed(&self) {
            // Call "constructed" on parent
            self.parent_constructed();
            self.search_listbox.bind_model(
                Some(&*self.yahoo_model),
                clone!(@weak self as window => @default-panic, move |item| {
                    let symbol = item.downcast_ref::<Symbol>()
                                     .expect("RowData is of wrong type");

                    let row = ActionRow::new();
                    row.set_title(&symbol.symbol());
                    row.set_subtitle(&symbol.longname());

                    let symbol_info = SymbolTrend::new(symbol);
                    row.add_suffix(&symbol_info);

                    row.upcast::<gtk::Widget>()
                }),
            );
        }
    }
    impl WidgetImpl for StocksWindow {}
    impl WindowImpl for StocksWindow {}
    impl ApplicationWindowImpl for StocksWindow {}
    impl AdwApplicationWindowImpl for StocksWindow {}
}

glib::wrapper! {
    pub struct StocksWindow(ObjectSubclass<imp::StocksWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl StocksWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
