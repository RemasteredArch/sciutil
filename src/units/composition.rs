use super::{Float, Unit};

// Dummy struct until proper integration
struct Seconds;

impl From<crate::units::Seconds> for Seconds {
    fn from(_: crate::units::Seconds) -> Self {
        Self
    }
}

impl Unit for Seconds {
    fn symbol(&self) -> String {
        "s".to_string()
    }
}

/// Represents multiple [`Unit`]s multiplied in a chain.
///
/// For example, a value in Newton-meters would have a [`Self`] implementation returns a [`Vec`] of
/// two [`Unit`]s, representing Newtons multiplied by meters.
///
/// The intended design for [`Self`] is that implementors are low- or zero-cost, allowing for cheap
/// wrappers over numeric values, with all the cost of allocation and computation incurred during
/// [`Self::flatten_units`], but implementors may make other decisions. For example, implementors
/// with low volumes of data but high frequency of display may prefer to store/cache their output
/// for [`Self::flatten_units`] to make it cheaper to run repeatedly.
pub trait Multiplied {
    /// Flatten the nested structure of [`Self`], returning an unordered [`Vec`] of all the
    /// contained [`Unit`]s.
    #[must_use]
    fn flatten_units(&self) -> Vec<&dyn Unit>;

    /// Flattened the nested structure of [`Self`] to return the symbol for the entire composite
    /// unit represented by [`Self`].
    ///
    /// The default implementation uses [`Self::flatten_units`] to construct a space-separated list
    /// of outputs from [`Unit::symbol`] to represent their multiplication.
    #[must_use]
    fn flatten_symbols(&self) -> String {
        let units = self.flatten_units();
        // Will have at least one bytes per symbol, then one byte per symbol for the separating
        // space, then one less byte because there is no trailing space.
        let mut str = String::with_capacity(units.len() * 2 - 1);

        for unit in units {
            str.push_str(&unit.symbol());
            str.push(' ');
        }
        let _ = str.pop(); // Remove the trailing space.

        str
    }
}

/// An empty [`Multiplied`] implementation, used as the tail of a [`UnitList`] to terminate it.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct UnitListNull;

impl Multiplied for UnitListNull {
    fn flatten_units(&self) -> Vec<&dyn Unit> {
        Vec::new()
    }
}

/// An implementation of [`Multiplied`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct UnitList<T: Unit, V: Multiplied>(T, V);

impl<T: Unit, V: Multiplied> Multiplied for UnitList<T, V> {
    fn flatten_units(&self) -> Vec<&dyn Unit> {
        let mut symbols = self.1.flatten_units();
        symbols.push(&self.0);
        symbols
    }
}

impl<T: Unit, V: Multiplied> Unit for UnitList<T, V> {
    fn symbol(&self) -> String {
        self.flatten_symbols()
    }
}

impl<T: Unit, V: Multiplied> UnitList<T, V> {
    /// Create a new instance of [`Self`].
    #[must_use]
    pub const fn new(unit: T, rest: V) -> Self {
        Self(unit, rest)
    }

    /// Prepend another [`Unit`] onto the existing chain.
    #[must_use]
    pub const fn prepend<O: Unit>(self, other: O) -> UnitList<O, Self> {
        UnitList::new(other, self)
    }
}

/// Gives an `T` physical units.
///
/// If the given types for the units are zero-sized, then this is a zero-cost wrapper. All sciutil
/// unit types are zero-sized, so a [`Self`] of all sciutil types will be zero-cost.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Valued<T, U: Unit> {
    value: T,
    unit: U,
}

impl<T, U: Unit> super::ValuedUnit<T, U> for Valued<T, U> {
    fn value(&self) -> T {
        todo!()
    }

    fn unit(&self) -> U {
        todo!()
    }
}

impl<T, U: Unit> Valued<T, U> {
    /// Construct a new [`Self`] from an existing `T` and `U`.
    pub const fn from_unit(value: T, unit: U) -> Self {
        Self { value, unit }
    }
}

