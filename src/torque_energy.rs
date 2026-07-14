//! Implements a bridging structure to distinguish between Torque and Energy

use super::*;

#[cfg(feature = "from_str")]
use crate::impl_from_str;

/// If you multiply a Force by a Length, we can't tell if you're
/// pushing something along (which requires Energy) or rotating
/// something (which creates a Torque). This struct is what results
/// from the multiplication, and you have to then convert
/// it to whichever you want.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Copy, Clone, Debug, Default)]
pub struct TorqueEnergy {
    newton_metres: f64,
}

impl core::convert::From<TorqueEnergy> for Torque {
    fn from(t: TorqueEnergy) -> Torque {
        Torque::from_newton_metres(t.newton_metres)
    }
}

impl core::convert::From<TorqueEnergy> for Energy {
    fn from(t: TorqueEnergy) -> Energy {
        Energy::from_joules(t.newton_metres)
    }
}

impl Measurement for TorqueEnergy {
    fn get_base_units_name(&self) -> &'static str {
        "Nm||J"
    }

    fn as_base_units(&self) -> f64 {
        self.newton_metres
    }

    fn from_base_units(units: f64) -> Self {
        TorqueEnergy {
            newton_metres: units,
        }
    }
}

#[cfg(feature = "from_str")]
impl_from_str! {
    TorqueEnergy,
    TorqueEnergy::from_base_units,
    (TorqueEnergy::from_base_units, "Nm", "J"),
}

#[cfg(test)]
mod test {
    #[cfg(feature = "from_str")]
    use {super::*, crate::test_utils::assert_almost_eq, core::str::FromStr};

    #[test]
    #[cfg(feature = "from_str")]
    fn torque_energy_from_str() {
        assert_almost_eq(
            123.4,
            TorqueEnergy::from_str("123.4 Nm").unwrap().as_base_units(),
        );
        assert_almost_eq(
            123.4,
            TorqueEnergy::from_str("123.4 J").unwrap().as_base_units(),
        );
    }
}
