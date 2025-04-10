// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `rounding`: Facilities for rounding floating-point values.

pub mod digits;
#[cfg(test)]
mod test;

use digits::Digits;

use crate::units::{Float, UncertainFloat};

/// Rounds uncertainty to one or two significant figures and rounds the value to the same place,
/// returning them as a string with a plus minus notation.
///
/// When rounding the uncertainty, it looks for the first non-zero digit. If that digit is 1 or 2,
/// it rounds to the next digit if there is one. Otherwise, it rounds to this first digit. It then
/// rounds the value to the same place place (hundreds, tens, ones, tenths, hundredths, etc.) that
/// the uncertainty was rounded to.
///
/// # Examples
///
/// ```rust
/// # use sciutil::{
/// #     rounding,
/// #     units::{Float, Seconds, UncertainFloat},
/// # };
/// #
/// // Units are included if the [`UncertainFloat`] has them:
/// assert_eq!(
///     rounding::round_with_uncertainty(&UncertainFloat::new(
///         Seconds::new(1_024.051_123_125_5),
///         Seconds::new(0.015_555_312)
///     )),
///     "1024.051 s ± 0.016 s",
/// );
///
/// // Units are omitted if they aren't present, and the value will not get extra zeros if it ends
/// // before the uncertainty.
/// assert_eq!(
///     rounding::round_with_uncertainty(&UncertainFloat::new(1_024.05, 0.015_555_312)),
///     "1024.05 ± 0.016",
/// );
/// ```
#[must_use]
pub fn round_with_uncertainty<F: Float>(with_uncertainty: &UncertainFloat<F>) -> String {
    let unit = F::SYMBOL.map_or(String::new(), |u| format!(" {u}"));

    let value = Digits::new(with_uncertainty.value().get());
    let uncertainty = Digits::new(with_uncertainty.uncertainty().get());

    let last_place = uncertainty.last_sigificant_place();
    let uncertainty = uncertainty.round_to_place(last_place);
    let value = value.round_to_place(last_place);

    format!("{value}{unit} ± {uncertainty}{unit}")
}
