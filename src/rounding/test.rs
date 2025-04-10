// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

use super::digits::{Digit, DigitSlice, Digits, Sign};

macro_rules! digit {
    ($digit:expr) => {
        match $digit {
            0 => Digit::Zero,
            1 => Digit::One,
            2 => Digit::Two,
            3 => Digit::Three,
            4 => Digit::Four,
            5 => Digit::Five,
            6 => Digit::Six,
            7 => Digit::Seven,
            8 => Digit::Eight,
            9 => Digit::Nine,
            _ => panic!("invalid macro input, expected digit"),
        }
    };
}
macro_rules! digit_slice {
    [ $($digit:expr),+ ] => {
        DigitSlice::new(const {
            &[ $(digit!($digit)),+ ]
        })
    };
}

macro_rules! digit_box {
    [ $($digit:expr),+ ] => {
        const { [$(
            digit!($digit)
        ),+] }.to_vec().into_boxed_slice()
     };
}

macro_rules! digits {
    ($sign:ident, $dot:expr, [$($digits:expr),+]) => {
        unsafe { Digits::from_raw_parts(Sign::$sign, $dot, digit_box![$($digits),+]) }
    };
}

/// A [`DigitSlice`] representing `1024.05`.
const SLICE_102405: DigitSlice = digit_slice![1, 0, 2, 4, 0, 5];

#[test]
fn check_macros() {
    assert_eq!(
        SLICE_102405,
        DigitSlice::new(&[
            Digit::One,
            Digit::Zero,
            Digit::Two,
            Digit::Four,
            Digit::Zero,
            Digit::Five
        ]),
    );

    assert_eq!(
        digit_box![1, 0, 2, 4, 0, 5],
        [
            Digit::One,
            Digit::Zero,
            Digit::Two,
            Digit::Four,
            Digit::Zero,
            Digit::Five
        ]
        .to_vec()
        .into_boxed_slice(),
    );
}

#[test]
fn digit_slice_add() {
    // Basic behavior.
    assert_eq!(SLICE_102405.add(1), digit_box![1, 0, 2, 4, 0, 6]);
    assert_eq!(SLICE_102405.add(100_000), digit_box![2, 0, 2, 4, 0, 5]);
    // The length of the digit slice grows as it needs to.
    assert_eq!(digit_slice!(9).add(1), digit_box![1, 0]);
    // Will not grow if it does not need to.
    assert_eq!(digit_slice!(0, 9).add(1), digit_box![1, 0]);
    // Will shrink to the minimum length.
    assert_eq!(digit_slice!(0, 0, 9).add(1), digit_box![1, 0]);
}

#[test]
fn to_from_digit_slice() {
    assert_eq!(u32::from(SLICE_102405), 102_405);
}

#[test]
fn to_digits() {
    let digits_1024 = digits!(Positive, 4, [1, 0, 2, 4]);
    let digits_102405 = digits!(Positive, 4, [1, 0, 2, 4, 0, 5]);
    let digits_zero = digits!(Positive, 1, [0]);
    let digits_neg_zero = digits!(Negative, 1, [0]);
    let digits_point_one_three = digits!(Positive, 1, [0, 0, 3]);

    assert_eq!(Digits::new(1024.0).to_string(), "1024");
    assert_eq!(Digits::new(1024.0), digits_1024);
    assert_eq!(Digits::new(1024.05), digits_102405);
    assert_eq!(Digits::new(0.0), digits_zero);
    assert_eq!(Digits::new(-0.0), digits_neg_zero);
    assert_eq!(Digits::new(0.03), digits_point_one_three);
}

#[test]
fn digits_to_string() {
    let tests = [
        (digits!(Positive, 4, [1, 0, 2, 4, 0, 5]), "1024.05"),
        (digits!(Positive, 4, [1, 0, 2, 4]), "1024"),
        (digits!(Positive, 1, [0]), "0"),
        (digits!(Negative, 1, [0]), "-0"),
        (digits!(Positive, 1, [0, 0, 3]), "0.03"),
    ];

    for (digits, expected) in tests {
        assert_eq!(digits.to_string(), expected);
    }
}

#[test]
fn last_sigificant_digit() {
    // rounding::round_with_uncertainty(1024.05, 0.015555312, "g")
    let digits_1024 = digits!(Positive, 4, [1, 0, 2, 4, 0, 5]);
    let digits_001 = digits!(Positive, 1, [0, 0, 1, 5, 5, 5, 5, 3, 1, 2]);

    assert_eq!(digits_001.last_significant_digit(), 3);
    assert_eq!(digits_1024.last_significant_digit(), 1);
}

#[test]
fn round_to() {
    let digits_102405 = digits!(Positive, 4, [1, 0, 2, 4, 0, 5]);
    let digits_00155 = digits!(Positive, 1, [0, 0, 1, 5, 5, 5, 5, 3, 1, 2]);
    let digits_00006 = digits!(Positive, 1, [0, 0, 0, 0, 6]);
    let digits_06 = digits!(Positive, 1, [0, 6]);

    // ```txt
    // 0.015555312
    //     ^
    // 0.016
    // ```
    assert_eq!(
        digits_00155.round_to_digit(3),
        digits!(Positive, 1, [0, 0, 1, 6])
    );
    // ```txt
    // 0.015555312
    //   ^
    // 0.0
    // ```
    assert_eq!(digits_00155.round_to_digit(1), digits!(Positive, 1, [0, 0]));
    // ```txt
    // 0.0006
    //   ^
    // 0.0
    // ```
    assert_eq!(digits_00006.round_to_digit(1), digits!(Positive, 1, [0, 0]));
    // ```txt
    // 0.0006
    //     ^
    // 0.001
    // ```
    assert_eq!(
        digits_00006.round_to_digit(3),
        digits!(Positive, 1, [0, 0, 0, 1])
    );
    // ```txt
    // 1024.05
    //  ^
    // 1000
    // ```
    assert_eq!(
        digits_102405.round_to_digit(1),
        digits!(Positive, 4, [1, 0, 0, 0])
    );
    // ```txt
    // 1024.05
    //      ^
    // 1024.0
    // ```
    assert_eq!(
        digits_102405.round_to_digit(4),
        digits!(Positive, 4, [1, 0, 2, 4, 0])
    );
    // ```txt
    // 1024.05
    //    ^
    // 1024
    // ```
    assert_eq!(
        digits_102405.round_to_digit(3),
        digits!(Positive, 4, [1, 0, 2, 4])
    );
    // ```txt
    // 0.6
    // ^
    // 1
    // ```
    assert_eq!(digits_06.round_to_digit(0), digits!(Positive, 1, [1]));
    // ```txt
    // 0.6
    //   ^
    // 0.6
    // ```
    assert_eq!(digits_06.round_to_digit(1), digits_06);
}
