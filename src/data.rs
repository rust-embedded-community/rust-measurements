//! Types and constants for handling amounts of data (in octets, or bits).

use super::measurement::*;

#[cfg(feature = "from_str")]
use crate::impl_from_str;

// Constants
const OCTET_BIT_FACTOR: f64 = 0.125;

// Constants, legacy
const OCTET_KILOOCTET_FACTOR: f64 = 1000.0;
const OCTET_MEGAOCTET_FACTOR: f64 = 1000.0 * 1000.0;
const OCTET_GIGAOCTET_FACTOR: f64 = 1000.0 * 1000.0 * 1000.0;
const OCTET_TERAOCTET_FACTOR: f64 = 1000.0 * 1000.0 * 1000.0 * 1000.0;

// Constants, SI
const OCTET_KIBIOCTET_FACTOR: f64 = 1024.0;
const OCTET_MEBIOCTET_FACTOR: f64 = 1024.0 * 1024.0;
const OCTET_GIBIOCTET_FACTOR: f64 = 1024.0 * 1024.0 * 1024.0;
const OCTET_TEBIOCTET_FACTOR: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

/// The `Data` struct can be used to deal with computer information in a common way.
/// Common legacy and SI units are supported.
///
/// # Example
///
/// ```
/// use measurements::Data;
///
/// let file_size = Data::from_mebioctets(2.5);
/// let octets = file_size.as_octets();
/// println!("There are {} octets in that file.", octets);
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
pub struct Data {
    octets: f64,
}

impl Data {
    /// Create new Data from floating point value in Octets
    pub fn from_octets(octets: f64) -> Self {
        Data { octets }
    }

    /// Create new Data from floating point value in Bits
    pub fn from_bits(bits: f64) -> Self {
        Self::from_octets(bits * OCTET_BIT_FACTOR)
    }

    // Inputs, legacy
    /// Create new Data from floating point value in Kilooctets (1000 octets)
    pub fn from_kilooctets(kilooctets: f64) -> Self {
        Self::from_octets(kilooctets * OCTET_KILOOCTET_FACTOR)
    }

    /// Create new Data from floating point value in Megaoctets (1e6 octets)
    pub fn from_megaoctets(megaoctets: f64) -> Self {
        Self::from_octets(megaoctets * OCTET_MEGAOCTET_FACTOR)
    }

    /// Create new Data from floating point value in Gigaoctets (1e9 octets)
    pub fn from_gigaoctets(gigaoctets: f64) -> Self {
        Self::from_octets(gigaoctets * OCTET_GIGAOCTET_FACTOR)
    }

    /// Create new Data from floating point value in Teraoctets (1e12 octets)
    pub fn from_teraoctets(teraoctets: f64) -> Self {
        Self::from_octets(teraoctets * OCTET_TERAOCTET_FACTOR)
    }

    /// Create new Data from floating point value in Kibioctets (1024 octets)
    pub fn from_kibioctets(kibioctets: f64) -> Self {
        Self::from_octets(kibioctets * OCTET_KIBIOCTET_FACTOR)
    }

    /// Create new Data from floating point value in Mebioctets (1024**2 octets)
    pub fn from_mebioctets(mebioctets: f64) -> Self {
        Self::from_octets(mebioctets * OCTET_MEBIOCTET_FACTOR)
    }

    /// Create new Data from floating point value in Gibioctets (1024**3 octets)
    pub fn from_gibioctets(gibioctets: f64) -> Self {
        Self::from_octets(gibioctets * OCTET_GIBIOCTET_FACTOR)
    }

    /// Create new Data from floating point value in Tebioctets (1024**4 octets)
    pub fn from_tebioctets(tebioctets: f64) -> Self {
        Self::from_octets(tebioctets * OCTET_TEBIOCTET_FACTOR)
    }

    /// Convert this Data to a floating point value in Octets
    pub fn as_octets(&self) -> f64 {
        self.octets
    }

    /// Convert this Data to a floating point value in Bits
    pub fn as_bits(&self) -> f64 {
        self.octets / OCTET_BIT_FACTOR
    }

    /// Convert this Data to a floating point value in Kilooctets (1000 octets)
    pub fn as_kilooctets(&self) -> f64 {
        self.octets / OCTET_KILOOCTET_FACTOR
    }

    /// Convert this Data to a floating point value in Megaoctets (1e6 octets)
    pub fn as_megaoctets(&self) -> f64 {
        self.octets / OCTET_MEGAOCTET_FACTOR
    }

