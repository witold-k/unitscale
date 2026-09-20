// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::*;

impl MetricScaleID {
    #[inline(always)]
    pub fn factor(&self) -> f64 {
        match self {
            MetricScaleID::Quetta => 1e30,
            MetricScaleID::Ronna  => 1e27,
            MetricScaleID::Yotta  => 1e24,
            MetricScaleID::Zetta  => 1e21,
            MetricScaleID::Exa    => 1e18,
            MetricScaleID::Peta   => 1e15,
            MetricScaleID::Tera   => 1e12,
            MetricScaleID::Giga   => 1e9,
            MetricScaleID::Mega   => 1e6,
            MetricScaleID::Kilo   => 1e3,
            MetricScaleID::Hecto  => 1e2,
            MetricScaleID::Deca   => 1e1,
            MetricScaleID::One    => 1.0,
            MetricScaleID::Deci   => 1e-1,
            MetricScaleID::Centi  => 1e-2,
            MetricScaleID::Milli  => 1e-3,
            MetricScaleID::Micro  => 1e-6,
            MetricScaleID::Nano   => 1e-9,
            MetricScaleID::Pico   => 1e-12,
            MetricScaleID::Femto  => 1e-15,
            MetricScaleID::Atto   => 1e-18,
            MetricScaleID::Zepto  => 1e-21,
            MetricScaleID::Yokto  => 1e-24,
            MetricScaleID::Ronto  => 1e-27,
            MetricScaleID::Quekto => 1e-30,
        }
    }
}

impl MetricScale for Mega  {
    const ID: MetricScaleID = MetricScaleID::Mega;
    const FACTOR: f64 = 1e6;
}

impl MetricScale for Kilo  {
    const ID: MetricScaleID = MetricScaleID::Kilo;
    const FACTOR: f64 = 1e3;
}

impl MetricScale for Hecto {
    const ID: MetricScaleID = MetricScaleID::Hecto;
    const FACTOR: f64 = 1e2;
}

impl MetricScale for Deca  {
    const ID: MetricScaleID = MetricScaleID::Deca;
    const FACTOR: f64 = 1e1;
}

impl MetricScale for One   {
    const ID: MetricScaleID = MetricScaleID::One;
    const FACTOR: f64 = 1e0;
}

impl MetricScale for Milli {
    const ID: MetricScaleID = MetricScaleID::Milli;
    const FACTOR: f64 = 1e-3;
}

