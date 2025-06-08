use crate::units::{FloatDisplay, Meters, Seconds};

use super::Multiplied;

#[test]
fn multiplied() {
    type SecondMeters = Multiplied<Seconds, Meters>;
    type SecondMetersSecondMeters = Multiplied<SecondMeters, SecondMeters>;

    assert_eq!(SecondMeters::symbol(), "s * m");
    assert_eq!(SecondMeters::name_single(), "second-meter");
    assert_eq!(SecondMeters::name_plural(), "second-meter");

    assert_eq!(SecondMetersSecondMeters::symbol(), "s * m * s * m");
    assert_eq!(
        SecondMetersSecondMeters::name_single(),
        "second-meter-second-meter"
    );
    assert_eq!(
        SecondMetersSecondMeters::name_plural(),
        "second-meter-second-meter"
    );
}
