// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `digits`: Facilities for handling floating-point values as lists of base-ten digits.
//!
//! In particular, see [`Digits`], which holds the implementation details for
//! [`super::round_with_uncertainty`].

use std::{
    fmt::Display,
    num::{FpCategory, NonZeroIsize},
};

/// Represents whether a number is positive or negative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Self::Positive => "",
            Self::Negative => "-",
        };

        write!(f, "{as_str}",)
    }
}

/// Represents a base-ten digit, from 0--9.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Digit {
    /// The minimum possible value of [`Self`] as a [`u8`].
    pub const MIN: u8 = 0;
    /// The maximum possible value of [`Self`] as a [`u8`].
    pub const MAX: u8 = 9;

    /// Creates a new [`Self`], checking that it is valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::Digit;
    /// #
    /// assert_eq!(Digit::new(0), Some(Digit::Zero));
    /// assert_eq!(Digit::new(9), Some(Digit::Nine));
    /// assert!(Digit::new(10).is_none());
    /// ```
    #[must_use]
    pub const fn new(digit: u8) -> Option<Self> {
        Some(match digit {
            0 => Self::Zero,
            1 => Self::One,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            _ => return None,
        })
    }

    /// Gets [`Self`] as a [`u8`].
    #[must_use]
    pub const fn get(&self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
        }
    }
}

impl TryFrom<u8> for Digit {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(())
    }
}

impl TryFrom<u32> for Digit {
    type Error = ();

    fn try_from(digit: u32) -> Result<Self, Self::Error> {
        u8::try_from(digit).map_err(|_| ())?.try_into()
    }
}

impl TryFrom<char> for Digit {
    type Error = ();

    fn try_from(digit: char) -> Result<Self, Self::Error> {
        // `to_digit(10)` will return a number from 0-9, so it is safe to cast to [`u8`] and
        // blindly construct [`Self`] with unwrap.
        #[expect(clippy::cast_possible_truncation, reason = "see comment")]
        Ok(Self::new(digit.to_digit(10).ok_or(())? as u8).unwrap())
    }
}

impl From<Digit> for char {
    #[must_use]
    fn from(digit: Digit) -> Self {
        const ASCII_ZERO: u8 = 0b0011_0000;

        // - `0b0011_0000` -> `'0'`
        // - `0b0011_0001` -> `'1'`
        // - `0b0011_0010` -> `'2'`
        // - Etc.
        (ASCII_ZERO + digit.get()) as Self
    }
}

impl From<Digit> for u8 {
    #[must_use]
    fn from(digit: Digit) -> Self {
        digit.get()
    }
}