    /// Convert this Data to a floating point value in Gigaoctets (1e9 octets)
    pub fn as_gigaoctets(&self) -> f64 {
        self.octets / OCTET_GIGAOCTET_FACTOR
    }

    /// Convert this Data to a floating point value in Teraoctets (1e12 octets)
    pub fn as_teraoctets(&self) -> f64 {
        self.octets / OCTET_TERAOCTET_FACTOR
    }

    /// Convert this Data to a floating point value in Kibioctets (1024 octets)
    pub fn as_kibioctets(&self) -> f64 {
        self.octets / OCTET_KIBIOCTET_FACTOR
    }

    /// Convert this Data to a floating point value in Mebioctets (1024**2 octets)
    pub fn as_mebioctets(&self) -> f64 {
        self.octets / OCTET_MEBIOCTET_FACTOR
    }

    /// Convert this Data to a floating point value in Gibioctets (1024**3 octets)
    pub fn as_gibioctets(&self) -> f64 {
        self.octets / OCTET_GIBIOCTET_FACTOR
    }

    /// Convert this Data to a floating point value in Tebioctets (1024**4 octets)
    pub fn as_tebioctets(&self) -> f64 {
        self.octets / OCTET_TEBIOCTET_FACTOR
    }
}

impl Measurement for Data {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        // Smallest to largest
        let list = [
            ("octets", 1.0),
            ("KiB", 1024.0),
            ("MiB", 1024.0 * 1024.0),
            ("GiB", 1024.0 * 1024.0 * 1024.0),
            ("TiB", 1024.0 * 1024.0 * 1024.0 * 1024.0),
            ("PiB", 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0),
            ("EiB", 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0),
        ];
        self.pick_appropriate_units(&list)
    }

    fn get_base_units_name(&self) -> &'static str {
        "octets"
    }

    fn as_base_units(&self) -> f64 {
        self.octets
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_octets(units)
    }
}

implement_measurement! { Data }

#[cfg(feature = "from_str")]
impl_from_str! {
    Data,
    Data::from_octets,
    (Data::from_bits, "bit"),
    (Data::from_octets, "o"),
    (Data::from_kilooctets, "ko"),
    (Data::from_megaoctets, "Mo"),
    (Data::from_gigaoctets, "Go"),
    (Data::from_teraoctets, "To"),
    (Data::from_kibioctets, "kio", "Ko"),
    (Data::from_mebioctets, "Mio"),
    (Data::from_gibioctets, "Gio"),
    (Data::from_tebioctets, "Tio"),
}

#[cfg(test)]
mod test {
    use crate::{data::*, test_utils::assert_almost_eq};

    #[cfg(feature = "from_str")]
    use core::str::FromStr;

    // Metric
    #[test]
    fn bits() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_bits();

