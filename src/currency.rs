/* currency.rs
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

use gtk::{glib};
use std::{str::FromStr, string::ToString};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, glib::Enum)]
#[enum_type(name = "Currency")]
#[repr(i32)]
pub enum Currency {
    #[default]
    USD = 0,
    EUR = 1,
    CAD = 2,
    MXN = 3,
    SEK = 4,
    GBP = 5,
    SGD = 6,
    ARS = 7,
    JPY = 8,
}

impl FromStr for Currency {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "usd" => Ok(Self::USD),
            "eur" => Ok(Self::EUR),
            "cad" => Ok(Self::CAD),
            "mxn" => Ok(Self::MXN),
            "sek" => Ok(Self::SEK),
            "gbp" => Ok(Self::GBP),
            "sgd" => Ok(Self::SGD),
            "ars" => Ok(Self::ARS),
            "jpy" => Ok(Self::JPY),
            _ => anyhow::bail!("Unsupported currency: {}", s),
        }
    }
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        match *self {
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::CAD => "CAD",
            Self::MXN => "MXN",
            Self::SEK => "SEK",
            Self::GBP => "GBP",
            Self::SGD => "SGD",
            Self::ARS => "ARS",
            Self::JPY => "JPY",
        }
        .to_string()
    }
}

impl Currency {
    pub fn to_symbol(&self) -> String {
        match *self {
            Self::USD => "$",
            Self::EUR => "€",
            Self::CAD => "Cad$",
            Self::MXN => "Mex$",
            Self::SEK => "skr",
            Self::GBP => "£",
            Self::SGD => "S$",
            Self::ARS => "Arg$",
            Self::JPY => "¥",
        }
        .to_string()
    }
}
