// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

fn eq(lhs: f64, rhs: f64) {
    const ACCEPTED_ERROR: f64 = 10e-16;

    assert!((lhs - rhs).abs() < ACCEPTED_ERROR);
}

#[test]
fn forward_difference() {
    let list = &[(0.0, 1.0), (1.0, 3.0)];
    let derivative: f64 = (3.0 - 1.0) / (1.0 - 0.0);

    assert_eq!(
        super::forward_difference_derivative(0, list),
        Ok((0.0, derivative))
    );

    // Division by zero should return [`f64::INFINITY`].
    let (independent, derivative) =
        super::forward_difference_derivative(0, &[(1.0, 1.0), (1.0, 3.0)]).unwrap();
    eq(independent, 1.0);
    assert!(derivative.is_infinite());
}

#[test]
fn backward_difference() {
    let list = &[(0.0, 1.0), (1.0, 3.0)];
    let derivative: f64 = (3.0 - 1.0) / (1.0 - 0.0);

    assert_eq!(
        super::backward_difference_derivative(1, list),
        Ok((1.0, derivative))
    );

    // Division by zero should return [`f64::INFINITY`].
    let (independent, derivative) =
        super::backward_difference_derivative(1, &[(1.0, 1.0), (1.0, 3.0)]).unwrap();
    eq(independent, 1.0);
    assert!(derivative.is_infinite());
}

#[test]
fn central_difference() {
    let list = &[(0.0, 1.0), (1.0, 3.0), (2.0, 5.0)];
    let derivative: f64 = (5.0 - 1.0) / (2.0 - 0.0);

    assert_eq!(
        super::central_difference_derivative(1, list),
        Ok((1.0, derivative))
    );

    // Division by zero should return [`f64::INFINITY`].
    let (independent, derivative) =
        super::backward_difference_derivative(1, &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)]).unwrap();
    eq(independent, 1.0);
    assert!(derivative.is_infinite());
}

#[test]
fn time_shifted() {
    // Normal behavior:
    let list = &[(0.0, 1.0), (1.0, 3.0), (2.0, 5.0)];
    let derivative = ((5.0_f64 - 3.0) / (2.0 - 1.0))
        .mul_add(1.0 - 0.0, (3.0 - 1.0) / (1.0 - 0.0) * (2.0 - 1.0))
        / (2.0 - 0.0);

    assert_eq!(
        super::derivative_time_shifted(1, list),
        Ok((1.0, derivative))
    );

    // Overlapping values should cause NaN derivatives:
    let (independent, derivative) =
        super::derivative_time_shifted(1, &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)]).unwrap();
    dbg!(independent, derivative);

    eq(independent, 1.0);
    assert!(derivative.is_nan());
}

#[test]
fn second_time_shifted() {
    // Normal behavior:
    let list = &[(0.0, 1.0), (1.0, 3.0), (2.0, 5.0)];
    let derivative =
        2.0 * (
            (5.0_f64 - 3.0) / (2.0 - 1.0) // f'_(avg,23)
            - (3.0 - 1.0) / (1.0 - 0.0)
            // f'_(avg,12)
        ) / (2.0 - 0.0); // Delta t_13

    assert_eq!(
        super::second_derivative_time_shifted(1, list),
        Ok((1.0, derivative))
    );

    // Overlapping values should cause NaN derivatives:
    let (independent, derivative) =
        super::second_derivative_time_shifted(1, &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)]).unwrap();
    dbg!(independent, derivative);

    eq(independent, 1.0);
    assert!(derivative.is_nan());
}
