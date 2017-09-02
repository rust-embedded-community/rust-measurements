mod length_tests;
mod temperature_tests;
mod weight_tests;
mod volume_tests;
mod data_tests;

const DEFAULT_DELTA: f64 = 0.00001;

fn almost_eq(a: f64, b: f64) -> bool {
    almost_eq_delta(a, b, DEFAULT_DELTA)
}

fn almost_eq_delta(a: f64, b: f64, d: f64) -> bool {
    (a - b).abs() < d
}

fn assert_almost_eq(a: f64, b: f64) {
    assert_almost_eq_delta(a, b, DEFAULT_DELTA);
}

fn assert_almost_eq_delta(a: f64, b: f64, d: f64) {
    if !almost_eq_delta(a, b, d) {
        panic!("assertion failed: {:?} != {:?} (within {:?})", a, b, d);
    }
}