        let i2 = Data::from_bits(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 800.0);
        assert_almost_eq(r2, 12.5);
    }

    #[test]
    fn kilooctet() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_kilooctets();

        let i2 = Data::from_kilooctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 0.1);
        assert_almost_eq(r2, 1e5);
    }

    #[test]
    fn megaoctet() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_megaoctets();

        let i2 = Data::from_megaoctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 0.0001);
        assert_almost_eq(r2, 1e8);
    }

    #[test]
    fn gigaoctet() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_gigaoctets();

        let i2 = Data::from_gigaoctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 1e-7);
        assert_almost_eq(r2, 1e11);
    }

    #[test]
    fn teraoctet() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_teraoctets();

        let i2 = Data::from_teraoctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 1e-10);
        assert_almost_eq(r2, 1e14);
    }

    // Imperial
    #[test]
    fn kibioctet() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_kibioctets();

        let i2 = Data::from_kibioctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 0.09765625);
        assert_almost_eq(r2, 102400.0);
    }

    #[test]
    fn mebioctet() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_mebioctets();

        let i2 = Data::from_mebioctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 9.536743e-5);
        assert_almost_eq(r2, 104857600.0);
    }

    #[test]
    fn gibioctets() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_gibioctets();

        let i2 = Data::from_gibioctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 9.313226e-8);
        assert_almost_eq(r2, 107374182400.0);
    }

    #[test]
    fn tebioctets() {
        let i1 = Data::from_octets(100.0);
        let r1 = i1.as_tebioctets();

        let i2 = Data::from_tebioctets(100.0);
        let r2 = i2.as_octets();

        assert_almost_eq(r1, 9.094947e-11);
        assert_almost_eq(r2, 109951162777600.0);
    }

    // Traits
    #[test]
    fn add() {
        let a = Data::from_octets(2.0);
        let b = Data::from_octets(4.0);
        let c = a + b;
        assert_almost_eq(c.as_octets(), 6.0);
    }

    #[test]
    fn sub() {
        let a = Data::from_octets(2.0);
        let b = Data::from_octets(4.0);
        let c = a - b;
        assert_almost_eq(c.as_octets(), -2.0);
    }

    #[test]
    fn mul() {
        let b = Data::from_octets(4.0);
        let d = b * 2.0;
        assert_almost_eq(d.as_octets(), 8.0);
    }

    #[test]
    fn div() {
        let b = Data::from_octets(4.0);
        let d = b / 2.0;
        assert_almost_eq(d.as_octets(), 2.0);
    }

    #[test]
    fn eq() {
        let a = Data::from_octets(2.0);
        let b = Data::from_octets(2.0);
        assert_eq!(a == b, true);
    }

    #[test]
    fn neq() {
        let a = Data::from_octets(2.0);
        let b = Data::from_octets(4.0);
        assert_eq!(a == b, false);
    }

    #[test]
    fn cmp() {
        let a = Data::from_octets(2.0);
        let b = Data::from_octets(4.0);
        assert_eq!(a < b, true);
        assert_eq!(a <= b, true);
        assert_eq!(a > b, false);
        assert_eq!(a >= b, false);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn bits_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 bit").unwrap().as_bits());
        assert_almost_eq(123.0, Data::from_str("123bit").unwrap().as_bits());
        assert_almost_eq(123.0, Data::from_str(" 123 bit   ").unwrap().as_bits());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn octets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 o").unwrap().as_octets());
        assert_almost_eq(123.0, Data::from_str("123o").unwrap().as_octets());
        assert_almost_eq(123.0, Data::from_str(" 123 o  ").unwrap().as_octets());

        assert_almost_eq(123.0, Data::from_str(" 123  ").unwrap().as_octets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn kilooctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 ko").unwrap().as_kilooctets());
        assert_almost_eq(123.0, Data::from_str("123ko").unwrap().as_kilooctets());
        assert_almost_eq(123.0, Data::from_str(" 123 ko  ").unwrap().as_kilooctets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn megaoctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 Mo").unwrap().as_megaoctets());
        assert_almost_eq(123.0, Data::from_str("123Mo").unwrap().as_megaoctets());
        assert_almost_eq(123.0, Data::from_str(" 123 Mo  ").unwrap().as_megaoctets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn gigaoctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 Go").unwrap().as_gigaoctets());
        assert_almost_eq(123.0, Data::from_str("123Go").unwrap().as_gigaoctets());
        assert_almost_eq(123.0, Data::from_str(" 123 Go  ").unwrap().as_gigaoctets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn teraoctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 To").unwrap().as_teraoctets());
        assert_almost_eq(123.0, Data::from_str("123To").unwrap().as_teraoctets());
        assert_almost_eq(123.0, Data::from_str(" 123 To  ").unwrap().as_teraoctets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn kibioctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 kio").unwrap().as_kibioctets());
        assert_almost_eq(123.0, Data::from_str("123kio").unwrap().as_kibioctets());
        assert_almost_eq(123.0, Data::from_str(" 123 kio  ").unwrap().as_kibioctets());

        assert_almost_eq(123.0, Data::from_str("123 Ko").unwrap().as_kibioctets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn mebioctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 Mio").unwrap().as_mebioctets());
        assert_almost_eq(123.0, Data::from_str("123Mio").unwrap().as_mebioctets());
        assert_almost_eq(123.0, Data::from_str(" 123 Mio  ").unwrap().as_mebioctets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn gibioctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 Gio").unwrap().as_gibioctets());
        assert_almost_eq(123.0, Data::from_str("123Gio").unwrap().as_gibioctets());
        assert_almost_eq(123.0, Data::from_str(" 123 Gio  ").unwrap().as_gibioctets());
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn tebioctets_from_str() {
        assert_almost_eq(123.0, Data::from_str("123 Tio").unwrap().as_tebioctets());
        assert_almost_eq(123.0, Data::from_str("123Tio").unwrap().as_tebioctets());
        assert_almost_eq(123.0, Data::from_str(" 123 Tio  ").unwrap().as_tebioctets());
    }
}
