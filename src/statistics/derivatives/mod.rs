// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `derivatives`: Calculate numerical derivatives of real data.
//!
//! For details on the math behind these algorithms, see the Typst document
//! `/docs/derivatives.typ`.

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
/// - Returns [`None`] if `index` or `index + 1` is out of bounds in `list`.
/// - Returns [`f64::INFINITY`] as the derivative if `t` at `index` is equal to `t` at `index + 1`.
#[must_use]
fn forward_difference_derivative<T: Float, F: Float>(
    index: usize,
    list: &[(T, F)],
) -> Option<(T, f64)> {
    let (t_1, f_1) = list.get(index)?;
    let (t_3, f_3) = list.get(index + 1)?;

    Some((
        T::new(t_1.get()),
        (f_3.get() - f_1.get()) / (t_3.get() - t_1.get()),
    ))
}

/// Calculates the backwards difference derivative. Returns `T` at `index` and the change in `F` over
/// `T` between `index - 1` and `index`.
///
/// Best used for the last item in a list. See [`central_difference_derivative`] for other items.
///
/// # Errors
///
/// - Returns [`None`] if `index` or `index - 1` is out of bounds in `list`.
/// - Returns [`f64::INFINITY`] as the derivative if `t` at `index` is equal to `t` at `index - 1`.
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
/// - Returns [`None`] if `index - 1` or `index + 1` is out of bounds in `list`.
/// - Returns [`f64::INFINITY`] as the derivative if `t` at `index - 1` is equal to `t` at
///   `index + 1`.
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

/// Calculates the numerical derivative of `F` with respect to `T`.
///
/// Assumes that the list is sorted by ascending `T` values (smallest first, largest last).
///
/// # Errors
///
/// - `list.len() < 2` returns an empty list.
/// - Overlapping `T` values will return [`f64::INFINITY`] as their derivative.
///
/// # Examples
///
/// Expected behavior:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// // `sin(t)` from `t = 0` to `t = 2`.
/// let list = (0..=10)
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
/// // The numerical derivative.
/// let result = derivatives::first_order(&list);
/// assert_eq!(result.len(), actual.len());
///
/// // Your data may have more or less error than this!
/// let accepted_error = 0.1;
/// // Assert that the two values are within `accepted_error` of each other.
/// let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
/// for i in 0..result.len() {
///     let (t, derivative) = actual[i];
///     let (result_t, result_derivative) = result[i];
///
///     // The independent variables should remain unchanged.
///     eq(t, 0.000_000_000_1, result_t, t);
///     // The derivatives should be within `accepted_error`.
///     eq(t, accepted_error, result_derivative, derivative);
/// }
/// ```
///
/// Overlapping values cause infinite derivatives:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// let list = &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)];
/// let result = derivatives::first_order(list);
/// assert_eq!(list.len(), result.len());
///
/// for (independent, derivative) in result {
///     assert_eq!(independent, 1.0);
///     assert!(derivative.is_infinite());
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

