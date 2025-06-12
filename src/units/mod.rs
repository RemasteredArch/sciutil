// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

//! `units`: Traits and wrapper structs to mark arbitrary floating-point values as physical
//! measurements.
//!
//! In particular, see [`Float`], [`UncertainFloat`], and [`Per`].

#[macro_use]
mod macros;

pub mod composition;

use std::fmt::Display;

use paste::paste;
#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

/// Represents a coherent physical unit, used to type an otherwise plain numeric value.
///
/// Designed to allow generically handling wrapper structs that embed physical units into the type.
/// This is typically with [`ValuedUnit`], which wraps an [`f64`] and a [`UnitList`]. If the
/// [`Self`] implementations of the generic parameters for [`ValuedUnit`] are zero sized, then it
/// will be a zero-cost wrapper. All implementations from sciutil are zero-sized.
pub trait Unit {
    /// The symbol associated with a unit. Sciutil will provide only SI symbols for SI units, but
    /// implementations from other crates may follow different recommendations.
    #[must_use]
    fn symbol(&self) -> String;
}

/// Represents a numeric value with an associated [`Unit`].
pub trait ValuedUnit<T, U: Unit> {
    /// The numeric value represented.
    #[must_use]
    fn value(&self) -> T;

    /// The physical [`Unit`] of the numeric value represented.
    #[must_use]
    fn unit(&self) -> U;
}

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
}

impl Float for f64 {
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
#[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
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

impl<F: Float> Display for UncertainFloat<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ± {}", self.value().get(), self.uncertainty().get())
    }
}

float_types![
    (Day, "d",),
    (Hour, "hr",),
    (Minute, "min",),
    (Second, "s",),
    (Meter, "m",),
    (Centimeter, "cm",),
    (Millimeter, "mm",),
    (Micrometer, "μm",),
    (Degree, "°",),
];

conversions![
    (Seconds * 60.0 = Minutes * 60.0 = Hours * 24.0 = Days),
    (Meters * 100.0 = Centimeters * 10.0 = Millimeters * 1_000.0 = Micrometers),
    // These should really be gotten for free from the above two lines, but doing it manually works
    // for now.
    (Seconds * (Seconds::TO_MINUTES * Minutes::TO_HOURS) = Hours),
    (Seconds * (Seconds::TO_HOURS * Hours::TO_DAYS) = Days),
    (Minutes * (Minutes::TO_HOURS * Hours::TO_DAYS) = Days),
    (Meters * (Meters::TO_CENTIMETERS * Centimeters::TO_MILLIMETERS) = Millimeters),
    (Meters * (Meters::TO_MILLIMETERS * Millimeters::TO_MICROMETERS) = Micrometers),
    (Centimeters * (Centimeters::TO_MILLIMETERS * Millimeters::TO_MICROMETERS) = Micrometers),
];
