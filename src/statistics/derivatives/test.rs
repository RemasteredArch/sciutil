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
        Some((0.0, derivative))
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
        Some((1.0, derivative))
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
        Some((1.0, derivative))
    );

    // Division by zero should return [`f64::INFINITY`].
    let (independent, derivative) =
        super::backward_difference_derivative(1, &[(1.0, 1.0), (1.0, 3.0), (1.0, 5.0)]).unwrap();
    eq(independent, 1.0);
    assert!(derivative.is_infinite());
}
