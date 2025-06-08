#[cfg(test)]
mod test;

use super::{Float, FloatDisplay};

use std::{fmt::Display, marker::PhantomData};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

/// Substrings found in a unit that indicate it would need to be wrapped in parentheses to be raised
/// to a power.
const PRECEDENCE_SPLIT_PATTERNS: [&str; 7] = ["-", " ", "*", "/", "times", "divided by", "per"];

#[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Multiplied<A: Float, B: Float>(f64, PhantomData<A>, PhantomData<B>);

FloatImpl! {
    impl<( A: Float, B: Float )> Float for Multiplied<( A, B )> {
        fn new(value: f64) -> Self {
            Self(value, PhantomData::<A>, PhantomData::<B>)
        }

        fn get(&self) -> f64 {
            self.0
        }
    }
}

impl<A: FloatDisplay, B: FloatDisplay> FloatDisplay for Multiplied<A, B> {
    fn symbol() -> String {
        format!("{} * {}", A::symbol(), B::symbol())
    }

    fn name_single() -> String {
        // Compound units are best hyphenated.
        //
        // <https://english.stackexchange.com/a/177213>
        format!("{}-{}", A::name_single(), B::name_single())
    }

    fn name_plural() -> String {
        // Hyphenated compound units don't use a plural form.
        //
        // <https://english.stackexchange.com/a/177213>
        Self::name_single()
    }
}

impl<A: FloatDisplay, B: FloatDisplay> Display for Multiplied<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.get(), Self::name_single())
    }
}

#[cfg_attr(any(feature = "serde", test), derive(Deserialize, Serialize))]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Power<F: Float, const P: usize>(f64, PhantomData<F>);

FloatImpl! {
    impl<( F: Float, const P: usize )> Float for Power<( F, P )> {
        fn new(value: f64) -> Self {
            Self(value, PhantomData::<F>)
        }

        fn get(&self) -> f64 {
            self.0
        }
    }
}

impl<F: FloatDisplay, const P: usize> FloatDisplay for Power<F, P> {
    fn symbol() -> String {
        if P == 0 {
            return String::new();
        }

        let unit = F::symbol();
        let raised = format!("^{P}");

        if PRECEDENCE_SPLIT_PATTERNS.iter().any(|p| unit.contains(p)) {
            format!("({unit}){raised}")
        } else {
            format!("{unit}{raised}")
        }
    }

    fn name_single() -> String {
        raise_to_power(&F::name_single(), P)
    }

    fn name_plural() -> String {
        raise_to_power(&F::name_plural(), P)
    }
}

fn raise_to_power(value: &str, p: usize) -> String {
    if p == 0 {
        return String::new();
    }

    let raised = match p {
        1 => String::new(),
        2 => " squared".to_string(),
        3 => " cubed".to_string(),
        _ => format!("^{p}"),
    };

    if PRECEDENCE_SPLIT_PATTERNS.iter().any(|p| value.contains(p)) {
        format!("({value}){raised}")
    } else {
        format!("{value}{raised}")
    }
}

impl<F: FloatDisplay, const P: usize> Display for Power<F, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.get(), Self::name_plural())
    }
}
