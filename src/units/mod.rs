// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `units`: Wrapper structs to mark arbitrary floating-point values as SI units.
//!
//! In particular, see [`Float`] and [`UncertainFloat`].

pub trait Float: From<f64> + Into<f64> {
    /// Constructs a new instance of [`Self`].
    #[must_use]
    fn new(value: f64) -> Self;

    /// Returns the internal [`f64`] representation of [`Self`].
    #[must_use]
    fn get(&self) -> f64;

    /// The long textual representation of this unit, if there is one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sciutil::units::{Float, Seconds};
    /// #
    /// assert_eq!(Seconds::NAME_SINGLE, Some("second"))
    /// ```
    const NAME_SINGLE: Option<&str>;

    /// The plural form of the long textual representation of this unit, if there is one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sciutil::units::{Float, Seconds};
    /// #
    /// assert_eq!(Seconds::NAME_PLURAL, Some("seconds"))
    /// ```
    const NAME_PLURAL: Option<&str>;

    /// The short representation of this unit, if there is one.
    ///
    /// # Examples
    ///
    /// ```
    /// # use sciutil::units::{Float, Seconds};
    /// #
    /// assert_eq!(Seconds::SYMBOL, Some("s"))
    /// ```
    const SYMBOL: Option<&str>;
}

impl Float for f64 {
    const SYMBOL: Option<&str> = None;
    const NAME_SINGLE: Option<&str> = None;
    const NAME_PLURAL: Option<&str> = None;

    fn new(value: f64) -> Self {
        value
    }

    fn get(&self) -> f64 {
        *self
    }
}

/// Represents a value with an associated absolute uncertainty.
///
/// # Examples
///
/// ```rust
/// # use sciutil::units::{Float, UncertainFloat};
/// #
/// let with_uncertainty = UncertainFloat::new(5.0, 1.0);
/// assert_eq!(with_uncertainty.min(), 4.0);
/// assert_eq!(with_uncertainty.max(), 6.0);
/// ```
pub struct UncertainFloat<F: Float> {
    /// The measured value.
    value: F,
    /// The absolute uncertainty in that value.
    uncertainty: F,
}

impl<F: Float> UncertainFloat<F> {
    /// Construct a new instance of [`Self`].
    #[must_use]
    pub const fn new(value: F, uncertainty: F) -> Self {
        Self { value, uncertainty }
    }

    /// Returns the measured value.
    #[must_use]
    pub const fn value(&self) -> &F {
        &self.value
    }

    /// Returns the absolute uncertainty.
    #[must_use]
    pub const fn uncertainty(&self) -> &F {
        &self.uncertainty
    }

    /// Returns the minimum possible value.
    #[must_use]
    pub fn min(&self) -> F {
        F::new(self.value.get() - self.uncertainty.get().abs())
    }

    /// Returns the maximum possible value.
    #[must_use]
    pub fn max(&self) -> F {
        F::new(self.value.get() + self.uncertainty.get().abs())
    }
}

macro_rules! float_type {
    ($(#[$attribute:meta])* $unit:ident, $symbol:expr, $name_single:expr, $name_plural:expr,) => {
        $(#[$attribute])*
        pub struct $unit(f64);

        impl Float for $unit {
            const SYMBOL: Option<&str> = Some($symbol);
            const NAME_SINGLE: Option<&str> = Some($name_single);
            const NAME_PLURAL: Option<&str> = Some($name_plural);

            fn new(value: f64) -> Self {
                Self(value)
            }

            fn get(&self) -> f64 {
                self.0
            }
        }

        impl From<f64> for $unit {
            fn from(value: f64) -> Self {
                Self::new(value)
            }
        }

        impl From<$unit> for f64 {
            fn from(value: $unit) -> Self {
                value.get()
            }
        }
    };
    ($(#[$attribute:meta])* $unit:ident, $symbol:expr, $name_single:expr,) => {
        float_type!(
            $(#[$attribute])*
            $unit,
            $symbol,
            $name_single,
            concat!($name_single, "s"),
        );
    };
}

float_type!(
    /// Represents seconds as a floating-point value.
    Seconds,
    "s",
    "second",
);

float_type!(
    /// Represents meters as a floating-point value.
    Meters,
    "m",
    "meter",
);

impl Meters {
    /// Multiply a [`Self`] by this value to produce a [`Centimeters`].
    pub const TO_CENTIMETERS: f64 = 100.0;
}

impl From<Centimeters> for Meters {
    #[inline]
    fn from(value: Centimeters) -> Self {
        Self::new(value.get() * Centimeters::TO_METERS)
    }
}

float_type!(
    /// Represents centimeters as a floating-point value.
    Centimeters,
    "cm",
    "centimeter",
);

impl Centimeters {
    /// Multiply a [`Self`] by this value to produce a [`Meters`].
    pub const TO_METERS: f64 = 1.0 / 100.0;
}

impl From<Meters> for Centimeters {
    #[inline]
    fn from(value: Meters) -> Self {
        Self::new(value.get() * Meters::TO_CENTIMETERS)
    }
}
