// SPDX-License-Identifier: MPL-2.0
//
// Copyright © 2025 RemasteredArch
//
// This Source Code Form is subject to the terms of the Mozilla Public License, version 2.0. If a
// copy of the Mozilla Public License was not distributed with this file, You can obtain one at
// <https://mozilla.org/MPL/2.0/>.

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

macro_rules! float_type {
    ($(#[$attribute:meta])* $unit:ident) => {
        $(#[$attribute])*
        pub struct $unit(f64);

        impl Float for $unit {
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
}

float_type!(
    /// Represents seconds as a floating-point value.
    Seconds
);

float_type!(
    /// Represents meters as a floating-point value.
    Meters
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
    Centimeters
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
