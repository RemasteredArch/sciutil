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
