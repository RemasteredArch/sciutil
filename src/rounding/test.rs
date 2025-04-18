// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

use serde::{Deserialize, Serialize};

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

#[expect(clippy::cognitive_complexity, reason = "it's long, but simple")]
#[test]
fn digit_conversion() {
    /// For every provided [`Digit`] and [`char`], assert that converting the digit into a
    /// character returns the expected result.
    macro_rules! into_char {
        [ $($digit:ident, $char:expr);+ ; ] => {
            $( assert_eq!(char::from(Digit::$digit), $char) );+
        };
    }

    /// For every provided [`Digit`] and number, assert that converting the digit into a number
    /// returns the expected result, using [`Digit::get`], [`u8::from`], and [`u32::from`].
    macro_rules! into_num {
        [ $($digit:ident, $num:literal);+ ; ] => { $(
                assert_eq!(Digit::$digit.get(), $num);
                assert_eq!(u8::from(Digit::$digit), $num);
                assert_eq!(u32::from(Digit::$digit), $num);
        )+ };
    }

    /// For every provided value and [`Digit`], assert that [`Digit::try_from`] will return
    /// `Err(())` if `from == Err` or `Ok(from)` otherwise.
    macro_rules! try_from {
        [ $($from:expr, $digit:ident);+ ; ] => {
            $( try_from!(@ $from, $digit) );+
        };

        (@ $from:expr, Err) => {
            assert_eq!(Digit::try_from($from), Err(()))
        };

        (@ $from:expr, $digit:ident) => {
            assert_eq!(Digit::try_from($from), Ok(Digit::$digit))
        };
    }

    /// For every provided number and [`Digit`], assert that [`Digit::try_from`] will return
    /// `Err(())` if `from == Err` or `Ok(value)` otherwise, when treating `from` as both a [`u8`]
    /// and a [`u32`].
    macro_rules! try_from_num {
        [ $($from:literal, $digit:ident);+ ; ] => { $(
            try_from![$from as u8, $digit;];
            try_from![$from as u32, $digit;];
        )+ };
    }

    // Converts into characters correctly.
    into_char![
        Zero, '0';
        One, '1';
        Two, '2';
        Three, '3';
        Four, '4';
        Five, '5';
        Six, '6';
        Seven, '7';
        Eight, '8';
        Nine, '9';
    ];

    // Converts into numbers correctly.
    into_num![
        Zero, 0;
        One, 1;
        Two, 2;
        Three, 3;
        Four, 4;
        Five, 5;
        Six, 6;
        Seven, 7;
        Eight, 8;
        Nine, 9;
    ];

    try_from![
        // Digits convert from numeral characters successfully.
        '0', Zero;
        '1', One;
        '2', Two;
        '3', Three;
        '4', Four;
        '5', Five;
        '6', Six;
        '7', Seven;
        '8', Eight;
        '9', Nine;
        // Any other character does not.
        'a', Err;
        'b', Err;
        '\0', Err;
    ];

    try_from_num![
        // Digits convert from 0--9 successfully.
        0, Zero;
        1, One;
        2, Two;
        3, Three;
        4, Four;
        5, Five;
        6, Six;
        7, Seven;
        8, Eight;
        9, Nine;
        // Any other number does not.
        10, Err;
        11, Err;
        255, Err;
    ];
}

#[ignore = "benchmark, use `cargo bench -- --ignored -- bench_` to run"]
#[test]
fn bench_digit_slice_add() {
    const SLICE_9: DigitSlice<'_> = digit_slice!(9);
    const SLICE_0_9: DigitSlice<'_> = digit_slice!(0, 9);
    const SLICE_0_0_9: DigitSlice<'_> = digit_slice!(0, 0, 9);

    let box_102406 = digit_box![1, 0, 2, 4, 0, 6];
    let box_202405 = digit_box![2, 0, 2, 4, 0, 5];
    let box_10 = digit_box![1, 0];

    for _ in 0..150_000 {
        // Basic behavior.
        assert_eq!(SLICE_102405.add(1), box_102406);
        assert_eq!(SLICE_102405.add(100_000), box_202405);
        // The length of the digit slice grows as it needs to.
        assert_eq!(SLICE_9.add(1), box_10);
        // Will not grow if it does not need to.
        assert_eq!(SLICE_0_9.add(1), box_10);
        // Will shrink to the minimum length.
        assert_eq!(SLICE_0_0_9.add(1), box_10);
    }
}

/// Serialize `start` into JSON and check that it serialized into `expected_json`, then deserialize
/// it into a `T` and check that it deserialized back into `start`.
///
/// `str_buf` is the string that `start` is serialized into. It really shouldn't be a part of the
/// API, but I couldn't figure out another way to fix the lifetime errors from
/// `Deserialize<'de>`.
fn serialize_and_deserialize<'de, T>(str_buf: &'de mut String, start: &T, expected_json: &str)
where
    T: Deserialize<'de> + Serialize + std::fmt::Debug + Eq,
{
    let mut buf: Vec<u8> = Vec::new();
    start
        .serialize(&mut serde_json::Serializer::new(std::io::BufWriter::new(
            &mut buf,
        )))
        .expect("JSON serialization shouldn't (but could!) fail");

    *str_buf = String::from_utf8(buf)
        .expect("JSON serialization shouldn't (but could!) produce invalid UTF-8");
    assert_eq!(str_buf, expected_json);

    let deserialized: T = serde_json::from_slice(str_buf.as_bytes())
        .expect("a serialized `T` should be (but could not be!) valid for deserialization");
    assert_eq!(&deserialized, start);
}

#[test]
fn digits_de_re_serialize() {
    serialize_and_deserialize(
        &mut String::new(),
        &Digits::new(15.0),
        r#"{"sign":"Positive","dot":2,"digits":["One","Five"]}"#,
    );

    serialize_and_deserialize(
        &mut String::new(),
        &Digits::new(-0.0),
        r#"{"sign":"Negative","dot":1,"digits":["Zero"]}"#,
    );
}
