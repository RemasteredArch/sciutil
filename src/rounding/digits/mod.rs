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

mod defs;
mod err;

use std::{cmp::Ordering, fmt::Display, marker::PhantomData, num::FpCategory};

// Everything that isn't [`Digits`] is kept out of this file to keep it from being too long, but
// needs to be publicly reexported to keep the API flat.
pub use defs::*;
pub use err::*;

use crate::{
    err::InvalidFloatError,
    units::{Float, FloatDisplay},
};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Deserializer, Serialize};

/// Represents a floating-point number in a stable manner.
///
/// ```txt
/// | `sign = Sign::Negative`
/// v
/// -105.2060   <-- `digits = [1, 0, 5, 2, 0, 6, 0]`
///     ^
///     | `dot = 3`
/// ```
// I probably have to implement [`Deserialize`] and [`Serialize`] myself, as with the rest of the
// traits I used to `derive` on [`Digits`].
#[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
#[cfg_attr(any(feature = "serde", test), serde(remote = "Self"))]
pub struct Digits<F: Float> {
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
    #[expect(clippy::struct_field_names, reason = "this is the core of the struct")]
    digits: Box<[Digit]>,

    /// Hold onto the type of the original [`Float`].
    #[cfg_attr(any(feature = "serde", test), serde(skip))]
    phantom: PhantomData<F>,
}

// The hack that makes the below `Deserialize` implementation work (the `serde(remote = "Self")`)
// also disables the derived `Serialized` implementation from being applied properly, so we just
// have to make a quick wrapper implementation.
#[cfg(any(feature = "serde", test))]
impl<F: Float> Serialize for Digits<F> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Self::serialize(self, serializer)
    }
}

#[cfg(any(feature = "serde", test))]
impl<'de, F: Float> Deserialize<'de> for Digits<F> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Use the derived implementation for the actual deserialization.
        let unchecked = Self::deserialize(deserializer)?;

        // Verify that invariants are upheld.
        if unchecked.digits.is_empty() {
            return Err(serde::de::Error::custom(
                "`Digits::digits` must have at least one digit",
            ));
        }

        if unchecked.dot > unchecked.digits.len() {
            return Err(serde::de::Error::custom(
                "`Digits::dot` must be no greater than `Digits::digits.len()`",
            ));
        }

        // Now assuredly valid.
        Ok(unchecked)
    }
}

impl<F: Float> Digits<F> {
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
    /// # use sciutil::{
    /// #     rounding::digits::Digits,
    /// #     units::{Float, FloatDisplay, Seconds},
    /// # };
    /// #
    /// assert_eq!(Digits::<f64>::new(&1024.0).to_string(), "1024");
    /// assert_eq!(Digits::<Seconds>::new(&Seconds::new(1024.05)).to_string_with_units(), "1024.05 s");
    /// assert_eq!(Digits::<f64>::new(&0.0).to_string(), "0");
    /// assert_eq!(Digits::<f64>::new(&-0.0).to_string(), "-0");
    /// assert_eq!(Digits::<f64>::new(&0.03).to_string(), "0.03");
    /// ```
    ///
    /// NaN values cause panics:
    ///
    /// ```should_panic
    /// # use sciutil::rounding::digits::Digits;
    /// #
    /// let nan = Digits::<f64>::new(&f64::NAN);
    /// ```
    ///
    /// Infinite values cause panics:
    ///
    /// ```should_panic
    /// # use sciutil::rounding::digits::Digits;
    /// #
    /// let inf = Digits::<f64>::new(&f64::INFINITY);
    /// ```
    #[must_use]
    pub fn new(value: &F) -> Self {
        (value.get())
            .try_into()
            .expect("received invalid floating-point number")
    }

