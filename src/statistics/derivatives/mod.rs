// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `derivatives`: Calculate numeric derivatives of real data.

#[cfg(test)]
mod test;

use crate::units::Float;

use std::num::NonZeroU32;

/// Calculates the forward difference derivative. Returns `T` at `index` and the change in `F` over
/// `T` between `index` and `index + 1`.
///
/// Best used for the first item in a list. See [`central_difference_derivative`] for other items.
///
/// # Errors
///
/// Returns [`None`] if `index` or `index + 1` is out of bounds in `list`.
#[must_use]
fn forward_difference_derivative<T: Float, F: Float>(
    index: usize,
    list: &[(T, F)],
) -> Option<(T, f64)> {
    let (t_1, f_1) = list.get(index)?;
    let (t_2, f_2) = list.get(index + 1)?;

    Some((
        T::new(t_1.get()),
        (f_2.get() - f_1.get()) / (t_2.get() - t_1.get()),
    ))
}

/// Calculates the backwards difference derivative. Returns `T` at `index` and the change in `F` over
/// `T` between `index - 1` and `index`.
///
/// Best used for the last item in a list. See [`central_difference_derivative`] for other items.
///
/// # Errors
///
/// Returns [`None`] if `index` or `index - 1` is out of bounds in `list`.
#[must_use]
fn backward_difference_derivative<T: Float, F: Float>(
    index: usize,
    list: &[(T, F)],
) -> Option<(T, f64)> {
    let (t_1, f_1) = list.get(index.checked_sub(1)?)?;
    let (t_2, f_2) = list.get(index)?;

    Some((
        T::new(t_2.get()),
        (f_2.get() - f_1.get()) / (t_2.get() - t_1.get()),
    ))
}

/// Calculates the central difference derivative. Returns `T` at `index` and the change in `F`
/// over `T` between `index - 1` and `index + 1`.
///
/// Best used for the middle items in a list. For the first item in a list, see
/// [`forward_difference_derivative`]. For the last item in a list, see
/// [`backward_difference_derivative`].
///
/// # Errors
///
/// Returns [`None`] if `index - 1` or `index + 1` is out of bounds in `list`.
#[must_use]
fn central_difference_derivative<T: Float, F: Float>(
    index: usize,
    list: &[(T, F)],
) -> Option<(T, f64)> {
    let (t_1, f_1) = list.get(index.checked_sub(1)?)?;
    let (t_2, f_2) = list.get(index + 1)?;

    let (t_middle, _) = list.get(index)?;

    Some((
        T::new(t_middle.get()),
        (f_2.get() - f_1.get()) / (t_2.get() - t_1.get()),
    ))
}

/// Calculates the numerical derivative of [`F`] with respect to [`T`].
///
/// # Examples
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// // `sin(t)` from `t = 0` to `t = 2`.
/// let list = (0..10)
///     .map(|i| {
///         let t = f64::from(i) * 0.2;
///         (t, t.sin())
///     })
///     .collect::<Box<_>>();
///
/// // `cos(t)` (because `d/dx sin(t) = cos(t)`) from `t = 0` to `t = 2`.
/// let actual = list.iter().map(|(t, _)| (*t, t.cos())).collect::<Box<_>>();
/// assert_eq!(actual.len(), list.len());
///
/// // The numeric derivative.
/// let result = derivatives::first_order(&list);
/// assert_eq!(result.len(), actual.len());
///
/// // Your data may have more error than this!
/// let accepted_error = 0.1;
///
/// for i in 0..result.len() {
///     // Assert that the two values are within `accepted_error` of each other.
///     let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
///     let (t, derivative) = actual[i];
///     let (result_t, result_derivative) = result[i];
///
///     // The independent variables should remain unchanged.
///     eq(t, 0.000_000_000_1, result_t, t);
///     // The derivatives should be within `accepted_error`.
///     eq(t, accepted_error, result_derivative, derivative);
/// }
/// ```
#[must_use]
#[expect(clippy::missing_panics_doc, reason = "see `expect` string")]
pub fn first_order<T: Float, F: Float>(list: &[(T, F)]) -> Box<[(T, f64)]> {
    if list.len() < 2 {
        return Box::default();
    }

    let mut derivative = Vec::with_capacity(list.len());

    derivative.push(
        forward_difference_derivative(0, list).expect("`len >= 2`, indices `0` and `1` exist"),
    );

    for index in 1..(list.len() - 1) {
        derivative.push(
            central_difference_derivative(index, list)
                .expect("`0 < index < len - 1`, indices `index - 1` and `index + 1` exist"),
        );
    }

    derivative.push(
        backward_difference_derivative(list.len() - 1, list)
            .expect("`len >= 2`, `len - 1` and `len - 2` exist"),
    );

    derivative.into_boxed_slice()
}

/// Calculates the nth numerical derivative of [`F`] with respect to [`T`].
///
/// # Examples
///
/// ```rust
/// # use std::num::NonZeroU32;
/// # use sciutil::statistics::derivatives;
/// #
/// // TODO: not actually to 2, this is to 1.8
/// // `sin(t)` from `t = 0` to `t = 2`.
/// let list = (0..10)
///     .map(|i| {
///         let t = f64::from(i) * 0.2;
///         (t, t.sin())
///     })
///     .collect::<Box<_>>();
///
/// // `-sin(t)` (because `d^2/dx^2 sin(t) = -sin(t)`) from `t = 0` to `t = 2`.
/// let actual = list.iter().map(|&(t, f)| (t, -f)).collect::<Box<_>>();
/// assert_eq!(actual.len(), list.len());
///
/// let second_order = NonZeroU32::new(2).expect("`2 > 0`");
/// // The numeric second derivative.
/// let result = derivatives::nth_order(second_order, &list);
/// assert_eq!(result.len(), actual.len());
///
/// dbg!(&actual, &result);
/// dbg!(derivatives::first_order(&list));
///
/// // Your data may have more error than this!
/// let accepted_error = 0.3;
///
/// for i in 0..result.len() {
///     // Assert that the two values are within `accepted_error` of each other.
///     let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
///     let (t, derivative) = actual[i];
///     let (result_t, result_derivative) = result[i];
///
///     // The independent variables should remain unchanged.
///     eq(t, 0.000_000_000_1, result_t, t);
///     // The derivatives should be within `accepted_error`.
///     eq(t, accepted_error, result_derivative, derivative);
/// }
/// ```
#[must_use]
pub fn nth_order<T: Float, F: Float>(order: NonZeroU32, list: &[(T, F)]) -> Box<[(T, f64)]> {
    let mut derivative: Box<_> = first_order(list);

    for _ in 2..=order.get() {
        derivative = first_order(&derivative);
    }

    derivative
}

// ```
// /// Calculates the central difference derivative approximation of
// #[must_use]
// pub fn first_order_weighted<T: Float, F: Float>(
//     list: &[(T, F)],
//     surrounding_points: Option<usize>,
// ) -> Box<[(T, f64)]> {
//     let surrounding_points = surrounding_points.unwrap_or(7);
//
//     for (index, point) in list.iter().enumerate() {}
//
//     todo!()
// }
//
// #[must_use]
// pub fn second_order_weighted<T: Float, F: Float>(
//     list: &[(T, F)],
//     surrounding_points: Option<usize>,
// ) -> Box<[(T, f64)]> {
//     todo!()
// }
//
// #[must_use]
// pub fn nth_order_weighted<T: Float, F: Float>(
//     order: NonZeroU32,
//     list: &[(T, F)],
//     surrounding_points: Option<usize>,
// ) -> Box<[(T, f64)]> {
//     todo!()
// }
// ```