impl<T, U: Unit + Default> Valued<T, U> {
    /// Construct a new [`Self`] from an existing `T` and the [`Default`] value for `U`.
    pub fn from_unit_default(value: T) -> Self {
        Self {
            value,
            unit: U::default(),
        }
    }
}

// TODO: remove the `+ Multiplied`?
impl<T: Sized + Copy, U: Unit + Sized + Copy + Multiplied> Valued<T, U> {
    /// Create a new instance of [`Valued`] with the same value, but with a new unit prepended onto
    /// its chain of [`Multiplied`] [`Unit`]s.
    pub const fn prepend_unit_const<O: Unit + Sized>(self, unit: O) -> Valued<T, UnitList<O, U>> {
        Valued::from_unit(self.value, UnitList::new(unit, self.unit))
    }
}

impl<T: Sized, U: Unit + Sized> Valued<T, U> {
    pub const fn value(&self) -> &T {
        &self.value
    }

    pub const fn unit(&self) -> &U {
        &self.unit
    }
}

// TODO: remove the `+ Multiplied`?
impl<T: Sized, U: Unit + Sized + Multiplied> Valued<T, U> {
    /// Create a new instance of [`Valued`] with the same value, but with a new unit prepended onto
    /// its chain of [`Multiplied`] [`Unit`]s.
    ///
    /// One should prefer [`Self::prepend_unit_const`] over this where possible, this is only
    /// implemented for the sake of outside implementations or generic users where `T` or `V` may
    /// prevent [`Self`] from being deconstructed in a `const` context.
    pub fn prepend_unit<O: Unit + Sized>(self, unit: O) -> Valued<T, UnitList<O, U>> {
        Valued::from_unit(self.value, UnitList::new(unit, self.unit))
    }
}

FloatImpl! {
    impl<( U: Unit + Default + Sized )> Float for Valued<( f64, U )> {
        fn new(value: f64) -> Self {
            Self { value, unit: U::default() }
        }

        fn get(&self) -> f64 {
            *self.value()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Power<T: Unit, const P: i32>(T);

impl<T: Unit, const P: i32> Power<T, P> {
    #[must_use]
    pub const fn power() -> i32 {
        P
    }
}

impl<T: Unit, const P: i32> Unit for Power<T, P> {
    fn symbol(&self) -> String {
        if P.is_positive() {
            format!("{}^{P}", self.0.symbol())
        } else {
            format!("{}^({P})", self.0.symbol())
        }
    }
}

// Dummy implementation for testing.
#[expect(dead_code, reason = "used only for testing type sizes")]
#[derive(Copy, Clone)]
pub struct Big([u8; 256]);

impl Multiplied for Big {
    fn flatten_units(&self) -> Vec<&dyn Unit> {
        unimplemented!("used only for testing type sizes")
    }
}

impl Unit for Big {
    fn symbol(&self) -> String {
        unimplemented!("used only for testing type sizes")
    }
}

#[expect(
    clippy::derivable_impls,
    reason = "you cannot `#[derive(Default)]` on an array of size 256"
)]
impl Default for Big {
    fn default() -> Self {
        Self([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    }
}

#[test]
fn func() {
    let list = UnitList::new(Seconds, UnitListNull)
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Power::<_, 2>(Seconds))
        .prepend(Seconds)
        .prepend(Power::<_, -2>(Seconds))
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Seconds)
        .prepend(Seconds);

    let flattened = list.flatten_units();
    for unit in flattened {
        println!("{}", unit.symbol());
    }
    assert_eq!(
        list.flatten_symbols(),
        "s s s s^2 s s^(-2) s s s s s s s s s"
    );

    let valued = Valued::from_unit(0.5, list);
    assert_eq!(size_of_val(&valued), size_of::<f64>());

    let big = Valued::from_unit(0.5, Big::default());
    assert_ne!(size_of_val(&big), size_of::<f64>());
    assert_eq!(size_of_val(&big), 8 + 256);
}
