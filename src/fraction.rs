//! Types and constants for handling fractions

use super::measurement::*;

#[cfg(feature = "from_str")]
use crate::impl_from_str;

/// The 'Fraction' struct can be used to deal with fractions in a common way.
///
/// In comparison to other measurements, fractions can be taken of all other available measurements.
/// See the example below where 0.1% of 1 kV is taken, which returns 1 V.
///
/// Note: The base "unit" for a fraction is simply a decimal number and thus it is unitless.
///
/// # Example
///
/// ```
/// use measurements::{Fraction, Voltage};
///
/// let f = Fraction::from_percent(0.1);
/// let u = Voltage::from_kilovolts(1.0);
///
/// // Take a fraction of any other measurement:
/// let uf = u * f;
/// assert_eq!(uf, Voltage::from_volts(1.0));
///
/// // Convert fraction to another notation:
/// assert_eq!(f.as_ppm(), 1.0e3);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Fraction {
    decimal: f64,
}

impl Fraction {
    /// Create a new fraction from a decimal value.
    pub fn from_decimal(decimal: f64) -> Self {
        Fraction { decimal }
    }

    /// Create a new fraction from a percent value.
    pub fn from_percent(percent: f64) -> Self {
        Self::from_decimal(percent / 100.0)
    }

    /// Create a new fraction from a permil value.
    pub fn from_permil(permil: f64) -> Self {
        Self::from_decimal(permil / 1_000.0)
    }

    /// Create a new fraction from a permyriad value.
    pub fn from_permyriad(permyriad: f64) -> Self {
        Self::from_decimal(permyriad / 10_000.0)
    }

    /// Create a new fraction from a per hundred thousand (per cent mille) value.
    pub fn from_pcm(pcm: f64) -> Self {
        Self::from_decimal(pcm / 100_000.0)
    }

    /// Create a new fraction from a parts-per million (ppm) value.
    pub fn from_ppm(ppm: f64) -> Self {
        Self::from_decimal(ppm / 1_000_000.0)
    }

    /// Create a new fraction from a parts-per billion (ppb) value.
    pub fn from_ppb(ppb: f64) -> Self {
        Self::from_decimal(ppb / 1_000_000_000.0)
    }

    /// Create a new fraction from a parts-per trillion (ppt) value.
    pub fn from_ppt(ppt: f64) -> Self {
        Self::from_decimal(ppt / 1_000_000_000_000.0)
    }

    /// Create a new fraction from a parts-per quadrillion (ppq) value.
    pub fn from_ppq(ppq: f64) -> Self {
        Self::from_decimal(ppq / 1_000_000_000_000_000.0)
    }

    /// Convert this fraction into a floating point value in percent.
    pub fn as_percent(&self) -> f64 {
        self.decimal * 100.0
    }

    /// Convert this fraction into a floating point decimal value.
    pub fn as_decimal(&self) -> f64 {
        self.decimal
    }

    /// Convert this fraction into a floating point value in permil.
    pub fn as_permil(&self) -> f64 {
        self.decimal * 1_000.0
    }

    /// Convert this fraction into a floating point value in permyriad.
    pub fn as_permyriad(&self) -> f64 {
        self.decimal * 10_000.0
    }

    /// Convert this fraction into a floating point value in per hundred thousand (per cent mille).
    pub fn as_pcm(&self) -> f64 {
        self.decimal * 100_000.0
    }
    /// Convert this fraction into a floating point value in parts-per million (ppm).
    pub fn as_ppm(&self) -> f64 {
        self.decimal * 1_000_000.0
    }

    /// Convert this fraction into a floating point value in parts-per billion (ppb).
    pub fn as_ppb(&self) -> f64 {
        self.decimal * 1_000_000_000.0
    }

    /// Convert this fraction into a floating point value in parts-per trillion (ppt).
    pub fn as_ppt(&self) -> f64 {
        self.decimal * 1_000_000_000_000.0
    }

    /// Convert this fraction into a floating point value in parts-per quadrillion (from_ppq).
    pub fn as_ppq(&self) -> f64 {
        self.decimal * 1_000_000_000_000_000.0
    }
}

impl Measurement for Fraction {
    fn get_base_units_name(&self) -> &'static str {
        ""
    }

    fn as_base_units(&self) -> f64 {
        self.decimal
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_decimal(units)
    }

    fn get_appropriate_units(&self) -> (&'static str, f64) {
        let list = [
            ("ppq", 1e-15),
            ("ppt", 1e-12),
            ("ppb", 1e-9),
            ("ppm", 1e-6),
            ("‰", 1e-3),
            ("%", 1e-2),
        ];
        self.pick_appropriate_units(&list)
    }
}