/// Calculates the nth numerical derivative of `F` with respect to `T`.
///
/// Assumes that the list is sorted by ascending `T` values (smallest first, largest last).
///
/// # Errors
///
/// - `list.len() < 2` returns an empty list.
/// - Overlapping `T` values will return unusual values.
///   - First-order derivatives will return [`f64::INFINITY`] as their derivative.
///   - Higher-order derivatives return a [`f64::NAN`] as their nth derivative.
///
/// # Examples
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// # use std::num::NonZeroU32;
/// #
/// // `sin(t)` from `t = 0` to `t = 2`.
/// let list = (0..=10)
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
/// // The numerical second derivative.
/// let result = derivatives::nth_order(second_order, &list);
/// assert_eq!(result.len(), actual.len());
///
/// dbg!(&actual, &result);
/// dbg!(derivatives::first_order(&list));
///
/// // Your data may have more or less error than this!
/// let mut accepted_error = 0.1;
/// // Assert that the two values are within `accepted_error` of each other.
/// let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
/// for i in 0..result.len() {
///     // The last two points have significantly higher error, unfortunately.
///     if i == result.len() - 2 {
///         accepted_error = 0.5;
///     }
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
///
/// Overlapping values cause infinite or NaN nth derivatives:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// # use std::num::NonZeroU32;
/// #
/// let list = &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)];
///
/// // Orders greater than one cause NaN.
/// let result = derivatives::nth_order(NonZeroU32::new(2).unwrap(), list);
/// assert_eq!(list.len(), result.len());
///
/// for (independent, derivative) in result {
///     assert_eq!(independent, 1.0);
///     assert!(derivative.is_nan());
/// }
///
/// // First derivatives cause infinity.
/// let result = derivatives::nth_order(NonZeroU32::new(1).unwrap(), list);
/// assert_eq!(list.len(), result.len());
///
/// for (independent, derivative) in result {
///     assert_eq!(independent, 1.0);
///     assert!(derivative.is_infinite());
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

/// Calculates the numerical derivative of `F` with respect to `T` at `index` using time-shifted
/// data points.
///
/// - Does not include the first or last data points.
/// - Assumes that the list is sorted by ascending `T` values (smallest first, largest last).
///
/// Traditional "rise over run" derivatives calculate the average derivative, at the center of a
/// time interval. This estimates the derivative at the _start_ of an interval. See
/// [`central_difference_derivative`] for a more traditional derivative without this time-shifting.
///
/// Here's the math written out as a Typst expression, calculating velocity from change in position
/// over time:
///
/// ```typst
/// $$$
///     v_2 &= (
///         v_("avg", 12) Delta t_23
///         + v_("avg", 23) Delta t_12
///     ) / (Delta t_13)\
///
///     v_2 &= (
///         (x_2 - x_1) / (t_2 - t_1) (t_3 - t_2)
///         + (x_3 - x_2) / (t_3 - t_2) (t_2 - t_1)
///     ) / (t_3 - t_1)\
/// $$$
/// ```
///
/// For details, see the Typst document `/docs/derivatives.typ`. It explains the math further and
/// derives it. The math is based on William Leonard's article "Dangers of Automated Data
/// Analysis," pub. _The Physics Teacher,_ vol. 35, April 1996, pp. 220-222.
///
/// # Errors
///
/// - Returns [`None`] if `index - 1`, or `index + 1` is out of bounds in `list`.
/// - Overlapping `T` values will return a [`f64::NAN`] as their derivative.
#[must_use]
fn derivative_time_shifted<T: Float, F: Float>(index: usize, list: &[(T, F)]) -> Option<(T, f64)> {
    let get = |index: usize| {
        let (t, f) = list.get(index)?;
        Some((t.get(), f.get()))
    };

    let (independent_1, dependent_1) = get(index.checked_sub(1)?)?;
    let (independent_2, dependent_2) = get(index)?;
    let (independent_3, dependent_3) = get(index + 1)?;

    let delta_independent_12 = independent_2 - independent_1;
    let delta_dependent_12 = dependent_2 - dependent_1;
    let derivative_avg_12 = delta_dependent_12 / delta_independent_12;

    let delta_independent_23 = independent_3 - independent_2;
    let delta_dependent_23 = dependent_3 - dependent_2;
    let derivative_avg_23 = delta_dependent_23 / delta_independent_23;

    let delta_independent_13 = independent_3 - independent_1;

    Some((
        T::new(independent_2.get()),
        derivative_avg_12.mul_add(
            delta_independent_23,
            derivative_avg_23 * delta_independent_12,
        ) / delta_independent_13,
    ))
}

