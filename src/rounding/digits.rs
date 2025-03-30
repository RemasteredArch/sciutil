// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

#![allow(dead_code)]

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
            Sign::Positive => "",
            Sign::Negative => "-",
        };

        write!(f, "{as_str}",)
    }
}

/// Represents a base-ten digit, from 0-9.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Digit(u8);

impl Digit {
    const MIN: u8 = 0;
    const MAX: u8 = 9;
    const ZERO: Self = Self(0);
    const ONE: Self = Self(1);

    pub fn new(digit: u8) -> Option<Self> {
        if (Self::MIN..=Self::MAX).contains(&digit) {
            return Some(Self(digit));
        }

        None
    }

    #[cfg(test)]
    pub const unsafe fn new_raw(digit: u8) -> Self {
        Self(digit)
    }

    pub fn get(&self) -> u8 {
        self.0
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
        // blindly construct [`Self`].
        Ok(Self(digit.to_digit(10).ok_or(())? as u8))
    }
}

impl From<Digit> for char {
    fn from(digit: Digit) -> Self {
        const CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        CHARS[digit.get() as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitSlice<'a>(&'a [Digit]);

impl<'a> DigitSlice<'a> {
    pub const fn new(digits: &'a [Digit]) -> Self {
        Self(digits)
    }

    pub fn add(&self, value: u32) -> Box<[Digit]> {
        // This could be more efficient, but whatever.
        (u32::from(self) + value)
            .to_string()
            .chars()
            .map(|c| Digit::try_from(c).expect("`u32::to_string` will only produce digits 0-9"))
            .collect()
    }

    pub fn get(&self) -> &'a [Digit] {
        self.0
    }

    pub fn into_boxed(self) -> Box<[Digit]> {
        self.0.to_vec().into_boxed_slice()
    }
}

impl From<&DigitSlice<'_>> for u32 {
    fn from(digits: &DigitSlice<'_>) -> Self {
        let mut value = 0;
        // Ones place is `place = 0`, tens place is `place = 1`, etc.
        for (place, digit) in digits.get().iter().rev().enumerate() {
            value += digit.get() as u32 * 10_u32.pow(place as u32)
        }

        value
    }
}

impl From<DigitSlice<'_>> for u32 {
    fn from(digits: DigitSlice<'_>) -> Self {
        (&digits).into()
    }
}

/// Represents a floating point number in a stable manner.
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
    /// - For `100.0`, `dot = 3`.
    dot: usize,
    /// The list of digits contained by a number.
    ///
    /// - For `105.2060`, `digits = [1, 0, 5, 2, 0, 6, 0]`.
    digits: Box<[Digit]>,
}

impl Digits {
    pub fn new(value: f64) -> Self {
        value.into()
    }

    #[cfg(test)]
    pub const unsafe fn new_raw(sign: Sign, dot: usize, digits: Box<[Digit]>) -> Self {
        Self { sign, dot, digits }
    }

    pub fn to_split(&self) -> SplitFloat {
        let lhs = self.digits[0..self.dot].to_vec().into_boxed_slice();
        let rhs = self.digits[self.dot..].to_vec().into_boxed_slice();

        (self.sign, lhs, rhs)
    }

    /// Returns the digit index of the last significant digit in [`Self`] when rounding to one or
    /// two significant figures.
    ///
    /// This looks for the first non-zero digit. If that digit is 1 or 2, it returns the index of
    /// the next digit if there is one. Otherwise, it returns the index of this first digit.
    pub fn last_sigificant_digit(&self) -> usize {
        let mut skipped_one_or_two_index = None;
        self.digits
            .iter()
            .enumerate()
            .find_map(|(index, digit)| match digit.get() {
                0 => None,
                1 | 2 if skipped_one_or_two_index.is_none() => {
                    skipped_one_or_two_index = Some(index);
                    None
                }
                _ => Some(index),
            })
            .or(skipped_one_or_two_index)
            .unwrap_or(0)
    }

    /// Returns the [`Place`] of the last significant digit in [`Self`] when rounding to one or two
    /// significant figures.
    ///
    /// This looks for the first non-zero digit. If that digit is 1 or 2, it returns the [`Place`]
    /// of the next digit if there is one. Otherwise, it returns the [`Place`] of this first digit.
    pub fn last_sigificant_place(&self) -> Place {
        self.digit_index_to_place(self.last_sigificant_digit())
    }

