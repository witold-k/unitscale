// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::cast::AsPrimitive;
use num_traits::Float;

use crate::metricscale::{MetricScale, One, ScaleDiv, ScaleMul};

/// A floating-point value tagged with an SI decimal scale.
///
/// The scale is type-level metadata and occupies no runtime storage.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct FloatMetric<T: Float, S: MetricScale> {
    val: T,
    scale: PhantomData<S>,
}

impl<T, S> FloatMetric<T, S>
where
    T: Float,
    S: MetricScale,
{
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            val: value,
            scale: PhantomData,
        }
    }

    #[inline(always)]
    pub fn raw_value(self) -> T {
        self.val
    }

    #[inline(always)]
    pub fn to<U>(self) -> FloatMetric<T, U>
    where
        U: MetricScale,
        T: 'static,
        f64: AsPrimitive<T>,
    {
        FloatMetric::new(self.val * (S::FACTOR / U::FACTOR).as_())
    }
}

impl<T: Float> FloatMetric<T, One> {
    #[inline(always)]
    pub const fn new_as<S: MetricScale>(value: T) -> FloatMetric<T, S> {
        FloatMetric::new(value)
    }
}

impl<T, S> Neg for FloatMetric<T, S>
where
    T: Float,
    S: MetricScale,
{
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.val)
    }
}

impl<T, S> Add for FloatMetric<T, S>
where
    T: Float,
    S: MetricScale,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val + rhs.val)
    }
}

impl<T, S> Sub for FloatMetric<T, S>
where
    T: Float,
    S: MetricScale,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.val - rhs.val)
    }
}

impl<T, L, R> Mul<FloatMetric<T, R>> for FloatMetric<T, L>
where
    T: Float,
    L: MetricScale + ScaleMul<R>,
    R: MetricScale,
{
    type Output = FloatMetric<T, <L as ScaleMul<R>>::Output>;

    #[inline(always)]
    fn mul(self, rhs: FloatMetric<T, R>) -> Self::Output {
        FloatMetric::new(self.val * rhs.val)
    }
}

impl<T, L, R> Div<FloatMetric<T, R>> for FloatMetric<T, L>
where
    T: Float,
    L: MetricScale + ScaleDiv<R>,
    R: MetricScale,
{
    type Output = FloatMetric<T, <L as ScaleDiv<R>>::Output>;

    #[inline(always)]
    fn div(self, rhs: FloatMetric<T, R>) -> Self::Output {
        FloatMetric::new(self.val / rhs.val)
    }
}

impl<T, S> Mul<T> for FloatMetric<T, S>
where
    T: Float,
    S: MetricScale,
{
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.val * rhs)
    }
}

impl<T, S> Div<T> for FloatMetric<T, S>
where
    T: Float,
    S: MetricScale,
{
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.val / rhs)
    }
}

impl<T, S> Display for FloatMetric<T, S>
where
    T: Float + Display,
    S: MetricScale,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if S::SYMBOL.is_empty() {
            write!(f, "{}", self.val)
        } else {
            write!(f, "{} {}", self.val, S::SYMBOL)
        }
    }
}