implement_measurement! { Fraction }

/// Implement math between all other units and fraction.
///
/// We should be allowed to take, e.g., 10% of 1 kV, which should be equal to 100 V.
macro_rules! impl_math_fractions {
    ($($t:ty)*) => ($(

        // Multiplying a measurement with a fraction should return a measurement.
        impl ::core::ops::Mul<Fraction> for $t {
            type Output = Self;

            fn mul(self, rhs: Fraction) -> Self {
                Self::from_base_units(self.as_base_units() * rhs.as_base_units())
            }
        }

        // Multiplication is commutative.
        impl ::core::ops::Mul<$t> for Fraction {
            type Output = $t;

            fn mul(self, rhs: $t) -> $t {
                <$t>::from_base_units(self.as_base_units() * rhs.as_base_units())
            }
        }

        // Division of a measurement with a fraction.
        impl ::core::ops::Div<Fraction> for $t {
            type Output = Self;

            fn div(self, rhs: Fraction) -> Self {
                Self::from_base_units(self.as_base_units() / rhs.as_base_units())
            }
        }

    )*)
}

impl_math_fractions! {
    crate::Acceleration
    crate::Angle
    crate::AngularVelocity
    crate::Area
    crate::Current
    crate::Data
    crate::Distance
    crate::Density
    crate::Energy
    crate::Force
    crate::Frequency
    crate::Humidity
    crate::Mass
    crate::Power
    crate::Pressure
    crate::Resistance
    crate::Speed
    crate::Temperature
    crate::TemperatureDelta
    crate::Torque
    crate::TorqueEnergy
    crate::Volume
    crate::Voltage
}

#[cfg(feature = "from_str")]
impl_from_str! {
    Fraction,
    Fraction::from_decimal,
    (Fraction::from_percent, "%", "percent"),
    (Fraction::from_permil, "‰", "permil"),
    (Fraction::from_permyriad, "‱", "permyriad"),
    (Fraction::from_pcm, "pcm"),
    (Fraction::from_ppm, "ppm"),
    (Fraction::from_ppb, "ppb"),
    (Fraction::from_ppt, "ppt"),
    (Fraction::from_ppq, "ppq"),
}

#[cfg(test)]
mod test {
    use crate::{fraction::*, test_utils::assert_almost_eq};
    #[cfg(feature = "from_str")]
    use core::str::FromStr;