    /// Constructs a [`Self`] from its component parts without checking any invariants.
    ///
    /// # Safety
    ///
    /// Assumes that `dot` is at most `digits.len()` and that `digits` has at least one digit.
    /// Other guarantees may be added in the future without notice, consider this an experimental
    /// API.
    #[must_use]
    pub const unsafe fn from_parts_unchecked(sign: Sign, dot: usize, digits: Box<[Digit]>) -> Self {
        Self {
            sign,
            dot,
            digits,
            phantom: PhantomData,
        }
    }

    /// Constructs a [`Self`] from its component parts.
    ///
    /// # Errors
    ///
    /// - Returns [`InvalidDigitsPartsError::OutOfBoundsDot`] if `dot` is greater than
    ///   `digits.len()`.
    /// - Returns [`InvalidDigitsPartsError::EmptyDigitsList`] if `digits` is empty.
    ///
    /// Other guarantees may be added in the future without notice, consider this an experimental
    /// API.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digit, Digits, InvalidDigitsPartsError, Sign};
    /// #
    /// # fn test() -> Result<(), InvalidDigitsPartsError> {
    /// let digits_0 = Digits::<f64>::from_parts(
    ///     Sign::Positive,
    ///     1,
    ///     [Digit::Zero].to_vec().into_boxed_slice()
    /// )?;
    /// assert_eq!(digits_0.to_string(), "0".to_string());
    ///
    /// // `dot` cannot be more than one away from the last index.
    /// assert!(
    ///     Digits::<f64>::from_parts(Sign::Positive, 2, [Digit::Zero].to_vec().into_boxed_slice())
    ///         .is_err()
    /// );
    ///
    /// let digits_102405 = Digits::<f64>::from_parts(
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
    /// #     Ok(())
    /// # }
    /// #
    /// # assert!(test().is_ok());
    /// ```
    pub fn from_parts(
        sign: Sign,
        dot: usize,
        digits: Box<[Digit]>,
    ) -> Result<Self, InvalidDigitsPartsError> {
        if digits.is_empty() {
            return Err(InvalidDigitsPartsError::EmptyDigitsList);
        }

        // TODO: should `dot` also be required to be greater than zero? Would there be any reason
        // to allow someone to opt-out of the leading zero? Should I refactor this whole class to
        // be based around [`NonZeroUsize`]?
        if dot > digits.len() {
            return Err(InvalidDigitsPartsError::OutOfBoundsDot);
        }

        Ok(Self {
            sign,
            dot,
            digits,
            phantom: PhantomData,
        })
    }

    /// Converts [`Self`] into a [`SplitFloat`], splitting the digits on the left and right side of
    /// this [`Self`]'s dot.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digit, Digits, Sign};
    /// #
    /// let (sign, lhs, rhs) = Digits::<f64>::new(&1024.05).to_split();
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