/// Calculates the numerical derivative of `F` with respect to `T` using time-shifted data points.
///
/// - Does not include the first or last data points.
/// - Assumes that the list is sorted by ascending `T` values (smallest first, largest last).
///
/// Traditional "rise over run" derivatives calculate the average derivative, at the center of a
/// time interval. This estimates the derivative at the _start_ of an interval.
///
/// For details, see the Typst document `/docs/derivatives.typ`. It explains the math further and
/// derives it. The math is based on William Leonard's article "Dangers of Automated Data
/// Analysis," pub. _The Physics Teacher,_ vol. 35, April 1996, pp. 220-222.
///
/// # Errors
///
/// - Overlapping `T` values will return a [`f64::NAN`] as their derivative.
/// - `list.len() < 3` returns an empty list.
///
/// # Examples
///
/// Comparing with the actual derivative:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// // `sin(t)` from `t = 0` to `t = 2`.
/// let list = (0..=10)
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
/// // The numerical derivative.
/// let result = derivatives::first_order_time_shifted(&list);
/// assert_eq!(result.len(), actual.len() - 2);
///
/// // Your data may have more or less error than this!
/// let accepted_error = 0.0075;
/// // Assert that the two values are within `accepted_error` of each other.
/// let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
/// for i in 0..result.len() {
///     let (t, derivative) = actual[i + 1];
///     let (result_t, result_derivative) = result[i];
///
///     // The independent variables should remain unchanged.
///     eq(t, 0.000_000_000_1, result_t, t);
///     // The derivatives should be within `accepted_error`.
///     eq(t, accepted_error, result_derivative, derivative);
/// }
/// ```
///
/// Comparing it to the values calculated by Vernier's Logger Pro®, version 3.16.2:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// const EXPECTED: [(f64, f64); 13] = [
///     (0.2, 0.973_545_855_772),
///     (0.4, 0.914_932_856_5),
///     (0.6, 0.819_844_371_477),
///     (0.8, 0.692_071_278_532),
///     (1.0, 0.536_707_487_669),
///     (1.2, 0.359_946_862_951),
///     (1.4, 0.168_836_292_686),
///     (1.6, -0.029_005_247_775_7),
///     (1.8, -0.225_690_440_54),
///     (2.0, -0.413_378_067_647),
///     (2.2, -0.584_585_615_686),
///     (2.4, -0.732_487_579_995),
///     (2.6, -0.851_187_575_988),
/// ];
///
/// // `sin(t)` from `t = 0` to `t = 2.8`.
/// let list = (0..15)
///     .map(|i| {
///         let t = f64::from(i) * 0.2;
///         (t, t.sin())
///     })
///     .collect::<Box<_>>();
/// assert_eq!(list.len(), EXPECTED.len() + 2);
///
/// // The numerical derivative.
/// let result = derivatives::first_order_time_shifted(&list);
/// assert_eq!(result.len(), EXPECTED.len());
///
/// let effectively_equal = 0.000_000_000_1;
/// // Assert that the two values are within `error` of each other.
/// let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
/// for i in 0..result.len() {
///     let (t, derivative) = EXPECTED[i];
///     let (result_t, result_derivative) = result[i];
///
///     // The independent variables should remain unchanged.
///     eq(t, effectively_equal, result_t, t);
///     // The derivatives should be effectively equal.
///     eq(t, effectively_equal, result_derivative, derivative);
/// }
/// ```
///
/// Overlapping values cause NaN derivatives:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// let list = &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)];
/// let result = derivatives::first_order_time_shifted(list);
/// assert_eq!(list.len() - 2, result.len());
///
/// for (independent, derivative) in result {
///     assert_eq!(independent, 1.0);
///     assert!(derivative.is_nan());
/// }
/// ```
#[must_use]
#[expect(clippy::missing_panics_doc, reason = "see `expect` string")]
pub fn first_order_time_shifted<T: Float, F: Float>(list: &[(T, F)]) -> Box<[(T, f64)]> {
    if list.len() < 3 {
        return Box::default();
    }

    let mut derivative = Vec::with_capacity(list.len() - 2);

    // Skips the first and last index.
    for i in 1..(list.len() - 1) {
        derivative.push(
            derivative_time_shifted(i, list).expect("`1 < i < list.len() - 1`, this is safe"),
        );
    }

    derivative.into_boxed_slice()
}

