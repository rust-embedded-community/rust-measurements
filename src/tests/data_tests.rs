use data::*;
use super::assert_almost_eq;

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
    let a = Data::from_octets(2.0);
    let b = Data::from_octets(4.0);
    let c = a * b;
    let d = b * 2.0;
    assert_almost_eq(c.as_octets(), 8.0);
    assert_almost_eq(d.as_octets(), 8.0);
}

#[test]
fn div() {
    let a = Data::from_octets(2.0);
    let b = Data::from_octets(4.0);
    let c = a * b;
    let d = a * 2.0;
    assert_almost_eq(c.as_octets(), 8.0);
    assert_almost_eq(d.as_octets(), 4.0);
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