    #[must_use]
    pub const fn is_one(&self) -> bool {
        self.dot == 1
            && self.digits.len() == 1
            && match self.digits.first() {
                Some(d) => matches!(*d, Digit::One),
                None => false,
            }
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
    /// assert_eq!(Digits::<f64>::new(&1024.05).last_significant_digit(), 1);
    ///
    /// // ```txt
    /// // 42
    /// // ^
    /// // ```
    /// assert_eq!(Digits::<f64>::new(&42.0).last_significant_digit(), 0);
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
    /// assert_eq!(Digits::<f64>::new(&1024.05).last_significant_place().get(), -3);
    ///
    /// // ```txt
    /// // 42
    /// // ^-
    /// // ```
    /// assert_eq!(Digits::<f64>::new(&42.0).last_significant_place().get(), -2);
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
    ///     Digits::<f64>::new(&0.015555312).round_to_digit(3).to_string(),
    ///     "0.016",
    /// );
    ///
    /// // ```txt
    /// // 0.015555312
    /// //   ^
    /// // 0.0
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&0.015555312).round_to_digit(1).to_string(),
    ///     "0.0",
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //  ^
    /// // 1000
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).round_to_digit(1).to_string(),
    ///     "1000",
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //      ^
    /// // 1024.0
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).round_to_digit(4).to_string(),
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
            phantom: PhantomData,
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digits, Place};
    /// #
    /// // ```txt
    /// // 0.015555312
    /// //     ^
    /// // 0.016
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&0.015555312).round_to_place(Place::new(3).unwrap()).to_string(),
    ///     "0.016",
    /// );
    ///
    /// // ```txt
    /// // 0.015555312
    /// //   ^
    /// // 0.0
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&0.015555312).round_to_place(Place::new(1).unwrap()).to_string(),
    ///     "0.0",
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //  ^
    /// // 1000
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).round_to_place(Place::new(-3).unwrap()).to_string(),
    ///     "1000",
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //      ^
    /// // 1024.0
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).round_to_place(Place::new(1).unwrap()).to_string(),
    ///     "1024.0",
    /// );
    /// ```
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
                    phantom: PhantomData,
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digits, Place};
    /// #
    /// // ```txt
    /// // 0.015555312
    /// //     ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&0.015555312).digit_index_to_place(3),
    ///     Place::new(3).unwrap(),
    /// );
    ///
    /// // ```txt
    /// // 0.015555312
    /// //   ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&0.015555312).digit_index_to_place(1),
    ///     Place::new(1).unwrap(),
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //  ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).digit_index_to_place(1),
    ///     Place::new(-3).unwrap(),
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //      ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).digit_index_to_place(4),
    ///     Place::new(1).unwrap(),
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //         ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).digit_index_to_place(7),
    ///     Place::new(4).unwrap(),
    /// );
    /// ```
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
    /// # Errors
    ///
    /// Returns [`OutOfBoundsPlaceError`] if the provided [`Place`] exists outside of the range of
    /// this [`Self`]'s list of digits.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::rounding::digits::{Digits, OutOfBoundsPlaceError, Place};
    /// #
    /// // ```txt
    /// // 0.015555312
    /// //     ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&0.015555312).place_to_digit_index(Place::new(3).unwrap()),
    ///     Ok(3),
    /// );
    ///
    /// // ```txt
    /// // 0.015555312
    /// //   ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&0.015555312).place_to_digit_index(Place::new(1).unwrap()),
    ///     Ok(1),
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //  ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).place_to_digit_index(Place::new(-3).unwrap()),
    ///     Ok(1),
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //      ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).place_to_digit_index(Place::new(1).unwrap()),
    ///     Ok(4),
    /// );
    ///
    /// // ```txt
    /// // 1024.05
    /// //         ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).place_to_digit_index(Place::new(4).unwrap()),
    ///     Err(OutOfBoundsPlaceError),
    /// );
    ///
    /// // ```txt
    /// //   1024.05
    /// // ^
    /// // ```
    /// assert_eq!(
    ///     Digits::<f64>::new(&1024.05).place_to_digit_index(Place::new(-6).unwrap()),
    ///     Err(OutOfBoundsPlaceError),
    /// );
    /// ```
    pub fn place_to_digit_index(&self, place: Place) -> Result<usize, OutOfBoundsPlaceError> {
        // Zero represents the dot for [`Place`] values, but the digit after the dot for digit
        // indices. This accounts for that difference.
        let offset = if place.is_positive() {
            place.get() - 1
        } else {
            place.get()
        };

        self.dot
            .checked_add_signed(offset)
            .map_or(Err(OutOfBoundsPlaceError), |dot| {
                if dot < self.digits.len() {
                    Ok(dot)
                } else {
                    Err(OutOfBoundsPlaceError)
                }
            })
    }

    /// Cast [`Self`] to a [`Digit<T>`] of some other [`Float`] `T`.
    ///
    /// ```rust
    /// # use sciutil::{
    /// #     rounding::digits::Digits,
    /// #     units::{Float, FloatDisplay, Seconds},
    /// # };
    /// #
    /// let a: Digits<f64> = Digits::<f64>::new(&123.0);
    /// let b: Digits<Seconds> = a.cast();
    ///
    /// assert_eq!(b.to_string_with_units(), "123 s");
    /// ```
    #[must_use]
    pub fn cast<T: Float>(self) -> Digits<T> {
        let Self {
            sign, dot, digits, ..
        } = self;

        Digits::<T> {
            sign,
            dot,
            digits,
            phantom: PhantomData,
        }
    }
}