/// Calculates the numerical second derivative of `F` with respect to `T` at `index` using
/// time-shifted data points.
///
/// Assumes that the list is sorted by ascending `T` values (smallest first, largest last).
///
/// This recognizes that traditional "rise over run" derivatives calculate the average derivative,
/// at the center of a time interval. It is a normal [`central_difference_derivative`] of the
/// [`first_order_time_shifted`] derivative, but running from the midpoint of `t_1` to `t_2`
/// and the midpoint of `t_2` to `t_3`, instead of from `t_1` to `t_3`.
///
/// Here's the math written out as a Typst expression, calculating acceleration from change in
/// velocity over time:
///
/// ```typst
/// $$$
///     a_2 &= 2 * (
///         v_("avg", 23)
///         - v_("avg", 12)
///     ) / (Delta t_13)\
///
///     a_2 &= 2 * (
///         (x_3 - x_2) / (t_3 - t_2)
///         - (x_2 - x_1) / (t_2 - t_1)
///     ) / (t_3 - t_1)\
/// $$$
/// ```
///
/// For details, see the Typst document `/docs/derivatives.typ`. It explains the math further and
/// derives it. The math is based on William Leonard's article "Dangers of Automated Data
/// Analysis," pub. _The Physics Teacher,_ vol. 35, April 1996, pp. 220-222.
///
/// # Errors
///
/// - Returns [`None`] if `index - 1` or `index + 1` is out of bounds in `list`.
/// - Overlapping `T` values will return a [`f64::NAN`] as their second derivative.
#[must_use]
fn second_derivative_time_shifted<T: Float, F: Float>(
    index: usize,
    list: &[(T, F)],
) -> Option<(T, f64)> {
    let get = |index: usize| {
        let (t, f) = list.get(index)?;
        Some((t.get(), f.get()))
    };

    let (independent_1, dependent_1) = get(index.checked_sub(1)?)?;
    let (independent_2, dependent_2) = get(index)?;
    let (independent_3, dependent_3) = get(index + 1)?;

    let derivative_avg_12 = (dependent_2 - dependent_1) / (independent_2 - independent_1);
    let derivative_avg_23 = (dependent_3 - dependent_2) / (independent_3 - independent_2);

    let delta_independent_13 = independent_3 - independent_1;

    Some((
        T::new(independent_2.get()),
        2.0 * (derivative_avg_23 - derivative_avg_12) / delta_independent_13,
    ))
}