impl From<Digit> for u32 {
    #[must_use]
    fn from(digit: Digit) -> Self {
        digit.get().into()
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

/// Represents an unsigned integer number as a shared slice of [`Digit`]s.
///
/// Mostly intended for use in intermediate steps when working with [`Digits`], most notably for
/// [`Self::add`].
///
/// # Examples
///
/// ```rust
/// # use sciutil::rounding::digits::{Digit, DigitSlice};
/// #
/// let ten = DigitSlice::new(&[Digit::One, Digit::Zero]);
///
/// assert_eq!(u32::from(ten), 10);
/// assert_eq!(ten.add(1), [Digit::One, Digit::One].to_vec().into_boxed_slice());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitSlice<'a>(&'a [Digit]);

impl<'a> DigitSlice<'a> {
    /// Constructs a new instance of [`Self`] from a slice of [`Digit`]s.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digit, DigitSlice};
    /// #
    /// let slice = &[Digit::One, Digit::Zero];
    /// assert_eq!(DigitSlice::new(slice).get(), slice);
    /// ```
    #[must_use]
    pub const fn new(digits: &'a [Digit]) -> Self {
        Self(digits)
    }

    /// Treats [`Self`] as a [`u32`], adds another [`u32`], then converts back to a (boxed) slice
    /// of [`Digit`]s. This may cause the slice to grow or shrink in length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digit, DigitSlice};
    /// #
    /// // `9`
    /// let nine = DigitSlice::new(&[Digit::Nine]);
    /// // `009`
    /// let zero_zero_nine = DigitSlice::new(&[Digit::Zero, Digit::Zero, Digit::Nine]);
    /// // `10`
    /// let ten = DigitSlice::new(&[Digit::One, Digit::Zero]).into_boxed();
    ///
    /// // The length of the digit slice grows as it needs to (`9` -> `10`).
    /// assert_eq!(nine.add(1), ten);
    ///
    /// // Does not maintain any leading zeros (`009` -> `10`).
    /// assert_eq!(zero_zero_nine.add(1), ten);
    /// ```
    #[expect(clippy::missing_panics_doc, reason = "see `expect` string")]
    #[must_use]
    pub fn add(&self, value: u32) -> Box<[Digit]> {
        // This could be more efficient, but whatever.
        (u32::from(self) + value)
            .to_string()
            .chars()
            .map(|c| Digit::try_from(c).expect("`u32::to_string` will only produce digits 0-9"))
            .collect()
    }

    /// Gets the internal slice representation of [`Self`].
    #[must_use]
    pub const fn get(&self) -> &'a [Digit] {
        self.0
    }

    /// Converts [`Self`] to a boxed slice of [`Digit`]s.
    #[must_use]
    pub fn into_boxed(self) -> Box<[Digit]> {
        self.0.to_vec().into_boxed_slice()
    }
}

impl From<&DigitSlice<'_>> for u32 {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "I've never seen the number of digits in an `f64` surpass `u32::MAX`"
    )]
    #[must_use]
    fn from(digits: &DigitSlice<'_>) -> Self {
        let mut value = 0;

        // Ones place is `place = 0`, tens place is `place = 1`, etc.
        for (place, &digit) in digits.get().iter().rev().enumerate() {
            value += Self::from(digit) * 10_u32.pow(place as Self);
        }

        value
    }
}

impl From<DigitSlice<'_>> for u32 {
    #[must_use]
    fn from(digits: DigitSlice<'_>) -> Self {
        (&digits).into()
    }
}

/// Represents a floating-point number in a stable manner.
///
/// ```txt
/// | `sign = Sign::Negative`
/// v
/// -105.2060   <-- `digits = [1, 0, 5, 2, 0, 6, 0]`
///     ^
///     | `dot = 3`
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digits {
    /// The sign of the number represented by [`Self`].
    sign: Sign,
    /// The digit index that the dot is placed before.
    ///
    /// - For `0.05`, `dot = 1`.
    /// - For `0`, `dot = 1`.
    /// - For `100`, `dot = 3`.
    /// - For `100.2`, `dot = 3`.
    dot: usize,
    /// The list of digits contained by a number.
    ///
    /// - For `105.2060`, `digits = [1, 0, 5, 2, 0, 6, 0]`.
    digits: Box<[Digit]>,
}

impl Digits {
    /// Parses a floating-point value into a [`Self`].
    ///
    /// # Panics
    ///
    /// Panics if `value` is [`FpCategory::Nan`] or [`FpCategory::Infinite`].
    ///
    /// # Examples
    ///
    /// Working as expected:
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::Digits;
    /// #
    /// assert_eq!(Digits::new(1024.0).to_string(), "1024");
    /// assert_eq!(Digits::new(1024.05).to_string(), "1024.05");
    /// assert_eq!(Digits::new(0.0).to_string(), "0");
    /// assert_eq!(Digits::new(-0.0).to_string(), "-0");
    /// assert_eq!(Digits::new(0.03).to_string(), "0.03");
    /// ```
    ///
    /// NaN values cause panics:
    ///
    /// ```should_panic
    /// # use sciutil::rounding::digits::Digits;
    /// #
    /// let nan = Digits::new(f64::NAN);
    /// ```
    ///
    /// Infinite values cause panics:
    ///
    /// ```should_panic
    /// # use sciutil::rounding::digits::Digits;
    /// #
    /// let inf = Digits::new(f64::INFINITY);
    /// ```
    #[must_use]
    pub fn new(value: f64) -> Self {
        value
            .try_into()
            .expect("received invalid floating-point number")
    }

