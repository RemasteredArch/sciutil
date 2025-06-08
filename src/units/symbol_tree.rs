use super::Float;

/// Represents a physical unit, used to type an otherwise plain numeric value.
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

// Dummy struct until proper integration
pub struct Seconds;

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
}

/// An empty [`Multiplied`] implementation, used as the tail of a [`UnitList`] to terminate it.
pub struct UnitListNull;

impl Multiplied for UnitListNull {
    fn flatten_units(&self) -> Vec<&dyn Unit> {
        Vec::new()
    }
}

/// An implementation of [`Multiplied`].
#[derive(Clone, Copy)]
pub struct UnitList<T: Unit, V: Multiplied>(T, V);

impl<T: Unit, V: Multiplied> Multiplied for UnitList<T, V> {
    fn flatten_units(&self) -> Vec<&dyn Unit> {
        let mut symbols = self.1.flatten_units();
        symbols.push(&self.0);
        symbols
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

/// Gives an [`f64`] physical units by wrapping it with a [`UnitList`].
///
/// If the given types for the units are zero-sized, then this is a zero-cost wrapper. All sciutil
/// unit types are zero-sized, so a [`Self`] of all sciutil types will be zero-cost.
#[derive(Clone, Copy)]
pub struct ValuedUnit<T: Unit + Sized, V: Multiplied + Sized> {
    value: f64,
    units: UnitList<T, V>,
}

impl<T: Unit + Sized> ValuedUnit<T, UnitListNull> {
    /// Construct a new [`Self`] from an existing [`f64`] and construct the tail end of a
    /// [`UnitList`] using the given [`Unit`].
    pub const fn from_unit(value: f64, unit: T) -> Self {
        Self {
            value,
            units: UnitList::new(unit, UnitListNull),
        }
    }
}

impl<T: Unit + Sized + Copy, V: Multiplied + Sized + Copy> ValuedUnit<T, V> {
    /// Create a new instance of [`ValuedUnit`] with the same value, but with a new unit prepended
    /// onto its chain of [`Multiplied`] [`Unit`]s.
    pub const fn prepend_unit_const<O: Unit + Sized>(
        self,
        unit: O,
    ) -> ValuedUnit<O, UnitList<T, V>> {
        ValuedUnit::from_unit_list(self.value(), UnitList::new(unit, self.units))
    }
}

impl<T: Unit + Sized, V: Multiplied + Sized> ValuedUnit<T, V> {
    pub const fn value(&self) -> f64 {
        self.value
    }

    pub const fn units(&self) -> &UnitList<T, V> {
        &self.units
    }

    /// Construct a new [`Self`] from an existing [`f64`] and [`UnitList`].
    pub const fn from_unit_list(value: f64, units: UnitList<T, V>) -> Self {
        Self { value, units }
    }

    /// Create a new instance of [`ValuedUnit`] with the same value, but with a new unit prepended
    /// onto its chain of [`Multiplied`] [`Unit`]s.
    ///
    /// One should prefer [`Self::prepend_unit_const`] over this where possible, this is only
    /// implemented for the sake of outside implementations or generic users where `T` or `V` may
    /// prevent [`Self`] from being deconstructed in a `const` context.
    pub fn prepend_unit<O: Unit + Sized>(self, unit: O) -> ValuedUnit<O, UnitList<T, V>> {
        ValuedUnit::from_unit_list(self.value(), UnitList::new(unit, self.units))
    }
}

FloatImpl! {
    impl<(
        T: Unit + Default + Sized,
        V: Multiplied + Default + Sized
    )> Float for ValuedUnit<( T, V )> {
        fn new(value: f64) -> Self {
            Self::from_unit_list(value, UnitList::new(T::default(), V::default()))
        }

        fn get(&self) -> f64 {
            self.value()
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
        .prepend(Seconds)
        .prepend(Seconds)
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

    let valued = ValuedUnit::from_unit_list(0.5, list);
    assert_eq!(size_of_val(&valued), size_of::<f64>());

    let big = ValuedUnit::from_unit(0.5, Big::default());
    assert_ne!(size_of_val(&big), size_of::<f64>());
    assert_eq!(size_of_val(&big), 8 + 256);
}
