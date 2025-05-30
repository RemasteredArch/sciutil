// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `display`: Miscellaneous facilities for pretty-printing things.

use std::fmt::{Display, Write};

/// Formats a list of values in a form that [Desmos](https://desmos.com/calculator) will accept as
/// a list variable.
///
/// Note that Desmos variable names can only be one character. If you want longer names, use
/// subscripts: `"b_{binding}"`.
///
/// This assumes that the elements of `list` produce values appropriate for Desmos and that
/// `variable_name` is a valid Desmos variable name.
///
/// # Examples
///
/// ```rust
/// # use sciutil::display::to_desmos_list;
/// #
/// assert_eq!(to_desmos_list("l", &[5, 6, 10]), "l = [5,6,10]");
/// assert_eq!(
///     to_desmos_list("m_{mass}", &[10.5, 202.0, 50.2001]),
///     "m_{mass} = [10.5,202,50.2001]",
/// );
/// ```
#[must_use]
pub fn to_desmos_list(variable_name: &str, list: &[impl Display]) -> String {
    let mut str = format!("{variable_name} = [");

    for value in list {
        write!(str, "{value},").expect("writing into a `String` should not fail");
    }
    if str.pop() == Some('[') {
        str.push('[');
    }

    str.push(']');

    str
}