    /// Constructs a [`Self`] from its component parts without checking any invariants.
    ///
    /// # Safety
    ///
    /// Assumes that `dot` is at most `digits.len()`. Other guarantees may be added in the future
    /// without notice, consider this an experimental API.
    #[must_use]
    pub const unsafe fn from_raw_parts(sign: Sign, dot: usize, digits: Box<[Digit]>) -> Self {
        Self { sign, dot, digits }
    }

    /// Constructs a [`Self`] from its component parts.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digit, Digits, Sign};
    /// #
    /// # fn test() -> Option<()> {
    /// let digits_0 =
    ///     Digits::from_parts(Sign::Positive, 1, [Digit::Zero].to_vec().into_boxed_slice())?;
    /// assert_eq!(digits_0.to_string(), "0".to_string());
    ///
    /// // `dot` cannot be more than one away from the last index.
    /// assert!(
    ///     Digits::from_parts(Sign::Positive, 2, [Digit::Zero].to_vec().into_boxed_slice()).is_none()
    /// );
    ///
    /// let digits_102405 = Digits::from_parts(
    ///     Sign::Negative,
    ///     4,
    ///     [
    ///         Digit::One,
    ///         Digit::Zero,
    ///         Digit::Two,
    ///         Digit::Four,
    ///         Digit::Zero,
    ///         Digit::Five,
    ///     ]
    ///     .to_vec()
    ///     .into_boxed_slice(),
    /// )?;
    /// assert_eq!(digits_102405.to_string(), "-1024.05".to_string());
    /// #
    /// #     Some(())
    /// # }
    /// #
    /// # assert!(test().is_some());
    /// ```
    #[must_use]
    pub fn from_parts(sign: Sign, dot: usize, digits: Box<[Digit]>) -> Option<Self> {
        // TODO: should `dot` also be required to be greater than zero? Would there be any reason
        // to allow someone to opt-out of the leading zero? Should I refactor this whole class to
        // be based around `NonZeroUsize`?
        if dot > digits.len() {
            return None;
        }

        Some(Self { sign, dot, digits })
    }

    /// Converts [`Self`] into a [`SplitFloat`], splitting the digits on the left and right side of
    /// this [`Self`]'s dot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digit, Digits, Sign};
    /// #
    /// let (sign, lhs, rhs) = Digits::new(1024.05).to_split();
    ///
    /// assert_eq!(sign, Sign::Positive);
    /// assert_eq!(
    ///     lhs,
    ///     [Digit::One, Digit::Zero, Digit::Two, Digit::Four]
    ///         .to_vec()
    ///         .into_boxed_slice()
    /// );
    /// assert_eq!(rhs, [Digit::Zero, Digit::Five].to_vec().into_boxed_slice());
    /// ```
    #[must_use]
    pub fn to_split(&self) -> SplitFloat {
        let lhs = self.digits[0..self.dot].to_vec().into_boxed_slice();
        let rhs = self.digits[self.dot..].to_vec().into_boxed_slice();

        (self.sign, lhs, rhs)
    }

    /// Returns the digit index of the last significant digit in [`Self`] when rounding to one or
    /// two significant figures.
    ///
    /// This looks for the first non-zero [`Digit`]. If that [`Digit`] is 1 or 2, it returns the
    /// index of the next [`Digit`] if there is one. Otherwise, it returns the index of this first
    /// [`Digit`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::Digits;
    /// #
    /// // ```txt
    /// // 1024.05
    /// // -^
    /// // ```
    /// assert_eq!(Digits::new(1024.05).last_significant_digit(), 1);
    ///
    /// // ```txt
    /// // 42
    /// // ^
    /// // ```
    /// assert_eq!(Digits::new(42.0).last_significant_digit(), 0);
    /// ```
    #[must_use]
    pub fn last_significant_digit(&self) -> usize {
        let mut skipped_one_or_two_index = None;
        self.digits
            .iter()
            .enumerate()
            .find_map(|(index, digit)| match digit.get() {
                0 if skipped_one_or_two_index.is_none() => None,
                1 | 2 if skipped_one_or_two_index.is_none() => {
                    skipped_one_or_two_index = Some(index);
                    None
                }
                _ => Some(index),
            })
            .or(skipped_one_or_two_index)
            .unwrap_or(0)
    }

