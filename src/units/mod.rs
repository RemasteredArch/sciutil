// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `units`: Wrapper structs to mark arbitrary floating-point values as SI units.
//!
//! In particular, see [`Float`], [`UncertainFloat`], and [`Per`].

use std::{fmt::Display, marker::PhantomData};

/// Contains or represents a floating-point value, optionally with physical units.
///
/// Designed to allow generically handling wrapper structs that embed physical units into the type,
/// such as [`Seconds`] marking an otherwise arbitrary [`f64`] as representing the SI unit of
/// seconds. This unit information is optional, so that plain [`f64`] (and other unit-less types)
/// can implement it.
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
/// # use sciutil::units::UncertainFloat;
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

/// A wrapper struct to show the dependence on one unit by another.
///
/// ```rust
/// # use sciutil::units::{Float, Meters, Per, Seconds};
/// #
/// // Formats nicely:
/// type Acceleration = Per<Meters, Seconds, 2>;
/// let accel = Acceleration::new(5.05);
/// assert_eq!(accel.to_string(), "5.05 meters per second squared");
///
/// // Either parameter not having a name disables any units:
/// assert_eq!(Per::<Meters, f64, 2>::new(5.05).to_string(), "5.05");
///
/// // More power-dependent formatting:
/// assert_eq!(Per::<Meters, Seconds, 0>::new(5.05).to_string(), "5.05 meters");
/// assert_eq!(Per::<Meters, Seconds, 1>::new(5.05).to_string(), "5.05 meters per second");
/// assert_eq!(Per::<Meters, Seconds, 3>::new(5.05).to_string(), "5.05 meters per second cubed");
/// assert_eq!(Per::<Meters, Seconds, 4>::new(5.05).to_string(), "5.05 meters per second^4");
/// ```
pub struct Per<F: Float, T: Float, const P: usize>(f64, PhantomData<F>, PhantomData<T>);

impl<F: Float, T: Float, const P: usize> Per<F, T, P> {
    /// Returns the full pretty name of the unit represented by [`Self`], if there is one.
    ///
    /// ```rust
    /// # use sciutil::units::{Float, Meters, Per, Seconds};
    /// #
    /// // Formats nicely:
    /// type Acceleration = Per<Meters, Seconds, 2>;
    /// assert_eq!(Acceleration::name(), Some("meters per second squared".to_string()));
    ///
    /// // Either parameter not having a name disables any units:
    /// assert_eq!(Per::<Meters, f64, 2>::name(), None);
    ///
    /// // More power-dependent formatting:
    /// assert_eq!(Per::<Meters, Seconds, 0>::name(), Some("meters".to_string()));
    /// assert_eq!(Per::<Meters, Seconds, 1>::name(), Some("meters per second".to_string()));
    /// assert_eq!(Per::<Meters, Seconds, 3>::name(), Some("meters per second cubed".to_string()));
    /// assert_eq!(Per::<Meters, Seconds, 4>::name(), Some("meters per second^4".to_string()));
    /// ```
    #[must_use]
    pub fn name() -> Option<String> {
        let dependent = F::NAME_PLURAL?;
        if P == 0 {
            return Some(dependent.to_string());
        }
        let independent = T::NAME_SINGLE?;

        Some(format!(
            "{dependent} per {independent}{}",
            match P {
                1 => String::new(),
                2 => " squared".to_string(),
                3 => " cubed".to_string(),
                _ => format!("^{P}"),
            }
        ))
    }

    /// Returns the short, symbolic textual representation of the unit represented by [`Self`], if
    /// there is one.
    ///
    /// ```rust
    /// # use sciutil::units::{Float, Meters, Per, Seconds};
    /// #
    /// // Formats nicely:
    /// type Acceleration = Per<Meters, Seconds, 2>;
    /// assert_eq!(Acceleration::symbol(), Some("m / s^2".to_string()));
    ///
    /// // Either parameter not having a symbol disables any units:
    /// assert_eq!(Per::<Meters, f64, 2>::symbol(), None);
    ///
    /// // More power-dependent formatting:
    /// assert_eq!(Per::<Meters, Seconds, 0>::symbol(), Some("m".to_string()));
    /// assert_eq!(Per::<Meters, Seconds, 1>::symbol(), Some("m / s".to_string()));
    /// assert_eq!(Per::<Meters, Seconds, 3>::symbol(), Some("m / s^3".to_string()));
    /// assert_eq!(Per::<Meters, Seconds, 4>::symbol(), Some("m / s^4".to_string()));
    /// ```
    #[must_use]
    pub fn symbol() -> Option<String> {
        let dependent = F::SYMBOL?;
        if P == 0 {
            return Some(dependent.to_string());
        }
        let independent = T::SYMBOL?;

        Some(format!(
            "{dependent} / {independent}{}",
            match P {
                1 => String::new(),
                _ => format!("^{P}"),
            }
        ))
    }
}

impl<F: Float, T: Float, const P: usize> Display for Per<F, T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.get(),
            Self::name().map_or(String::new(), |s| format!(" {s}"))
        )
    }
}

impl<F: Float, T: Float, const P: usize> Float for Per<F, T, P> {
    /// See [`Self::name`] instead.
    const NAME_SINGLE: Option<&str> = None;
    /// See [`Self::name`] instead.
    const NAME_PLURAL: Option<&str> = None;
    /// See [`Self::symbol`] instead.
    const SYMBOL: Option<&str> = None;

    fn new(value: f64) -> Self {
        Self(value, PhantomData::<F>, PhantomData::<T>)
    }

    fn get(&self) -> f64 {
        self.0
    }
}

impl<T: Float, F: Float, const P: usize> From<f64> for Per<F, T, P> {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl<T: Float, F: Float, const P: usize> From<Per<F, T, P>> for f64 {
    fn from(value: Per<F, T, P>) -> Self {
        value.get()
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
