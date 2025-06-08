// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `defs`: Implementations for auxiliary types for [`super`].
//!
//! Everything that isn't [`Digits`] is kept out of [`super`] to keep it from being too long (but
//! publicly reexported so that the API remains flat).

use crate::{err::InvalidDigitError, units::Float};

use super::Digits;

use std::{fmt::Display, num::NonZeroIsize};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

/// Represents whether a number is positive or negative.
#[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Sign {
    #[default]
    Positive,
    Negative,
}

impl Display for Sign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let as_str = match self {
            Self::Positive => "+",
            Self::Negative => "-",
        };

        write!(f, "{as_str}",)
    }
}

/// Represents a base-ten digit, from 0--9.
#[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Digit {
    #[default]
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
    /// # Errors
    ///
    /// Returns [`InvalidDigitError`] if `digit` is not between zero and nine.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::Digit;
    /// #
    /// assert_eq!(Digit::new(0), Ok(Digit::Zero));
    /// assert_eq!(Digit::new(9), Ok(Digit::Nine));
    /// assert!(Digit::new(10).is_err());
    /// ```
    pub const fn new(digit: u8) -> Result<Self, InvalidDigitError> {
        Ok(match digit {
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
            _ => return Err(InvalidDigitError),
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
    type Error = InvalidDigitError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<u32> for Digit {
    type Error = InvalidDigitError;

    fn try_from(digit: u32) -> Result<Self, Self::Error> {
        u8::try_from(digit)
            .map_err(|_| InvalidDigitError)?
            .try_into()
    }
}

impl TryFrom<char> for Digit {
    type Error = InvalidDigitError;

    fn try_from(digit: char) -> Result<Self, Self::Error> {
        // `to_digit(10)` will return a number from 0--9, so it is safe to cast to [`u8`] and
        // blindly construct [`Self`] with unwrap.
        #[expect(clippy::cast_possible_truncation, reason = "see comment")]
        Ok(Self::new(digit.to_digit(10).ok_or(InvalidDigitError)? as u8).unwrap())
    }
}

impl From<Digit> for char {
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
    fn from(digit: Digit) -> Self {
        digit.get()
    }
}

impl From<Digit> for u32 {
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
#[cfg_attr(any(feature = "serde", test), derive(Serialize))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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
    pub fn add(&self, mut value: u32) -> Box<[Digit]> {
        value += u32::from(self);

        // `value.ilog10()` panics if `value == 0`, so we special case that.
        if value == 0 {
            return [Digit::Zero].to_vec().into_boxed_slice();
        }
        // The number of digits in `value`.
        let len = (value.ilog10() + 1) as usize;

        let mut digits = [Digit::Zero].repeat(len).into_boxed_slice();
        for i in (0..len).rev() {
            digits[i] = Digit::try_from(value % 10)
                .expect("`u32 % 10` won't produce a value greater than 9");
            value /= 10;
        }

        digits
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
    fn from(digits: DigitSlice<'_>) -> Self {
        (&digits).into()
    }
}

/// Represents a float-point value split at the dot.
///
/// E.g., `123.456 == SplitFloat(Positive, [1, 2, 3], [4, 5, 6])`.
pub type SplitFloat = (Sign, Box<[Digit]>, Box<[Digit]>);

/// Represents the "place" (position) of a digit in a number.
///
/// Negative distances are head left away from the dot, positive values are heading right:
///
/// - `-1` is the ones place, `-2` the tens place, `-3` the hundreds place, etc.
/// - `1` is the tenths place, `2` is the tenths place, `3` is the hundredths place, etc.
///
/// ```txt
/// ...  1245.6789 ...
///      ^  ^ ^  ^
/// ... -4 -1 1  4 ...
/// ```
pub type Place = NonZeroIsize;

/// The absolute uncertainty in that value.
pub struct UncertainDigits<F: Float> {
    /// The measured value.
    value: Digits<F>,

    /// The absolute uncertainty in that value.
    uncertainty: Digits<F>,
}

// Perhaps there should be an `Uncertain` trait to cover both [`UncertainDigits`] and
// [`sciutil::units::UncertainFloat`]?
impl<F: Float> UncertainDigits<F> {
    /// Construct a new instance of [`Self`].
    #[must_use]
    pub const fn new(value: Digits<F>, uncertainty: Digits<F>) -> Self {
        Self { value, uncertainty }
    }

    /// Returns the measured value.
    #[must_use]
    pub const fn value(&self) -> &Digits<F> {
        &self.value
    }

    /// Returns the absolute uncertainty.
    #[must_use]
    pub const fn uncertainty(&self) -> &Digits<F> {
        &self.uncertainty
    }

    // Requires that I implement math for [`Digits`].
    //
    // ```rust
    // /// Returns the minimum possible value.
    // #[must_use]
    // pub fn min(&self) -> Digits<F> {
    //     self.value - self.uncertainty.abs()
    // }
    //
    // /// Returns the maximum possible value.
    // #[must_use]
    // pub fn max(&self) -> Digits<F> {
    //     self.value + self.uncertainty.abs()
    // }
    // ```
}
