// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

pub mod digits;
#[cfg(test)]
mod test;

use digits::Digits;

#[must_use]
pub fn round_with_uncertainty(value: f64, uncertainty: f64, units: &str) -> String {
    let value = Digits::new(value);
    let uncertainty = Digits::new(uncertainty);

    let last_place = uncertainty.last_sigificant_place();
    let uncertainty = uncertainty.round_to_place(last_place);
    let value = value.round_to_place(last_place);

    format!("{value} {units} ± {uncertainty} {units}")
}
