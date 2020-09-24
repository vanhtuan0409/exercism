// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR: f64 = 365.25 * (24 * 60 * 60) as f64;

macro_rules! planet_impl {
    ($factor:expr) => {
        fn years_during(d: &Duration) -> f64 {
            d.0 as f64 / (EARTH_YEAR * $factor)
        }
    };
}

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s)
    }
}

pub trait Planet {
    planet_impl!(1 as f64);
}

pub struct Mercury;
impl Planet for Mercury {
    planet_impl!(0.2408467);
}
pub struct Venus;
impl Planet for Venus {
    planet_impl!(0.61519726);
}
pub struct Earth;
impl Planet for Earth {}
pub struct Mars;
impl Planet for Mars {
    planet_impl!(1.8808158);
}
pub struct Jupiter;
impl Planet for Jupiter {
    planet_impl!(11.862615);
}
pub struct Saturn;
impl Planet for Saturn {
    planet_impl!(29.447498);
}
pub struct Uranus;
impl Planet for Uranus {
    planet_impl!(84.016846);
}
pub struct Neptune;
impl Planet for Neptune {
    planet_impl!(164.79132);
}
