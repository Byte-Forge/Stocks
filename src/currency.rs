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

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, glib::Enum)]
#[enum_type(name = "Currency")]
#[repr(i32)]
pub enum Currency {
    #[default]
    USD = 0,
    EUR = 1,
}