    #[test]
    fn as_decimal() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_decimal(), 0.01);
    }

    #[test]
    fn as_percent() {
        let u = Fraction::from_decimal(0.01);
        assert_almost_eq(u.as_percent(), 1.0);
    }

    #[test]
    fn as_permil() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_permil(), 10.0);
    }

    #[test]
    fn as_permyriad() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_permyriad(), 100.0);
    }

    #[test]
    fn as_pcm() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_pcm(), 1_000.0);
    }

    #[test]
    fn as_ppm() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_ppm(), 10_000.0);
    }

    #[test]
    fn as_ppb() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_ppb(), 10.0e6);
    }

    #[test]
    fn as_ppt() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_ppt(), 10.0e9);
    }

    #[test]
    fn as_ppq() {
        let u = Fraction::from_percent(1.0);
        assert_almost_eq(u.as_ppq(), 10.0e12);
    }

    #[test]
    fn from_permil() {
        let u = Fraction::from_permil(1.0);
        assert_almost_eq(u.as_percent(), 0.1);
    }

    #[test]
    fn from_permyriad() {
        let u = Fraction::from_permyriad(1.0);
        assert_almost_eq(u.as_percent(), 0.01);
    }

    #[test]
    fn from_pcm() {
        let u = Fraction::from_pcm(1.0);
        assert_almost_eq(u.as_percent(), 0.001);
    }

    #[test]
    fn from_ppm() {
        let u = Fraction::from_ppm(1.0);
        assert_almost_eq(u.as_percent(), 1.0e-4);
    }

    #[test]
    fn from_ppb() {
        let u = Fraction::from_ppb(1.0);
        assert_almost_eq(u.as_percent(), 1.0e-7);
    }

    #[test]
    fn from_ppt() {
        let u = Fraction::from_ppt(1.0);
        assert_almost_eq(u.as_percent(), 1.0e-10);
    }

    #[test]
    fn from_ppq() {
        let u = Fraction::from_ppq(1.0);
        assert_almost_eq(u.as_percent(), 1.0e-13);
    }

    // Measurement traits.
    #[test]
    fn add() {
        let u = Fraction::from_percent(10.0);
        let v = Fraction::from_permil(10.0);
        let res = u + v;
        assert_almost_eq(res.as_decimal(), 0.11);
    }

    #[test]
    fn sub() {
        let u = Fraction::from_percent(10.0);
        let v = Fraction::from_permil(10.0);
        let res = u - v;
        assert_almost_eq(res.as_decimal(), 0.09);
    }

    #[test]
    fn mul() {
        let u = Fraction::from_percent(10.0);
        let f = 4.0;

        let exp = 0.4;

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_decimal(), exp);
        assert_almost_eq(res2.as_decimal(), exp);
    }

    #[test]
    fn div() {
        let u = Fraction::from_percent(10.0);
        let v = Fraction::from_permil(20.0);
        let res_f64 = u / v;
        let res_frac = u / 2.0;

        assert_almost_eq(res_f64, 5.0);
        assert_almost_eq(res_frac.as_percent(), 5.0);
    }

    #[test]
    fn eq() {
        let u = Fraction::from_decimal(1.0);
        let v = Fraction::from_ppm(1.0e6);

        assert_eq!(u, v);
    }

    #[test]
    fn neq() {
        let u = Fraction::from_decimal(1.0);
        let v = Fraction::from_ppm(1.0);

        assert_ne!(u, v);
    }

    #[test]
    fn cmp() {
        let u = Fraction::from_percent(10.0);
        let v = Fraction::from_permil(10.0);

        assert!(u > v);
        assert!(u >= v);
        assert!(!(u < v));
        assert!(!(u <= v));
    }

    // Operations with other units.
    #[test]
    fn acceleration() {
        let u = crate::Acceleration::from_meters_per_second_per_second(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_meters_per_second_per_second(), 0.001);
        assert_almost_eq(res2.as_meters_per_second_per_second(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_meters_per_second_per_second(), 1_000.0);
    }

    #[test]
    fn angle() {
        let u = crate::Angle::from_radians(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_radians(), 0.001);
        assert_almost_eq(res2.as_radians(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_radians(), 1_000.0);
    }

    #[test]
    fn angular_velocity() {
        let u = crate::AngularVelocity::from_hertz(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_hertz(), 0.001);
        assert_almost_eq(res2.as_hertz(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_hertz(), 1_000.0);
    }

    #[test]
    fn area() {
        let u = crate::Area::from_square_meters(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_square_meters(), 0.001);
        assert_almost_eq(res2.as_square_meters(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_square_meters(), 1_000.0);
    }

    #[test]
    fn current() {
        let u = crate::Current::from_amperes(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_milliamperes(), 1.0);
        assert_almost_eq(res2.as_milliamperes(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_amperes(), 1_000.0);
    }

    #[test]
    fn data() {
        let u = crate::Data::from_kilooctets(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_octets(), 1.0);
        assert_almost_eq(res2.as_octets(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_megaoctets(), 1.0);
    }

    #[test]
    fn density() {
        let u = crate::Density::from_kilograms_per_cubic_meter(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_kilograms_per_cubic_meter(), 0.001);
        assert_almost_eq(res2.as_kilograms_per_cubic_meter(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_kilograms_per_cubic_meter(), 1_000.0);
    }

    #[test]
    fn distance() {
        let u = crate::Distance::from_meters(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = u * f;

        assert_almost_eq(res1.as_millimeters(), 1.0);
        assert_almost_eq(res2.as_millimeters(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_kilometers(), 1.0);
    }

    #[test]
    fn energy() {
        let u = crate::Energy::from_joules(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_joules(), 0.001);
        assert_almost_eq(res2.as_joules(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_joules(), 1_000.0);
    }

    #[test]
    fn force() {
        let u = crate::Force::from_newtons(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_millinewtons(), 1.0);
        assert_almost_eq(res2.as_millinewtons(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_newtons(), 1_000.0);
    }

    #[test]
    fn frequency() {
        let u = crate::Frequency::from_hertz(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_millihertz(), 1.0);
        assert_almost_eq(res2.as_millihertz(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_kilohertz(), 1.0);
    }

    #[test]
    fn humidty() {
        let u = crate::Humidity::from_percent(10.0);
        let f = Fraction::from_percent(10.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_percent(), 1.0);
        assert_almost_eq(res2.as_percent(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_percent(), 100.0);
    }

    #[test]
    fn mass() {
        let u = crate::Mass::from_kilograms(1.0);
        let f = Fraction::from_percent(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_grams(), 10.0);
        assert_almost_eq(res2.as_grams(), 10.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_kilograms(), 100.0);
    }

    #[test]
    fn power() {
        let u = crate::Power::from_watts(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_milliwatts(), 1.0);
        assert_almost_eq(res2.as_milliwatts(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_kilowatts(), 1.0);
    }

    #[test]
    fn pressure() {
        let u = crate::Pressure::from_bars(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_millibars(), 1.0);
        assert_almost_eq(res2.as_millibars(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_bars(), 1_000.0);
    }

    #[test]
    fn resistance() {
        let u = crate::Resistance::from_ohms(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_ohms(), 0.001);
        assert_almost_eq(res2.as_ohms(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_kiloohms(), 1.0);
    }

    #[test]
    fn speed() {
        let u = crate::Speed::from_meters_per_second(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_meters_per_second(), 0.001);
        assert_almost_eq(res2.as_meters_per_second(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_meters_per_second(), 1_000.0);
    }

    #[test]
    fn temperature() {
        let u = crate::Temperature::from_kelvin(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_kelvin(), 0.001);
        assert_almost_eq(res2.as_kelvin(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_kelvin(), 1_000.0);
    }

    #[test]
    fn temperature_delta() {
        let u = crate::TemperatureDelta::from_kelvin(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_kelvin(), 0.001);
        assert_almost_eq(res2.as_kelvin(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_kelvin(), 1_000.0);
    }

    #[test]
    fn torque() {
        let u = crate::Torque::from_newton_meters(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_newton_meters(), 0.001);
        assert_almost_eq(res2.as_newton_meters(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_newton_meters(), 1_000.0);
    }

    #[test]
    fn torque_energy() {
        let u = crate::TorqueEnergy::from_base_units(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_base_units(), 0.001);
        assert_almost_eq(res2.as_base_units(), 0.001);

        let res3 = u / f;

        assert_almost_eq(res3.as_base_units(), 1_000.0);
    }

    #[test]
    fn voltage() {
        let u = crate::Voltage::from_volts(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_millivolts(), 1.0);
        assert_almost_eq(res2.as_millivolts(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_kilovolts(), 1.0);
    }

    #[test]
    fn volume() {
        let u = crate::Volume::from_cubic_meters(1.0);
        let f = Fraction::from_permil(1.0);

        let res1 = u * f;
        let res2 = f * u;

        assert_almost_eq(res1.as_liters(), 1.0);
        assert_almost_eq(res2.as_liters(), 1.0);

        let res3 = u / f;

        assert_almost_eq(res3.as_cubic_meters(), 1_000.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn empty_val_from_str() {
        let v = Fraction::from_str("");
        assert!(v.is_ok());
        assert_eq!(0.0, v.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn percent_from_str() {
        let v = Fraction::from_str("10.0 percent");
        assert!(v.is_ok());
        assert_eq!(0.1, v.unwrap().as_decimal());

        let v2 = Fraction::from_str("10.0%");
        assert!(v2.is_ok());
        assert_eq!(0.1, v2.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn permil_from_str() {
        let v = Fraction::from_str("10.0 permil");
        assert!(v.is_ok());
        assert_eq!(0.01, v.unwrap().as_decimal());

        let v2 = Fraction::from_str("10.0‰");
        assert!(v2.is_ok());
        assert_eq!(0.01, v2.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn permyriad_from_str() {
        let v = Fraction::from_str("10.0 permyriad");
        assert!(v.is_ok());
        assert_eq!(0.001, v.unwrap().as_decimal());

        let v2 = Fraction::from_str("10.0‱");
        assert!(v2.is_ok());
        assert_eq!(0.001, v2.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn pcm_from_str() {
        let v = Fraction::from_str("1.0 pcm");
        assert!(v.is_ok());
        assert_eq!(1.0e-5, v.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn ppm_from_str() {
        let v = Fraction::from_str("1.0 ppm");
        assert!(v.is_ok());
        assert_eq!(1.0e-6, v.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn ppb_from_str() {
        let v = Fraction::from_str("1.0 ppb");
        assert!(v.is_ok());
        assert_eq!(1.0e-9, v.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn ppt_from_str() {
        let v = Fraction::from_str("1.0 ppt");
        assert!(v.is_ok());
        assert_eq!(1.0e-12, v.unwrap().as_decimal());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn ppq_from_str() {
        let v = Fraction::from_str("1.0 ppq");
        assert!(v.is_ok());
        assert_eq!(1.0e-15, v.unwrap().as_decimal());
    }
}
