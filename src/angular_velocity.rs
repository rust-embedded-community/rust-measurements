//! Types and constants for handling speed of rotation (angular velocity)

use super::measurement::*;
use crate::PI;

#[cfg(feature = "from_str")]
use crate::impl_from_str;

/// The 'AngularVelocity' struct can be used to deal with angular velocities in a common way.
///
/// # Example
///
/// ```
/// use measurements::AngularVelocity;
///
/// const cylinders: f64 = 6.0;
/// let engine_speed = AngularVelocity::from_rpm(9000.0);
/// let sparks_per_second = (engine_speed.as_hertz() / 2.0) * cylinders;
/// ```
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
pub struct AngularVelocity {
    radians_per_second: f64,
}

impl AngularVelocity {
    /// Create a new AngularVelocity from a floating point value in radians per second
    pub fn from_radians_per_second(radians_per_second: f64) -> Self {
        AngularVelocity { radians_per_second }
    }

    /// Create a new AngularVelocity from a floating point value in revolutions per minute (RPM)
    pub fn from_rpm(rpm: f64) -> Self {
        let revs_per_second = rpm / 60.0;
        AngularVelocity::from_radians_per_second(revs_per_second * PI * 2.0)
    }

    /// Create a new AngularVelocity from a floating point value in revolutions per second (Hz)
    pub fn from_hertz(hz: f64) -> Self {
        AngularVelocity::from_radians_per_second(hz * PI * 2.0)
    }

    /// Convert this AngularVelocity to a floating point value in radians per second
    pub fn as_radians_per_second(&self) -> f64 {
        self.radians_per_second
    }

    /// Convert this AngularVelocity to a floating point value in degrees
    pub fn as_rpm(&self) -> f64 {
        (self.radians_per_second * 60.0) / (2.0 * PI)
    }

    /// Convert this AngularVelocity to a floating point value in revolutions per second (Hz)
    pub fn as_hertz(&self) -> f64 {
        self.radians_per_second / (2.0 * PI)
    }
}

impl Measurement for AngularVelocity {
    fn get_base_units_name(&self) -> &'static str {
        "rad/s"
    }

    fn as_base_units(&self) -> f64 {
        self.radians_per_second
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_radians_per_second(units)
    }
}

#[cfg(feature = "from_str")]
impl_from_str! {
    AngularVelocity,
    AngularVelocity::from_radians_per_second,
    (AngularVelocity::from_radians_per_second, "rad/s"),
    (AngularVelocity::from_rpm, "rpm"),
    (AngularVelocity::from_hertz, "Hz"),
}

implement_measurement! { AngularVelocity }

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::assert_almost_eq;

    #[cfg(feature = "from_str")]
    use core::str::FromStr;

    #[test]
    fn rpm() {
        let i1 = AngularVelocity::from_rpm(6000.0);
        let r1 = i1.as_radians_per_second();
        let i2 = AngularVelocity::from_radians_per_second(100.0);
        let r2 = i2.as_rpm();
        assert_almost_eq(r1, 628.31853);
        assert_almost_eq(r2, 954.929659642538);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn empty_str() {
        let t = AngularVelocity::from_str("");
        assert!(t.is_ok());

        let o = t.unwrap().as_radians_per_second();
        assert_eq!(o, 0.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn rad_per_second_string() {
        let t = AngularVelocity::from_str("100 rad/s");
        assert!(t.is_ok());

        let o = t.unwrap().as_radians_per_second();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn rpm_string() {
        let t = AngularVelocity::from_str("100rpm");
        assert!(t.is_ok());

        let o = t.unwrap().as_rpm();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn hertz_string() {
        let t = AngularVelocity::from_str("100 Hz");
        assert!(t.is_ok());

        let o = t.unwrap().as_hertz();
        assert_almost_eq(o, 100.0);
    }

    #[test]
    #[cfg(feature = "from_str")]
    fn invalid_str() {
        let t = AngularVelocity::from_str("abcd");
        assert!(t.is_err());
    }
}