    /// Returns the [`Place`] of the last significant [`Digit`] in [`Self`] when rounding to one or
    /// two significant figures.
    ///
    /// This looks for the first non-zero [`Digit`]. If that [`Digit`] is 1 or 2, it returns the
    /// [`Place`] of the next [`Digit`] if there is one. Otherwise, it returns the [`Place`] of
    /// this first [`Digit`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::Digits;
    /// #
    /// // ```txt
    /// // 1024.05
    /// //  ^--
    /// // ```
    /// assert_eq!(Digits::new(1024.05).last_significant_place().get(), -3);
    ///
    /// // ```txt
    /// // 42
    /// // ^-
    /// // ```
    /// assert_eq!(Digits::new(42.0).last_significant_place().get(), -2);
    /// ```
    #[must_use]
    pub fn last_significant_place(&self) -> Place {
        self.digit_index_to_place(self.last_significant_digit())
    }

    /// Rounds [`Self`] to the given digit index.
    ///
    /// If `digit_index` is out of range, it will return a copy of [`Self`], unchanged.
    ///
    /// If the [`Digit`] at `digit_index + 1` is:
    ///
    /// - Out of range,
    /// - 0--4,
    /// - or 5 and the [`Digit`] at `digit_index` is even,
    ///
    /// It rounds down, simply truncating [`Self`] at `digits_index`.
    ///
    /// If the [`Digit`] at `digit_index + 1` is:
    ///
    /// - 6--9
    /// - or 5 and the [`Digit`] at `digit_index` is odd
    ///
    /// It rounds up, adding `1` to the [`Digit`] at `digit_index` (carrying tens up as necessary).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digits};
    /// #
    /// // ```txt
    /// // 0.015555312
    /// //     ^
    /// // 0.016
    /// // ```
    /// assert_eq!(
    ///     Digits::new(0.015555312).round_to_digit(3).to_string(),
    ///     "0.016",
    /// );
    ///
    /// // ```txt
    /// // 0.015555312
    /// //   ^
    /// // 0.0
    /// // ```
    /// assert_eq!(
    ///     Digits::new(0.015555312).round_to_digit(1).to_string(),
    ///     "0.0",
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //  ^
    /// // 1000
    /// // ```
    /// assert_eq!(
    ///     Digits::new(1024.05).round_to_digit(1).to_string(),
    ///     "1000",
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //      ^
    /// // 1024.0
    /// // ```
    /// assert_eq!(
    ///     Digits::new(1024.05).round_to_digit(4).to_string(),
    ///     "1024.0",
    /// );
    /// ```
    #[must_use]
    pub fn round_to_digit(&self, digit_index: usize) -> Self {
        if digit_index >= self.digits.len() {
            return self.clone();
        }

        // Truncating before the dot will create a number that's some number of orders of
        // magnitudes too small. This tracks the number of zeros that will need to be
        // appended.
        //
        // ```txt
        // 012345 6 `self.dot = 6`
        // 102345.0 `self.digits`
        //   ^      `digit_index = 2`
        //    ^^^   `trailing_zeros = 3`
        // ```
        let trailing_zeros = if digit_index < self.dot {
            self.dot - 1 - digit_index
        } else {
            0
        };

        let last_digit = self.digits[digit_index];
        let trailing_digit = self
            .digits
            .get(digit_index + 1)
            .copied()
            .unwrap_or(Digit::Zero);

        // Truncate digits beyond `digit_index`.
        let digits = DigitSlice::new(&self.digits[0..=digit_index]);

        // Round up if necessary.
        let mut digits = match trailing_digit.get() {
            0..=4 => digits.into_boxed(),
            5 if last_digit.get() % 2 == 0 => digits.into_boxed(),
            _ => digits.add(1),
        };

        // If rounding up caused another digit to be added, move the dot one digit to the right.
        let dot = if digits.len() == digit_index {
            self.dot + 1
        } else {
            self.dot
        };

        // If the addition return a slice shorter than expected, then there were some leading zeros
        // that got trimmed.
        //
        // ```txt
        // 009  Start
        //  10  Rounded to `digit_index` 1
        // 010  After restoring missing leading zeros
        // ```
        //
        // Note that it doesn't restore the same _number_ of leading zeros, it restores the leading
        // zeros up to the first _place_ they appeared.
        if digits.len() <= digit_index {
            let missing_leading_zeros = digit_index + 1 - digits.len();

            let mut vec = [Digit::Zero].repeat(missing_leading_zeros);
            vec.append(&mut digits.to_vec());

            digits = vec.into_boxed_slice();
        }

        // When rounding to a given digit, we truncate at that digit. If that digit was more
        // significant than the ones place, then suddenly you've lost some magnitudes. This tracks
        // the number of zeros between the last digit and the dot.
        //
        //
        // ```txt
        // 102345.0 `self.digits`
        //   ^      `digit_index = 2`
        //    ^^^   `trailing_zeros = 3`
        // 102      After rounding
        // 102000   After appending trailing zeros
        // ```
        if trailing_zeros > 0 {
            let mut vec = digits.to_vec();
            vec.append(&mut [Digit::Zero].repeat(trailing_zeros));

            digits = vec.into_boxed_slice();
        }

        Self {
            sign: self.sign, // Is this always true?
            digits,
            dot,
        }
    }

