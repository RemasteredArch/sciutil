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

use std::{fmt::Display, marker::PhantomData};

use paste::paste;
#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

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
    /// ```rust
    /// # use sciutil::units::{Float, Seconds};
    /// #
    /// assert_eq!(Seconds::NAME_SINGLE, Some("second"))
    /// ```
    const NAME_SINGLE: Option<&str>;

    /// The plural form of the long textual representation of this unit, if there is one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use sciutil::units::{Float, Seconds};
    /// #
    /// assert_eq!(Seconds::NAME_PLURAL, Some("seconds"))
    /// ```
    const NAME_PLURAL: Option<&str>;

    /// The short representation of this unit, if there is one.
    ///
    /// # Examples
    ///
    /// ```rust
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
#[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
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

/// Define a list of standard [`Float`] types. Takes a list of tuples, holding either:
///
/// - `(#[attributes] unit, symbol, name single, name plural)`. The "unit" is the type name, and the
///   others refer to [`Float::SYMBOL`], [`Float::NAME_SINGLE`], and [`Float::NAME_PLURAL`],
///   respectively. A value for [`Float::NAME_PLURAL`] is optional, and will be generated by
///   appending an `'s'` onto [`Float::NAME_SINGLE`] if not explicitly provided.
/// - `(#[attributes] unit single, symbol)`. The "unit single" is the type name in singular form,
///   such that the actual type name can be formed by appending `'s'`, the singular unit name can be
///   generated by setting it to lowercase, and the plural name can be generated by setting it to
///   lowercase and appending `'s'`.
macro_rules! float_types {
    // Takes in a list of definitions, calls the implementation branches of this macro, then checks
    // that all the defined types are `Float + Send + Sync`.
    [ $((
        $(#[$attribute:meta])* $unit:ident, $( $rest:tt, )+
    ),)+ ] => {
        $(
            float_types!(
                @
                $(#[$attribute])*
                $unit,
                $($rest,)+
            );
        )+

        #[cfg(test)]
        mod _float_types_test {
            #[test]
            fn check_send_sync() {
                fn assert_send_sync<F: Send + Sync>() {}

                $( float_types!(@test $unit, $( $rest, )+); )+
            }
        }
    };

    // For explicitly defined types, just drop in the `$unit` identifier.
    (@test $unit:ident, $symbol:expr, $( $rest:tt, )+) => {
        ::paste::paste! {
            assert_send_sync::<super::$unit>();
            assert_send_sync::<super::Per<super::$unit, super::$unit, 1>>();
            assert_send_sync::<super::UncertainFloat<super::$unit>>();
        }
    };

    // For types generated from a singular string, append `'s'` to the `$unit` identifier.
    (@test $unit:ident, $symbol:expr,) => {
        ::paste::paste! {
            // This is now a plural identifier, call the regular branch for plural identifiers.
            float_types!(@test [<$unit s>], _, _,);
        }
    };


    // Matches on declarations that just have a unit identifier and a symbol, such that the
    // identifier can have an `'s'` appended to form the type's identifier, be set to lowercase to
    // form the unit's name, and be set to lowercase and an `'s'` appended to form the unit's plural
    // name. Also generates a documentation comment with a basic description.
    (@ $(#[$attribute:meta])* $unit_single:ident, $symbol:expr,) => {
        paste! {
            float_types!(
                @
                #[doc = "Represents " $unit_single:lower "s as a floating-point value."]
                $(#[$attribute])*
                [<$unit_single s>],
                $symbol,
                stringify!([< $unit_single:lower >]),
            );
        }
    };

    // Defines a [`Float`] type.
    (@ $(#[$attribute:meta])* $unit:ident, $symbol:expr, $name_single:expr, $name_plural:expr,) => {
        $(#[$attribute])*
        #[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
        #[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
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

    // Shorthand for a [`Float`] type definition for when the plural form of a unit is that unit
    // with an `'s'` at the end.
    (@ $(#[$attribute:meta])* $unit:ident, $symbol:expr, $name_single:expr,) => {
        float_types!(
            @
            $(#[$attribute])*
            $unit,
            $symbol,
            $name_single,
            concat!($name_single, "s"),
        );
    };
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
];

/// Defines conversions between units. Takes in a list of parentheses-wrapped conversions. For each
/// conversion, it expects `FromType * factor = ToType`. The factor may be wrapped in parentheses to
/// allow for something more complex than an [`f64`] literal. A conversion may also include
/// chaining, as in `FromType * factor = ToType * next_factor = NextToType`, which creates
/// conversions between `FromType * factor = ToType` and `ToType * next_factor = NextToType`. This
/// can be chained forever. In the future, I would also like it to create conversions between
/// _every_ item in the chain, e.g., `FromType * factor * next_factor = NextToType`.
///
/// A "conversion" of `FromType * factor = ToType` creates:
///
/// 1. `impl From<$FromType> for $ToType`
/// 2. `impl From<$ToType> for $FromType`
/// 3. `impl $FromType { const TO_<$TO_TYPE>: f64 = $factor; }`
/// 3. `impl $ToType { const TO_<$FROM_TYPE>: f64 = 1.0 / $factor; }`
macro_rules! conversions {
    // A series of conversions.
    [$(
        ( $(#[$attribute:meta])* $from:ident * $($rest:tt)+ ),
    )+] => {
        $(
            conversions!(@ $(#[$attribute])* $from * $($rest)+);
        )+
    };

    // Simple combined declaration, e.g., `Meters * 100.0 = Centimeters`.
    (@ $(#[$attribute:meta])* $from:ident * $factor:literal = $to:ident $(*)?) => {
        conversions!(@ $(#[$attribute])* $from * ($factor) = $to);
    };

    // Simple combined declaration with a more arbitrary factor, e.g.,
    // `Meters * (1.0 / 100.0) = Centimeters`.
    (@ $(#[$attribute:meta])* $from:ident * ($( $factor:tt )+) = $to:ident) => {
        conversions!(@@ $(#[$attribute])* $from * ($($factor)+) = $to);
        conversions!(@@ $(#[$attribute])* $to * (1.0 / $($factor)+) = $from);
    };

    // Complex combined declaration, recursively defining conversion factors.
    (@ $(#[$attribute:meta])* $from:ident * $factor:literal = $to:ident * $($rest:tt)+) => {
        conversions!(@ $(#[$attribute])* $from * ($factor) = $to);
        conversions!(@ $(#[$attribute])* $to * $( $rest )+);
    };

    // Complex combined declaration, recursively defining conversion factors, with more arbitrary
    // factors.
    (@ $(#[$attribute:meta])* $from:ident * ($( $factor:tt )+) = $to:ident * $($rest:tt)+) => {
        conversions!(@ $(#[$attribute])* $from * ($( $factor )+) = $to);
        conversions!(@ $(#[$attribute])* $to * $( $rest )+);
    };

    // Declaration of a single conversion of `from * factor = to`. Implements a const called
    // `TO_$TO` and implements `From<$from> for $to`.
    (@@ $(#[$attribute:meta])* $from:ident * ($( $factor:tt )+) = $to:ident) => {
        paste! {
            impl $from {
                #[doc = "Multiply a [`Self`] by this value to produce a [`" $to "`]."]
                $(#[$attribute])*
                pub const [<TO_ $to:snake:upper>]: f64 = $($factor)+;
            }

            impl From<$from> for $to {
                #[inline]
                fn from(value: $from) -> Self {
                    Self::new(value.get() * $from::[<TO_ $to:snake:upper>])
                }
            }
        }
    };
}

conversions![
    (Seconds * 60.0 = Minutes * 60.0 = Hours * 24.0 = Days),
    (Meters * 100.0 = Centimeters * 10.0 = Millimeters * (1.0 / 1_000.0) = Micrometers),
    // These should really be gotten for free from the above two lines, but doing it manually works
    // for now.
    (Seconds * (Seconds::TO_MINUTES * Minutes::TO_HOURS) = Hours),
    (Seconds * (Seconds::TO_HOURS * Hours::TO_DAYS) = Days),
    (Minutes * (Minutes::TO_HOURS * Hours::TO_DAYS) = Days),
    (Meters * (Meters::TO_CENTIMETERS * Centimeters::TO_MILLIMETERS) = Millimeters),
    (Meters * (Meters::TO_MILLIMETERS * Millimeters::TO_MICROMETERS) = Micrometers),
    (Centimeters * (Centimeters::TO_MILLIMETERS * Millimeters::TO_MICROMETERS) = Micrometers),
];
