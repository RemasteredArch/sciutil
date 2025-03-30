// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

use super::{
    super::rounding,
    digits::{DigitSlice, Digits, Sign},
};
use digits::*;

mod digits {
    macro_rules! digits {
        [$(
            $name:ident, $digit:expr;
        )+] => {$(
           pub const $name: super::super::digits::Digit =
                unsafe { super::super::digits::Digit::new_unchecked($digit) };
        )+};
    }

    digits![
        ZERO, 0;
        ONE, 1;
        TWO, 2;
        THREE, 3;
        FOUR, 4;
        FIVE, 5;
        SIX, 6;
        SEVEN, 7;
        EIGHT, 8;
        NINE, 9;
    ];
}

macro_rules! digit {
    ($digit:expr) => {
        match $digit {
            0 => ZERO,
            1 => ONE,
            2 => TWO,
            3 => THREE,
            4 => FOUR,
            5 => FIVE,
            6 => SIX,
            7 => SEVEN,
            8 => EIGHT,
            9 => NINE,
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
        DigitSlice::new(&[ONE, ZERO, TWO, FOUR, ZERO, FIVE]),
    );

    assert_eq!(
        digit_box![1, 0, 2, 4, 0, 5],
        [ONE, ZERO, TWO, FOUR, ZERO, FIVE]
            .to_vec()
            .into_boxed_slice(),
    );
}

#[test]
fn digit_slice_add() {
    assert_eq!(SLICE_102405.add(1), digit_box![1, 0, 2, 4, 0, 6]);
    assert_eq!(SLICE_102405.add(100_000), digit_box![2, 0, 2, 4, 0, 5]);
    assert_eq!(digit_slice!(9).add(1), digit_box![1, 0]);
    assert_eq!(digit_slice!(0, 9).add(1), digit_box![1, 0]);
}

#[test]
fn to_from_digit_slice() {
    assert_eq!(u32::from(SLICE_102405), 102405);
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

    assert_eq!(digits_001.last_sigificant_digit(), 3);
    assert_eq!(digits_1024.last_sigificant_digit(), 2);
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
    // 1000.0
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

#[test]
fn round_with_uncertainty() {
    assert_eq!(
        rounding::round_with_uncertainty(1024.05, 0.015555312, "g"),
        "1024.05 g ± 0.016 g"
    );
    assert_eq!(
        rounding::round_with_uncertainty(1024.0511231255, 0.015555312, "g"),
        "1024.051 g ± 0.016 g"
    )
}