    /// Wrapper around [`Self::round_to_digit`] that uses [`Place`]s instead of digit indices.
    ///
    /// - If the provided [`Place`] is one digit to the left of [`Self`]'s first digit, this will
    ///   attempt to round up.
    /// - If the provided [`Place`] is more than one digit to the left of [`Self`]'s first digit,
    ///   this will return 0.
    /// - If the provided [`Place`] is to the right of [`Self`]'s last digit, it will return
    ///   [`Self`], unchanged.
    ///
    /// Otherwise, behaves the same as calling [`Self::place_to_digit_index`] and
    /// [`Self::round_to_digit`].
    #[expect(clippy::missing_panics_doc, reason = "see `expect` string")]
    #[must_use]
    pub fn round_to_place(&self, place: Place) -> Self {
        // Zero represents the dot for [`Place`] values, but the digit after the dot for digit
        // indices. This accounts for that difference.
        let offset = if place.is_positive() {
            place.get() - 1
        } else {
            place.get()
        };

        #[expect(
            clippy::cast_possible_wrap,
            reason = "I've never seen the number of digits in an `f64` surpass `i32::MAX`"
        )]
        let digit_index = self.dot as isize + offset;

        // If we're rounding to the digit immediately to the left of the first digit in [`Self`],
        // we have to opportunity to round up.
        if digit_index == -1 {
            if self.digits[0].get() > 5 {
                // E.g.,
                //
                // ```txt
                //  0123 4  `self.dot`
                //  6024.0  `self`
                //
                // 10000    return value
                // ```
                let mut rounded_up = vec![Digit::One];
                rounded_up.append(&mut [Digit::Zero].repeat(self.dot));
                return Self {
                    sign: self.sign,
                    dot: self.dot + 1,
                    digits: rounded_up.into_boxed_slice(),
                };
            }

            return Self::default();
        }

        if digit_index < -1 {
            return Self::default();
        }

        #[expect(
            clippy::cast_possible_wrap,
            reason = "I've never seen the number of digits in an `f64` surpass `i32::MAX`"
        )]
        if digit_index >= self.digits.len() as isize {
            return self.clone();
        }

        self.round_to_digit(
            self.place_to_digit_index(place)
                .expect("handled every out-of-range case"),
        )
    }

    /// Converts a digit index (oriented the list of digits, specific to this [`Self`]) to a
    /// generic [`Place`] (oriented around this [`Self`]'s dot).
    #[expect(clippy::missing_panics_doc, reason = "see `expect` string")]
    #[must_use]
    pub const fn digit_index_to_place(&self, digit_index: usize) -> Place {
        #[expect(
            clippy::cast_possible_wrap,
            reason = "I've never seen the number of digits in an `f64` surpass `i32::MAX`"
        )]
        let place = digit_index as isize - self.dot as isize;

        // Zero represents the dot for [`Place`] values, but the digit after the dot for digit
        // indices. This accounts for that difference.
        Place::new(if digit_index >= self.dot {
            // This prevents any zero values.
            place + 1
        } else {
            place
        })
        .expect("`a - b == 0` only when `a == b`, but `place = a - b + 1` when that is true")
    }

    /// Converts a generic [`Place`] (oriented around this [`Self`]'s dot) to a digit index
    /// (oriented around the list of digits, specific to this [`Self`]).
    ///
    /// Returns [`None`] if the provided [`Place`] exists outside of the range of this [`Self`]'s
    /// list of digits.
    #[must_use]
    pub fn place_to_digit_index(&self, place: Place) -> Option<usize> {
        // Zero represents the dot for [`Place`] values, but the digit after the dot for digit
        // indices. This accounts for that difference.
        let offset = if place.is_positive() {
            place.get() - 1
        } else {
            place.get()
        };

        self.dot.checked_add_signed(offset).and_then(|dot| {
            if dot < self.digits.len() {
                Some(dot)
            } else {
                None
            }
        })
    }
}