/// Calculates the numerical second derivative of `F` with respect to `T` using time-shifted data
/// points.
///
/// - Does not include the first or last data points.
/// - Assumes that the list is sorted by ascending `T` values (smallest first, largest last).
///
/// This recognizes that traditional "rise over run" derivatives calculate the average derivative,
/// at the center of a time interval. This adjusts to calculated derivatives based on midpoints
/// instead of the start of intervals.
///
/// For details, see the Typst document `/docs/derivatives.typ`. It explains the math further and
/// derives it. The math is based on William Leonard's article "Dangers of Automated Data
/// Analysis," pub. _The Physics Teacher,_ vol. 35, April 1996, pp. 220-222.
///
/// # Errors
///
/// - `list.len() < 3` returns an empty list.
/// - Overlapping `T` values will return a [`f64::NAN`] as their second derivative.
///
/// # Examples
///
/// Comparing with the actual second derivative:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// // `sin(t)` from `t = 0` to `t = 2`.
/// let list = (0..=10)
///     .map(|i| {
///         let t = f64::from(i) * 0.2;
///         (t, t.sin())
///     })
///     .collect::<Box<_>>();
///
/// // `-sin(t)` (because `d^2/dx^2 sin(t) = -sin(t)`) from `t = 0` to `t = 2`.
/// let actual = list.iter().map(|(t, _)| (*t, -t.sin())).collect::<Box<_>>();
/// assert_eq!(actual.len(), list.len());
///
/// // The time-shifted numerical second derivative.
/// let result = derivatives::second_order_time_shifted(&list);
/// assert_eq!(result.len(), actual.len() - 2);
///
/// // Your data may have more or less error than this!
/// let accepted_error = 0.005;
/// // Assert that the two values are within `accepted_error` of each other.
/// let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
/// for i in 0..result.len() {
///     let (t, derivative) = actual[i + 1];
///     let (result_t, result_derivative) = result[i];
///
///     // The independent variables should remain unchanged.
///     eq(t, 0.000_000_000_1, result_t, t);
///     // The derivatives should be within `accepted_error`.
///     eq(t, accepted_error, result_derivative, derivative);
/// }
/// ```
///
/// Comparing it to the values calculated by Vernier's Logger Pro®, version 3.16.2:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// const EXPECTED: [(f64, f64); 13] = [
///     (0.2, -0.198_007_982_037),
///     (0.4, -0.388_122_010_68),
///     (0.6, -0.562_762_839_547),
///     (0.8, -0.714_968_089_903),
///     (1.0, -0.838_669_818_726),
///     (1.2, -0.928_936_428_452),
///     (1.4, -0.982_169_274_205),
///     (1.6, -0.996_246_130_409),
///     (1.8, -0.970_605_797_23),
///     (2.0, -0.906_270_473_839),
///     (2.2, -0.805_805_006_559),
///     (2.4, -0.673_214_636_531),
///     (2.6, -0.513_785_323_397),
/// ];
///
/// // `sin(t)` from `t = 0` to `t = 2.8`.
/// let list = (0..15)
///     .map(|i| {
///         let t = f64::from(i) * 0.2;
///         (t, t.sin())
///     })
///     .collect::<Box<_>>();
/// assert_eq!(list.len(), EXPECTED.len() + 2);
///
/// // The time-shifted numerical second derivative.
/// let result = derivatives::second_order_time_shifted(&list);
/// assert_eq!(result.len(), EXPECTED.len());
///
/// let effectively_equal = 0.000_000_000_1;
/// // Assert that the two values are within `error` of each other.
/// let eq = |t, error, a: f64, b: f64| assert!((a - b).abs() <= error, "{a} != {b} @ {t}");
///
/// for i in 0..result.len() {
///     let (t, derivative) = EXPECTED[i];
///     let (result_t, result_derivative) = result[i];
///
///     // The independent variables should remain unchanged.
///     eq(t, effectively_equal, result_t, t);
///     // The derivatives should be effectively equal.
///     eq(t, effectively_equal, result_derivative, derivative);
/// }
/// ```
///
/// Overlapping values cause NaN second derivatives:
///
/// ```rust
/// # use sciutil::statistics::derivatives;
/// #
/// let list = &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)];
/// let result = derivatives::second_order_time_shifted(list);
/// assert_eq!(list.len() - 2, result.len());
///
/// for (independent, derivative) in result {
///     assert_eq!(independent, 1.0);
///     assert!(derivative.is_nan());
/// }
/// ```
#[must_use]
#[expect(clippy::missing_panics_doc, reason = "see `expect` string")]
pub fn second_order_time_shifted<T: Float, F: Float>(list: &[(T, F)]) -> Box<[(T, f64)]> {
    if list.len() < 3 {
        return Box::default();
    }

    let mut derivative = Vec::with_capacity(list.len() - 2);

    // Skips the first and last index.
    for i in 1..(list.len() - 1) {
        derivative.push(
            second_derivative_time_shifted(i, list)
                .expect("`1 < i < list.len() - 1`, this is safe"),
        );
    }

    derivative.into_boxed_slice()
}
