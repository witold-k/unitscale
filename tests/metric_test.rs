// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::mem::size_of;
use unitscale::*;

fn assert_relative_eq(actual: f64, expected: f64, tolerance: f64) {
    let scale = actual.abs().max(expected.abs());
    assert!(
        (actual - expected).abs() <= tolerance * scale,
        "actual={actual:e}, expected={expected:e}"
    );
}

#[test]
fn same_scale_addition_and_subtraction_preserve_scale() {
    let a = FloatMetric::<f64, Mega>::new(10.0);
    let b = FloatMetric::<f64, Mega>::new(2.0);

    let sum: FloatMetric<f64, Mega> = a + b;
    let difference: FloatMetric<f64, Mega> = a - b;

    assert_eq!(sum.raw_value(), 12.0);
    assert_eq!(difference.raw_value(), 8.0);
}

#[test]
fn multiplication_combines_metric_scales() {
    let mega = FloatMetric::<f64, Mega>::new(2.0);
    let kilo = FloatMetric::<f64, Kilo>::new(3.0);

    let giga: FloatMetric<f64, Giga> = mega * kilo;
    assert_eq!(giga.raw_value(), 6.0);

    let mega_squared_scale: FloatMetric<f64, Tera> =
        FloatMetric::<f64, Mega>::new(2.0) * FloatMetric::<f64, Mega>::new(4.0);
    assert_eq!(mega_squared_scale.raw_value(), 8.0);
}

#[test]
fn division_combines_metric_scales() {
    let giga = FloatMetric::<f64, Giga>::new(12.0);
    let mega = FloatMetric::<f64, Mega>::new(3.0);

    let kilo: FloatMetric<f64, Kilo> = giga / mega;
    assert_eq!(kilo.raw_value(), 4.0);

    let unscaled: FloatMetric<f64, One> =
        FloatMetric::<f64, Mega>::new(8.0) / FloatMetric::<f64, Mega>::new(4.0);
    assert_eq!(unscaled.raw_value(), 2.0);
}

#[test]
fn inverse_scales_work() {
    let reciprocal_kilo: FloatMetric<f64, Milli> =
        FloatMetric::<f64, One>::new(1.0) / FloatMetric::<f64, Kilo>::new(2.0);
    assert_eq!(reciprocal_kilo.raw_value(), 0.5);

    let one: FloatMetric<f64, One> =
        FloatMetric::<f64, Kilo>::new(2.0) * FloatMetric::<f64, Milli>::new(3.0);
    assert_eq!(one.raw_value(), 6.0);
}

#[test]
fn scalar_arithmetic_preserves_scale() {
    let value = FloatMetric::<f64, Mega>::new(10.0);

    assert_eq!((value * 3.0).raw_value(), 30.0);
    assert_eq!((value / 2.0).raw_value(), 5.0);
}

#[test]
fn conversion_changes_representation_only() {
    let mega = FloatMetric::<f64, Mega>::new(1.0);

    assert_eq!(mega.to::<Kilo>().raw_value(), 1_000.0);
    assert_eq!(mega.to::<One>().raw_value(), 1_000_000.0);
}

#[test]
fn full_si_prefix_range_is_available() {
    let base = FloatMetric::<f64, One>::new(1.0);

    assert_relative_eq(base.to::<Quetta>().raw_value(), 1e-30, 1e-14);
    assert_relative_eq(base.to::<Quekto>().raw_value(), 1e30, 1e-14);
}

#[test]
fn angle_converts_between_degrees_and_radians() {
    let degrees = Angle::<f64, Deg>::new(180.0);
    let radians = degrees.to::<Rad>();

    assert!((radians.raw_value() - std::f64::consts::PI).abs() < 1e-12);
    assert!((radians.to::<Deg>().raw_value() - 180.0).abs() < 1e-12);
}

#[test]
fn angle_arithmetic_preserves_representation() {
    let a = Angle::<f64, Deg>::new(90.0);
    let b = Angle::<f64, Deg>::new(30.0);

    assert_eq!((a + b).raw_value(), 120.0);
    assert_eq!((a - b).raw_value(), 60.0);
    assert_eq!((-b).raw_value(), -30.0);
    assert_eq!((b * 2.0).raw_value(), 60.0);
    assert_eq!((b / 2.0).raw_value(), 15.0);
}

#[test]
fn wrappers_have_no_storage_overhead() {
    assert_eq!(size_of::<FloatMetric<f64, Mega>>(), size_of::<f64>());
    assert_eq!(size_of::<FloatMetric<f32, Milli>>(), size_of::<f32>());
    assert_eq!(size_of::<Angle<f64, Deg>>(), size_of::<f64>());
}

#[test]
fn scale_algebra_covers_nontrivial_prefix_pairs() {
    let one_from_large_scales: FloatMetric<f64, One> =
        FloatMetric::<f64, Nano>::new(2.0) * FloatMetric::<f64, Giga>::new(3.0);
    assert_eq!(one_from_large_scales.raw_value(), 6.0);

    let centi: FloatMetric<f64, Centi> =
        FloatMetric::<f64, Deci>::new(2.0) * FloatMetric::<f64, Deci>::new(3.0);
    assert_eq!(centi.raw_value(), 6.0);

    let tera: FloatMetric<f64, Tera> =
        FloatMetric::<f64, Peta>::new(8.0) / FloatMetric::<f64, Kilo>::new(2.0);
    assert_eq!(tera.raw_value(), 4.0);

    let quetta: FloatMetric<f64, Quetta> =
        FloatMetric::<f64, One>::new(8.0) / FloatMetric::<f64, Quekto>::new(2.0);
    assert_eq!(quetta.raw_value(), 4.0);
}

#[test]
fn multiplication_is_invariant_under_scale_conversion() {
    let mega = FloatMetric::<f64, Mega>::new(2.0);
    let kilo = FloatMetric::<f64, Kilo>::new(3.0);

    let direct: FloatMetric<f64, Giga> = mega * kilo;
    let converted: FloatMetric<f64, Mega> = mega.to::<Kilo>() * kilo;

    assert_eq!(direct.to::<One>().raw_value(), converted.to::<One>().raw_value());
}
