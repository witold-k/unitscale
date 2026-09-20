// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

pub mod metric;
pub mod metricscale;
use struct_extractors::extract_number;


use std::marker::PhantomData;
use num_traits::Float;
//use num_traits::PrimInt;

#[derive(Debug, Copy, Clone)]
pub enum AngleTypeID {
    Rad  = 0,
    Deg  = 1,
    Grad = 2,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rad;

#[derive(Copy, Clone, Debug, Default)]
pub struct Deg;

#[derive(Copy, Clone, Debug, Default)]
pub struct Grad;

pub trait AngleType { const ID: AngleTypeID; }

pub struct Angle<T: Float, U> {
    pub val: T,
    scale: PhantomData::<U>
}

/**
 * https://de.wikipedia.org/wiki/Vors%C3%A4tze_f%C3%BCr_Ma%C3%9Feinheiten
 * This represents a scale factor and is base for the struct Metric<T: Float, MetricScale>
 */
#[derive(Debug, Copy, Clone)]
pub enum MetricScaleID {
    Quetta,
    Ronna,
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deca,
    One,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yokto,
    Ronto,
    Quekto
}

pub trait MetricScale {
    const ID: MetricScaleID;
    const FACTOR: f64;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Mega;

#[derive(Copy, Clone, Debug, Default)]
pub struct Hecto;

#[derive(Copy, Clone, Debug, Default)]
pub struct Deca;

#[derive(Copy, Clone, Debug, Default)]
pub struct Kilo;

#[derive(Copy, Clone, Debug, Default)]
pub struct One;

#[derive(Copy, Clone, Debug, Default)]
pub struct Milli;

//pub struct MetricScale<const S: MetricScaleID>;
/**
 * A Metric Float, shares the same rust traits like a float
 */
#[derive(Clone, Copy, Debug, Default)]
#[extract_number(val)]
pub struct FloatMetric<T: Float, S: MetricScale>
{
    val: T,
    scale: PhantomData::<S>
}

