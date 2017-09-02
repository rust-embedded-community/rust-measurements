use super::measurement::*;

// Constants
const OCTET_BIT_FACTOR: f64 = 0.125;

// Constants, legacy
const OCTET_KILOOCTET_FACTOR: f64 = 1000.;
const OCTET_MEGAOCTET_FACTOR: f64 = 1000000.;
const OCTET_GIGAOCTET_FACTOR: f64 = 1000000000.;
const OCTET_TERAOCTET_FACTOR: f64 = 1000000000000.;

// Constants, SI
const OCTET_KIBIOCTET_FACTOR: f64 = 1024.;
const OCTET_MEBIOCTET_FACTOR: f64 = 1048576.;
const OCTET_GIBIOCTET_FACTOR: f64 = 1073741824.;
const OCTET_TEBIOCTET_FACTOR: f64 = 1099511627776.;

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
#[derive(Copy, Clone, Debug)]
pub struct Data {
    octets: f64,
}

impl Data {
    // Inputs
    pub fn from_octets(octets: f64) -> Self {
        Data { octets: octets }
    }

    pub fn from_bits(bits: f64) -> Self {
        Self::from_octets(bits * OCTET_BIT_FACTOR)
    }

    // Inputs, legacy
    pub fn from_kilooctets(kilooctets: f64) -> Self {
        Self::from_octets(kilooctets * OCTET_KILOOCTET_FACTOR)
    }

    pub fn from_megaoctets(megaoctets: f64) -> Self {
        Self::from_octets(megaoctets * OCTET_MEGAOCTET_FACTOR)
    }

    pub fn from_gigaoctets(gigaoctets: f64) -> Self {
        Self::from_octets(gigaoctets * OCTET_GIGAOCTET_FACTOR)
    }

    pub fn from_teraoctets(teraoctets: f64) -> Self {
        Self::from_octets(teraoctets * OCTET_TERAOCTET_FACTOR)
    }

    // Inputs, SI
    pub fn from_kibioctets(kibioctets: f64) -> Self {
        Self::from_octets(kibioctets * OCTET_KIBIOCTET_FACTOR)
    }

    pub fn from_mebioctets(mebioctets: f64) -> Self {
        Self::from_octets(mebioctets * OCTET_MEBIOCTET_FACTOR)
    }

    pub fn from_gibioctets(gibioctets: f64) -> Self {
        Self::from_octets(gibioctets * OCTET_GIBIOCTET_FACTOR)
    }

    pub fn from_tebioctets(tebioctets: f64) -> Self {
        Self::from_octets(tebioctets * OCTET_TEBIOCTET_FACTOR)
    }

    // Outputs
    pub fn as_octets(&self) -> f64 {
        self.octets
    }

    pub fn as_bits(&self) -> f64 {
        self.octets / OCTET_BIT_FACTOR
    }

    // Outputs, legacy
    pub fn as_kilooctets(&self) -> f64 {
        self.octets / OCTET_KILOOCTET_FACTOR
    }

    pub fn as_megaoctets(&self) -> f64 {
        self.octets / OCTET_MEGAOCTET_FACTOR
    }

    pub fn as_gigaoctets(&self) -> f64 {
        self.octets / OCTET_GIGAOCTET_FACTOR
    }

    pub fn as_teraoctets(&self) -> f64 {
        self.octets / OCTET_TERAOCTET_FACTOR
    }

    // Outputs, SI
    pub fn as_kibioctets(&self) -> f64 {
        self.octets / OCTET_KIBIOCTET_FACTOR
    }

    pub fn as_mebioctets(&self) -> f64 {
        self.octets / OCTET_MEBIOCTET_FACTOR
    }

    pub fn as_gibioctets(&self) -> f64 {
        self.octets / OCTET_GIBIOCTET_FACTOR
    }

    pub fn as_tebioctets(&self) -> f64 {
        self.octets / OCTET_TEBIOCTET_FACTOR
    }
}

impl Measurement for Data {
    fn get_base_units(&self) -> f64 {
        self.octets
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_octets(units)
    }
}

implement_measurement! { Data }

impl ::std::fmt::Display for Data {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:.1} o", self.as_octets())
    }
}
