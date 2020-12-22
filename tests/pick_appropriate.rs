extern crate measurements;

use measurements::{
    Measurement,
    mass::Mass,
    test_utils::assert_almost_eq,
};

#[test]
fn appropriate_units_eq_1() {
    let val : f64 = 1e0;
    let mass = Mass::from_kilograms(val);
    let (s, v) = mass.get_appropriate_units();
    assert_eq!(s, "kg");
    assert_almost_eq(val, v);
}

#[test]
fn appropriate_units_neg_1() {
    let val : f64 = -1e0;
    let mass = Mass::from_kilograms(val);
    let (s, v) = mass.get_appropriate_units();
    assert_eq!(s, "kg");
    assert_almost_eq(val, v);
}

#[test]
fn appropriate_units_lt_1() {
    // units will now be in grams
    let val : f64 = 1e0 - 1e-3;
    let mass = Mass::from_kilograms(val);
    let (s, v) = mass.get_appropriate_units();
    assert_eq!(s, "g");
    assert_almost_eq(val * 1e3, v);
}

#[test]
fn appropriate_units_gt_1() {
    let val : f64 = 1e0 + 1e-3;
    let mass = Mass::from_kilograms(val);
    let (s, v) = mass.get_appropriate_units();
    assert_eq!(s, "kg");
    assert_almost_eq(val, v);
}

#[test]
fn appropriate_units_from_grams() {
    let val : f64 = 1e3;
    let mass = Mass::from_grams(val);
    let (s, v) = mass.get_appropriate_units();
    assert_eq!(s, "kg");
    assert_almost_eq(val, v * 1e3);
}

#[test]
fn appropriate_units_to_grams() {
    let val : f64 = 1e-3;
    let mass = Mass::from_kilograms(val);
    let (s, v) = mass.get_appropriate_units();
    assert_eq!(s, "g");
    assert_almost_eq(val * 1e3, v);
}