impl<F: Float> TryFrom<f64> for Digits<F> {
    type Error = InvalidFloatError;

    /// Converts an [`f64`] to base-ten decimal number and parses it into a [`Self`].
    ///
    /// This has to be `impl<F: Float> TryFrom<f64> for Digits<F>` instead of
    /// `impl<F: Float> TryFrom<F> for Digits<F>` because downstream types that implement [`Float`]
    /// may also implement [`Into<Digits>`], which would create a conflicting implementation of
    /// [`TryInto<Digits>`] through [`core`]'s blanket implementation of [`TryInto`] for any type
    /// that implements [`Into`]. This would be fixed by [specialization][rust#31844].
    ///
    /// See also [`Digits::new`].
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if `value` is [`FpCategory::Nan`] or [`FpCategory::Infinite`].
    ///
    /// [rust#31844]: <https://github.com/rust-lang/rust/issues/31844>
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value.classify() {
            FpCategory::Nan => return Err(InvalidFloatError::Nan),
            FpCategory::Infinite => return Err(InvalidFloatError::Infinite),
            _ => (),
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
            phantom: PhantomData,
        })
    }
}

impl<F: Float> Display for Digits<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::with_capacity(self.digits.len());

        if matches!(self.sign, Sign::Negative) {
            str.push('-');
        }

        // Print zero as `"0"`, not `".0"`.
        if self.digits.len() == 1 && self.digits[0] == Digit::Zero &&
            // Should this be zero or one?
            self.dot == 0
        {
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

impl<F: FloatDisplay> Digits<F> {
    #[must_use]
    pub fn to_string_with_units(&self) -> String {
        let mut str = self.to_string();
        str.push(' ');
        str.push_str(&F::symbol());
        str
    }
}

// The following implementations are manual implementations of commonly derived traits.
//
// `#[derive(...)]` will not derive a trait if one of its generics doesn't implement it. We want to
// implement this traits anyways because [`Digits<F>`] doesn't actually store any instances of `F`.
// Unfortunately, that means we have to implement these traits ourselves.

impl<F: Float> Clone for Digits<F> {
    fn clone(&self) -> Self {
        Self {
            sign: self.sign,
            dot: self.dot,
            digits: self.digits.clone(),
            phantom: PhantomData,
        }
    }
}

impl<F: Float> Default for Digits<F> {
    fn default() -> Self {
        Self {
            sign: Sign::Positive,
            // Should this be zero or one?
            dot: 0,
            digits: [Digit::Zero].to_vec().into_boxed_slice(),
            phantom: PhantomData,
        }
    }
}

impl<F: Float> Eq for Digits<F> {}

impl<F: Float> PartialEq for Digits<F> {
    fn eq(&self, other: &Self) -> bool {
        self.sign == other.sign && self.dot == other.dot && self.digits == other.digits
    }
}

impl<F: Float> Ord for Digits<F> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let cmp = self.sign.cmp(&other.sign);
        if cmp != Ordering::Equal {
            return cmp;
        }
        let cmp = self.dot.cmp(&other.dot);
        if cmp != Ordering::Equal {
            return cmp;
        }
        self.digits.cmp(&other.digits)
    }
}

impl<F: Float> PartialOrd for Digits<F> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<F: Float> std::hash::Hash for Digits<F> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sign.hash(state);
        self.dot.hash(state);
        self.digits.hash(state);
        self.phantom.hash(state);
    }
}

impl<F: Float> std::fmt::Debug for Digits<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Digits")
            .field("sign", &self.sign)
            .field("dot", &self.dot)
            .field("digits", &self.digits)
            .field("phantom", &self.phantom)
            .finish()
    }
}
