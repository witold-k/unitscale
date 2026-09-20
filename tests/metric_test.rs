// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use num_traits::identities::One;
use num_traits::identities::Zero;
use unitscale::*;

#[test]
fn test_basic_arithmetic() {
    let a = FloatMetric::<f64, Mega>::new(10.0);
    let b = FloatMetric::<f64, Mega>::new(2.0);

    assert_eq!((a + b).raw_value(), 12.0);
    assert_eq!((a - b).raw_value(), 8.0);
    assert_eq!((a * b).raw_value(), 20.0);
    assert_eq!((a / b).raw_value(), 5.0);
}

#[test]
fn test_mul_div_with_inner_type() {
    let a = FloatMetric::<f64, Mega>::new(10.0);

    assert_eq!((a * 3.0).raw_value(), 30.0);
    assert_eq!((a / 2.0).raw_value(), 5.0);
}

#[test]
fn test_assign_ops() {
    let mut x = FloatMetric::<f64, Mega>::new(10.0);
    let y = FloatMetric::<f64, Mega>::new(2.0);

    x += y;
    assert_eq!(x.raw_value(), 12.0);

    x -= y;
    assert_eq!(x.raw_value(), 10.0);

    x *= 3.0;
    assert_eq!(x.raw_value(), 30.0);

    x /= 2.0;
    assert_eq!(x.raw_value(), 15.0);
}

#[test]
fn test_neg() {
    let a = FloatMetric::<f64, Mega>::new(5.0);
    assert_eq!((-a).raw_value(), -5.0);
}

#[test]
fn test_comparisons() {
    let a = FloatMetric::<f64, Mega>::new(3.0);
    let b = FloatMetric::<f64, Mega>::new(7.0);

    assert!(a < b);
    assert!(b > a);
    assert!(a != b);
}

#[test]
fn test_sum_and_product() {
    let values = [
        FloatMetric::<f64, Mega>::new(2.0),
        FloatMetric::<f64, Mega>::new(3.0),
        FloatMetric::<f64, Mega>::new(5.0),
    ];

    let sum: FloatMetric<f64, Mega> = values.iter().copied().sum();
    assert_eq!(sum.raw_value(), 10.0);

    let product: FloatMetric<f64, Mega> = values.iter().copied().product();
    assert_eq!(product.raw_value(), 30.0);
}

#[test]
fn test_zero_one() {
    let z = FloatMetric::<f64, Mega>::zero();
    let o = FloatMetric::<f64, Mega>::one();

    assert_eq!(z.raw_value(), 0.0);
    assert_eq!(o.raw_value(), 1.0);
}

#[test]
fn test_same_type() {
    let m = FloatMetric::new_as::<Mega>(10f64);
    let k = m.to::<Kilo>();

    assert_eq!(m, m);
    assert_eq!(k.raw_value(), 10000f64);
}