    /// Rounds [`Self`] to the given digit index.
    ///
    /// If the digit at `digit_index + 1` is:
    ///
    /// - Out of range,
    /// - 0-4,
    /// - or 5 and the digit at `digit_index` is even,
    ///
    /// It rounds down, simply truncating [`Self`] at `digits_index`.
    ///
    /// If the digit at `digit_index + 1` is:
    ///
    /// - 6-9
    /// - or 5 and the digit at `digit_index` is odd
    ///
    /// It rounds up, adding `1` to the digit at `digit_index` (carrying tens up as necessary).
    ///
    /// # Panics
    ///
    /// Panics if `digit_index` is out of range.
    pub fn round_to_digit(&self, digit_index: usize) -> Self {
        if digit_index >= self.digits.len() {
            panic!("digit index out of range");
        }

        // Truncating before the dot will create a number that's some number of orders of
        // magnitudes too small. This tracks the number of zeros that will need to be
        // appended.
        //
        // ```
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
            .unwrap_or(Digit::ZERO);

        // Truncate digits beyond `digit_index`.
        let digits = DigitSlice(&self.digits[0..=digit_index]);

        // Round up if necessary.
        let mut digits = match trailing_digit.get() {
            0..=4 => digits.into_boxed(),
            5 if last_digit.get() % 2 == 0 => digits.into_boxed(),
            _ => digits.add(1),
        };

        // If rounding up caused another digits to be added, move the dot one digit to the right.
        let dot = if digits.len() == digit_index {
            self.dot + 1
        } else {
            self.dot
        };

        if digits.len() <= digit_index && digit_index != 0 {
            let missing_leading_zeros = digit_index + 1 - digits.len();

            let mut vec = [Digit::ZERO].repeat(missing_leading_zeros);
            vec.append(&mut digits.to_vec());

            digits = vec.into_boxed_slice();
        }

        if trailing_zeros > 0 {
            let mut vec = digits.to_vec();
            vec.append(&mut [Digit::ZERO].repeat(trailing_zeros));

            digits = vec.into_boxed_slice();
        }

        Self {
            sign: self.sign, // Is this always true?
            digits,
            dot,
        }
    }

    /// Wrapper around [`Self::round_to_digit`] that uses [`Place`]s instead of digit indices. See
    /// that function's documentation for more.
    ///
    /// Returns [`None`] if the provided [`Place`] exists outside of the range of [`Self::digits`].
    pub fn round_to_place(&self, place: Place) -> Option<Self> {
        // Zero represents the dot for [`Place`] values, but the digit after the dot for digit
        // indices. This accounts for that difference.
        let offset = if place.is_positive() {
            place.get() - 1
        } else {
            place.get()
        };
        let digit_index = self.dot as isize + offset;

        // If we're rounding to the digit immediately to the left of the first digit in [`Self`],
        // we have to opportunity to round up.
        if digit_index == -1 {
            if self.digits[0].get() > 5 {
                // E.g.,
                //
                // ```
                //  0123 4  `self.dot`
                //  6024.0  `self`
                //
                // 10000    return value
                // ```
                let mut rounded_up = vec![Digit::ONE];
                rounded_up.append(&mut [Digit::ZERO].repeat(self.dot));
                return Some(Self {
                    sign: self.sign,
                    dot: self.dot + 1,
                    digits: rounded_up.into_boxed_slice(),
                });
            }

            return Some(Self {
                sign: self.sign,
                dot: 0,
                digits: [Digit::ZERO].to_vec().into_boxed_slice(),
            });
        }

        if digit_index >= self.digits.len() as isize {
            return Some(self.clone());
        }

        Some(self.round_to_digit(self.place_to_digit_index(place)?))
    }

    /// Converts a digit index (oriented the list of digits, specific to this [`Self`]) to a
    /// generic [`Place`] (oriented around [`Self::dot`]).
    pub fn digit_index_to_place(&self, digit_index: usize) -> Place {
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

    /// Converts a generic [`Place`] (oriented around [`Self::dot`]) to a digit index (oriented
    /// around the list of digits, specific to this [`Self`]).
    ///
    /// Returns [`None`] if the provided [`Place`] exists outside of the range of [`Self::digits`].
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

impl From<f64> for Digits {
    fn from(value: f64) -> Self {
        match value.classify() {
            FpCategory::Nan | FpCategory::Infinite => {
                panic!("received invalid floating point number")
            }
            _ => (),
        }

        let str = value.to_string();
        let (sign, str) = match str.strip_prefix("-") {
            Some(str) => (Sign::Negative, str),
            None => (Sign::Positive, str.as_str()),
        };

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

        Self {
            sign,
            dot: dot.unwrap_or(digits.len()),
            digits: digits.into_boxed_slice(),
        }
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = self.sign.to_string();

        // Print zero as `"0"`, not `".0"`.
        if self.digits.len() == 1 && self.digits[0].get() == 0 && self.dot == 0 {
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

pub type SplitFloat = (Sign, Box<[Digit]>, Box<[Digit]>);

/// Negative distances are head left away from `.`, positive values are heading right:
///
/// - `-1` is the ones place, `-2` the tens place, `-3` the hundreds place, etc.
/// - `1` is the tenths place, `2` is the tenths place, `3` is the hundredths place, etc.
pub type Place = NonZeroIsize;
