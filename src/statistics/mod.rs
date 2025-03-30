// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

use std::ops::Div;

use super::units::Float;

/// Computes the mean of a list of values.
///
/// # Panics
///
/// Panics if `list.len() == 0`.
///
/// # Examples
///
/// ```rust
/// # use sciutil::statistics::mean;
///
/// assert_eq!(mean(&[2.0, 3.0, 4.0]), 3.0);
/// assert_eq!(mean(&[2.0, 3.0, 5.0]), 3.3333333333333335);
/// ```
#[inline]
pub fn mean<F: Float>(list: &[F]) -> F {
    list.iter()
        .map(Float::get)
        .sum::<f64>()
        .div(list.len() as f64)
        .into()
}

/// Computes the corrected sample standard distribution of a list of values.
///
/// In a somewhat human-readable form, that's:
/// `sqrt( 1 / (list.len - 1) * sum_n (x[n] - mean(x))^2 )`.
///
/// In Typst, that's:
///
/// ```typst
/// $"stddev"(x) = sqrt( 1 / ("count"(x) - 1) sum_(n = 1)^"count"(x) (x_n - "mean"(x))^2 )$
/// ```
///
/// # Panics
///
/// Panics if `list.len() == 0`.
///
/// # Examples
///
/// ```rust
/// # use sciutil::statistics::stddev;
///
/// assert_eq!(stddev(&[2.0, 3.0, 4.0]), 1.0);
/// assert_eq!(stddev(&[10.0, 25.0, 50.0]), 20.207259421636902);
/// ```
#[inline]
pub fn stddev<F: Float>(list: &[F]) -> F {
    let mean = mean(list).get();

    (1.0 / ((list.len() - 1) as f64)
        * list
            .iter()
            .map(|value| (value.get() - mean).powi(2))
            .sum::<f64>())
    .sqrt()
    .into()
}
