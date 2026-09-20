# unitscale

`unitscale` is a small Rust library for zero-overhead numeric wrappers with compile-time scale information.

It models SI decimal scale prefixes and angles without becoming a physical dimensional-analysis framework.

## Metric scaling

```rust
use unitscale::{FloatMetric, Kilo, Mega};

let mega = FloatMetric::<f64, Mega>::new(2.5);
let kilo = mega.to::<Kilo>();

assert_eq!(kilo.raw_value(), 2_500.0);
```

The prefix is part of the type and occupies no runtime storage. Conversion between differently scaled representations is explicit.

All current SI decimal prefixes from `Quetta` (`10^30`) through `Quekto` (`10^-30`) are available, together with `One`.

## Scale algebra

Multiplication and division combine the decimal exponents of their scales at compile time:

```rust
use unitscale::{FloatMetric, Giga, Kilo, Mega, One};

let mega = FloatMetric::<f64, Mega>::new(2.0);
let kilo = FloatMetric::<f64, Kilo>::new(3.0);

let giga: FloatMetric<f64, Giga> = mega * kilo;
assert_eq!(giga.raw_value(), 6.0);

let one: FloatMetric<f64, One> =
    FloatMetric::<f64, Kilo>::new(2.0)
    * FloatMetric::<f64, unitscale::Milli>::new(3.0);
assert_eq!(one.raw_value(), 6.0);
```

Thus `Mega * Kilo -> Giga`, `Kilo * Kilo -> Mega`, and `Giga / Mega -> Kilo`.

Only results that correspond to a defined SI decimal prefix are implemented. For example, `Hecto * Kilo` would produce `10^5`, for which SI defines no prefix, so that expression fails at compile time instead of inventing a runtime scale.

Addition and subtraction require identical scale types. Values with different scales can be converted explicitly before adding or subtracting them.

## Angles

Angles are a separate zero-overhead wrapper with `Rad` and `Deg` representations.

```rust
use unitscale::{Angle, Deg, Rad};

let degrees = Angle::<f64, Deg>::new(180.0);
let radians = degrees.to::<Rad>();

assert!((radians.raw_value() - std::f64::consts::PI).abs() < 1e-12);
```

Addition, subtraction, negation, and scalar multiplication/division preserve the angle representation. Different representations require explicit conversion.

## Zero-cost representation

`FloatMetric<T, S>` and `Angle<T, S>` use zero-sized marker types and `PhantomData`. Both wrappers are `#[repr(transparent)]`; their storage size is exactly the size of `T`.

Scale algebra is resolved through associated types at compile time. There is no runtime scale field, lookup table, dynamic dispatch, or allocation. Conversions perform only the numeric multiplication required to change representation.

## Design goals

- Model numeric scaling, not physical dimensions.
- Keep scale algebra mathematically consistent.
- Resolve valid scale multiplication and division at compile time.
- Reject scale combinations that cannot be represented by an SI prefix.
- Require explicit conversion for addition/subtraction across scales.
- Keep scale and angle metadata out of runtime storage.
- Stay on stable Rust.
- Keep the public API small enough to stabilize after the draft phase.
- Consistent test layout: tests live under `tests/`, mirror the relative `src/` hierarchy where relevant, and use the source filename with a `_test.rs` suffix.

## Build and test

```bash
just
```

CI additionally checks formatting, all targets, tests, and Clippy with warnings denied.

## Status

The crate is still in its draft/redesign phase. The current API establishes the intended abstraction boundary before downstream code starts depending on it.
