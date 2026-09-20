// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::cast::AsPrimitive;
use num_traits::Float;

pub trait AngleScale: Copy + Clone + Default + 'static {
    const TO_RAD: f64;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rad;

#[derive(Copy, Clone, Debug, Default)]
pub struct Deg;

impl AngleScale for Rad {
    const TO_RAD: f64 = 1.0;
}

impl AngleScale for Deg {
    const TO_RAD: f64 = std::f64::consts::PI / 180.0;
}

/// A zero-overhead angle wrapper with an explicit representation.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Angle<T: Float, S: AngleScale> {
    val: T,
    marker: PhantomData<S>,
}

impl<T, S> Angle<T, S>
where
    T: Float,
    S: AngleScale,
{
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            val: value,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    pub fn raw_value(self) -> T {
        self.val
    }

    #[inline(always)]
    pub fn to<U>(self) -> Angle<T, U>
    where
        U: AngleScale,
        T: 'static,
        f64: AsPrimitive<T>,
    {
        Angle::new(self.val * (S::TO_RAD / U::TO_RAD).as_())
    }
}

impl<T, S> Add for Angle<T, S>
where
    T: Float,
    S: AngleScale,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.val + rhs.val)
    }
}

impl<T, S> Sub for Angle<T, S>
where
    T: Float,
    S: AngleScale,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.val - rhs.val)
    }
}

impl<T, S> Neg for Angle<T, S>
where
    T: Float,
    S: AngleScale,
{
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::new(-self.val)
    }
}

impl<T, S> Mul<T> for Angle<T, S>
where
    T: Float,
    S: AngleScale,
{
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.val * rhs)
    }
}

impl<T, S> Div<T> for Angle<T, S>
where
    T: Float,
    S: AngleScale,
{
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.val / rhs)
    }
}
