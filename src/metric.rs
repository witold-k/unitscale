// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::fmt::Display;
use num_traits::Float;
use num_traits::cast::AsPrimitive;
use crate::*;

impl<T, S> Display for FloatMetric<T, S>
where
    T: Float + Display,
    S: MetricScale,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.val, S::ID)
    }
}

impl<T, S> FloatMetric<T, S>
where
    T: Float,
    S: MetricScale,
{
    #[inline(always)]
    pub fn new(val: T) -> Self {
        FloatMetric::<T, S> { val, scale: PhantomData }
    }

    #[inline(always)]
    pub fn raw_value(self) -> T {
        self.val
    }

    #[inline(always)]
    pub fn to<U: MetricScale>(self) -> FloatMetric<T, U>
    where
        T: 'static,
        f64: AsPrimitive<T>
    {
        FloatMetric::<T, U> {
            val: self.val * (S::ID.factor() / U::ID.factor()).as_(),
            scale: PhantomData,
        }
    }
}

impl<T: Float> FloatMetric<T, One> {
    #[inline(always)]
    pub fn new_as<U: MetricScale>(val: T) -> FloatMetric<T, U> {
        FloatMetric { val, scale: PhantomData }
    }
}