impl Default for Digits {
    #[must_use]
    fn default() -> Self {
        Self {
            sign: Sign::Positive,
            dot: 0,
            digits: [Digit::Zero].to_vec().into_boxed_slice(),
        }
    }
}

impl TryFrom<f64> for Digits {
    type Error = ();

    /// Converts an [`f64`] to base-ten decimal number and parses it into a [`Self`].
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if `value` is [`FpCategory::Nan`] or [`FpCategory::Infinite`].
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if matches!(value.classify(), FpCategory::Nan | FpCategory::Infinite) {
            return Err(());
        }

        let str = value.to_string();
        let (sign, str) = str
            .strip_prefix("-")
            .map_or((Sign::Positive, str.as_str()), |str| (Sign::Negative, str));

        let mut digits: Vec<Digit> = vec![];
        let mut dot = None;

        for (index, digit) in str.chars().enumerate() {
            if digit == '.' {
                dot = Some(index);
            } else {
                digits.push(digit.try_into().expect(
                    "`f64::to_string` should only return sign, digits, and dots for normal numbers",
                ));
            }
        }

        Ok(Self {
            sign,
            dot: dot.unwrap_or(digits.len()),
            digits: digits.into_boxed_slice(),
        })
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = self.sign.to_string();

        // Print zero as `"0"`, not `".0"`.
        if self.digits.len() == 1 && self.digits[0] == Digit::Zero && self.dot == 0 {
            str.push('0');
            return write!(f, "{str}");
        }

        for (index, &digit) in self.digits.iter().enumerate() {
            if index == self.dot {
                str.push('.');
            }

            str.push(digit.into());
        }

        write!(f, "{str}")
    }
}

/// Represents a float-point value split at the dot.
pub type SplitFloat = (Sign, Box<[Digit]>, Box<[Digit]>);

/// Represents the "place" (position) of a digit in a number.
///
/// Negative distances are head left away from the dot, positive values are heading right:
///
/// - `-1` is the ones place, `-2` the tens place, `-3` the hundreds place, etc.
/// - `1` is the tenths place, `2` is the tenths place, `3` is the hundredths place, etc.
pub type Place = NonZeroIsize;
