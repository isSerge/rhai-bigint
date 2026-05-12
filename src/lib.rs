#![doc = include_str!("../README.md")]

//! Arbitrary-precision BigInt support for Rhai scripts.
//!
//! Provides [`BigIntPackage`] (via `def_package!`) for registering BigInt
//! support into a Rhai [`Engine`](rhai::Engine).

use num_bigint::BigInt;
use rhai::{def_package, plugin::*};

#[export_module]
mod bigint_functions {
    use num_bigint::{BigInt, Sign};
    use num_traits::{FromPrimitive, ToPrimitive, Zero};

    /// Creates a `BigInt` from an integer.
    pub fn bigint(value: i64) -> BigInt {
        value.into()
    }

    /// Creates a `BigInt` from a float by truncating toward zero.
    #[rhai_fn(name = "bigint", return_raw)]
    pub fn bigint_from_float(value: rhai::FLOAT) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        BigInt::from_f64(value)
            .ok_or_else(|| format!("Cannot convert {value} to BigInt: value must be finite").into())
    }

    /// Creates a `BigInt` from a string.
    #[rhai_fn(name = "bigint", return_raw)]
    pub fn bigint_from_str(value: String) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        value
            .parse::<BigInt>()
            .map_err(|e| format!("Failed to create BigInt from string: {e}").into())
    }

    #[rhai_fn(name = "+", pure)]
    pub fn add(l: &mut BigInt, r: BigInt) -> BigInt {
        l.clone() + r
    }

    #[rhai_fn(name = "-", pure)]
    pub fn sub(l: &mut BigInt, r: BigInt) -> BigInt {
        l.clone() - r
    }

    #[rhai_fn(name = "*", pure)]
    pub fn mul(l: &mut BigInt, r: BigInt) -> BigInt {
        l.clone() * r
    }

    #[rhai_fn(name = "/", pure, return_raw)]
    pub fn div(l: &mut BigInt, r: BigInt) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        if r.is_zero() {
            return Err("Division by zero".into());
        }
        Ok(l.clone() / r)
    }

    #[rhai_fn(name = "%", pure, return_raw)]
    pub fn rem(l: &mut BigInt, r: BigInt) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        if r.is_zero() {
            return Err("Modulo by zero".into());
        }
        Ok(l.clone() % r)
    }

    #[rhai_fn(name = "-", pure)]
    pub fn neg(value: &mut BigInt) -> BigInt {
        -value.clone()
    }

    /// Raises a `BigInt` to an integer power. The exponent must be non-negative
    /// and fit in a `u32`; returns an error otherwise.
    #[rhai_fn(name = "**", pure, return_raw)]
    pub fn pow(base: &mut BigInt, exp: i64) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        let exp_u32 = u32::try_from(exp).map_err(|_| -> Box<rhai::EvalAltResult> {
            format!("Exponent must be a non-negative integer that fits in u32, got {exp}").into()
        })?;
        Ok(base.clone().pow(exp_u32))
    }

    #[rhai_fn(name = "&", pure)]
    pub fn bitand(l: &mut BigInt, r: BigInt) -> BigInt {
        l.clone() & r
    }

    #[rhai_fn(name = "|", pure)]
    pub fn bitor(l: &mut BigInt, r: BigInt) -> BigInt {
        l.clone() | r
    }

    #[rhai_fn(name = "^", pure)]
    pub fn bitxor(l: &mut BigInt, r: BigInt) -> BigInt {
        l.clone() ^ r
    }

    /// Left-shifts a `BigInt` by `shift` bits. `shift` must be non-negative.
    #[rhai_fn(name = "<<", pure, return_raw)]
    pub fn shl(value: &mut BigInt, shift: i64) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        let shift_u64 = u64::try_from(shift).map_err(|_| -> Box<rhai::EvalAltResult> {
            format!("Shift amount must be non-negative, got {shift}").into()
        })?;
        Ok(value.clone() << shift_u64)
    }

    /// Right-shifts a `BigInt` by `shift` bits. `shift` must be non-negative.
    #[rhai_fn(name = ">>", pure, return_raw)]
    pub fn shr(value: &mut BigInt, shift: i64) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        let shift_u64 = u64::try_from(shift).map_err(|_| -> Box<rhai::EvalAltResult> {
            format!("Shift amount must be non-negative, got {shift}").into()
        })?;
        Ok(value.clone() >> shift_u64)
    }

    #[rhai_fn(name = "==", pure)]
    pub fn eq(l: &mut BigInt, r: BigInt) -> bool {
        *l == r
    }

    #[rhai_fn(name = "!=", pure)]
    pub fn ne(l: &mut BigInt, r: BigInt) -> bool {
        *l != r
    }

    #[rhai_fn(name = "<", pure)]
    pub fn lt(l: &mut BigInt, r: BigInt) -> bool {
        *l < r
    }

    #[rhai_fn(name = "<=", pure)]
    pub fn le(l: &mut BigInt, r: BigInt) -> bool {
        *l <= r
    }

    #[rhai_fn(name = ">", pure)]
    pub fn gt(l: &mut BigInt, r: BigInt) -> bool {
        *l > r
    }

    #[rhai_fn(name = ">=", pure)]
    pub fn ge(l: &mut BigInt, r: BigInt) -> bool {
        *l >= r
    }

    /// Converts a `BigInt` to its decimal string representation.
    #[rhai_fn(name = "to_string", pure)]
    pub fn to_string(value: &mut BigInt) -> String {
        value.to_string()
    }

    /// Converts a `BigInt` to a `0x`-prefixed lowercase hex string.
    /// Negative values are prefixed with `-0x`.
    #[rhai_fn(name = "to_hex", pure)]
    pub fn to_hex(value: &mut BigInt) -> String {
        let hex = format!("{:x}", value.magnitude());
        if value.sign() == Sign::Minus {
            format!("-0x{hex}")
        } else {
            format!("0x{hex}")
        }
    }

    /// Converts a `BigInt` to a float, returning an error if the value
    /// is too large to represent as a finite float.
    #[rhai_fn(name = "to_float", pure, return_raw)]
    pub fn to_float(value: &mut BigInt) -> Result<rhai::FLOAT, Box<rhai::EvalAltResult>> {
        value
            .to_f64()
            .map(|f| f as rhai::FLOAT)
            .filter(|f| f.is_finite())
            .ok_or_else(|| "BigInt value is too large to represent as a finite float".into())
    }
}

def_package! {
    /// Arbitrary-precision BigInt for Rhai: `bigint()` constructor plus
    /// arithmetic (`+`, `-`, `*`, `/`, `%`), unary negation (`-`), and comparison operators.
    pub BigIntPackage(lib) {
        lib.set_custom_type::<BigInt>("BigInt");
        combine_with_exported_module!(lib, "bigint", bigint_functions);
    }
}

#[cfg(test)]
mod tests {
    use rhai::{Engine, packages::Package};

    use super::*;

    #[test]
    fn test_rhai_integration() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: BigInt = engine.eval("bigint(42)").unwrap();
        assert_eq!(result.to_string(), "42");

        let result: BigInt = engine
            .eval("bigint(\"123456789012345678901234567890\")")
            .unwrap();
        assert_eq!(result.to_string(), "123456789012345678901234567890");

        let result: BigInt = engine.eval("bigint(42) + bigint(58)").unwrap();
        assert_eq!(result.to_string(), "100");

        let result: bool = engine.eval("bigint(50) > bigint(42)").unwrap();
        assert!(result);

        let result: bool = engine.eval("bigint(42) == bigint(42)").unwrap();
        assert!(result);

        let result: bool = engine.eval("bigint(42) != bigint(100)").unwrap();
        assert!(result);
    }

    #[test]
    fn test_core_functionality() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: BigInt = engine
            .eval("bigint(1000000000000000000) + bigint(2000000000000000000)")
            .unwrap();
        assert_eq!(result.to_string(), "3000000000000000000");

        let result: BigInt = engine
            .eval("bigint(5000000000000000000) - bigint(1000000000000000000)")
            .unwrap();
        assert_eq!(result.to_string(), "4000000000000000000");

        let result: BigInt = engine.eval("bigint(1000000) * bigint(1000000)").unwrap();
        assert_eq!(result.to_string(), "1000000000000");

        let result: BigInt = engine
            .eval("bigint(1000000000000) / bigint(1000000)")
            .unwrap();
        assert_eq!(result.to_string(), "1000000");

        let result: BigInt = engine.eval("bigint(10) % bigint(3)").unwrap();
        assert_eq!(result.to_string(), "1");

        let result: BigInt = engine.eval("-bigint(42)").unwrap();
        assert_eq!(result.to_string(), "-42");
    }

    #[test]
    fn test_error_handling() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result = engine.eval::<BigInt>("bigint(\"not_a_number\")");
        assert!(result.is_err());

        let result = engine.eval::<BigInt>("bigint(42) / bigint(0)");
        assert!(result.is_err());

        let result = engine.eval::<BigInt>("bigint(42) % bigint(0)");
        assert!(result.is_err());
    }

    #[test]
    fn test_bigint_from_f64() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // fractional part is truncated toward zero
        let result: BigInt = engine.eval("bigint(1.5)").unwrap();
        assert_eq!(result.to_string(), "1");

        let result: BigInt = engine.eval("bigint(-2.9)").unwrap();
        assert_eq!(result.to_string(), "-2");

        // exactly representable whole-number floats convert exactly
        let result: BigInt = engine.eval("bigint(42.0)").unwrap();
        assert_eq!(result.to_string(), "42");

        // large float that exceeds i64 range
        let result: BigInt = engine.eval("bigint(1e30)").unwrap();
        assert_eq!(result.to_string(), "1000000000000000019884624838656");
    }

    #[test]
    fn test_bigint_from_f64_errors() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result = engine.eval::<BigInt>("bigint(1.0 / 0.0)");
        assert!(result.is_err(), "infinity should be rejected");

        let result = engine.eval::<BigInt>("bigint(0.0 / 0.0)");
        assert!(result.is_err(), "NaN should be rejected");
    }

    #[test]
    fn test_to_string() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: String = engine.eval("bigint(42).to_string()").unwrap();
        assert_eq!(result, "42");

        let result: String = engine.eval("bigint(-99).to_string()").unwrap();
        assert_eq!(result, "-99");

        let result: String = engine
            .eval("bigint(\"123456789012345678901234567890\").to_string()")
            .unwrap();
        assert_eq!(result, "123456789012345678901234567890");
    }

    #[test]
    fn test_to_hex() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: String = engine.eval("bigint(255).to_hex()").unwrap();
        assert_eq!(result, "0xff");

        let result: String = engine.eval("bigint(0).to_hex()").unwrap();
        assert_eq!(result, "0x0");

        let result: String = engine.eval("bigint(-255).to_hex()").unwrap();
        assert_eq!(result, "-0xff");

        let result: String = engine.eval("bigint(256).to_hex()").unwrap();
        assert_eq!(result, "0x100");
    }

    #[test]
    fn test_exponentiation() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: BigInt = engine.eval("bigint(2) ** 10").unwrap();
        assert_eq!(result.to_string(), "1024");

        let result: BigInt = engine.eval("bigint(10) ** 18").unwrap();
        assert_eq!(result.to_string(), "1000000000000000000");

        let result: BigInt = engine.eval("bigint(-3) ** 3").unwrap();
        assert_eq!(result.to_string(), "-27");

        let result: BigInt = engine.eval("bigint(5) ** 0").unwrap();
        assert_eq!(result.to_string(), "1");

        // negative exponent should be rejected
        let result = engine.eval::<BigInt>("bigint(2) ** -1");
        assert!(result.is_err(), "negative exponent should be rejected");
    }

    #[test]
    fn test_bitwise_operators() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // AND
        let result: BigInt = engine.eval("bigint(0b1100) & bigint(0b1010)").unwrap();
        assert_eq!(result.to_string(), "8"); // 0b1000

        let result: BigInt = engine.eval("bigint(255) & bigint(15)").unwrap();
        assert_eq!(result.to_string(), "15");

        // OR
        let result: BigInt = engine.eval("bigint(0b1100) | bigint(0b1010)").unwrap();
        assert_eq!(result.to_string(), "14"); // 0b1110

        let result: BigInt = engine.eval("bigint(240) | bigint(15)").unwrap();
        assert_eq!(result.to_string(), "255");

        // XOR
        let result: BigInt = engine.eval("bigint(0b1100) ^ bigint(0b1010)").unwrap();
        assert_eq!(result.to_string(), "6"); // 0b0110

        let result: BigInt = engine.eval("bigint(255) ^ bigint(255)").unwrap();
        assert_eq!(result.to_string(), "0");

        // Left shift
        let result: BigInt = engine.eval("bigint(1) << 10").unwrap();
        assert_eq!(result.to_string(), "1024");

        let result: BigInt = engine.eval("bigint(1) << 64").unwrap();
        assert_eq!(result.to_string(), "18446744073709551616");

        // Right shift
        let result: BigInt = engine.eval("bigint(1024) >> 3").unwrap();
        assert_eq!(result.to_string(), "128");

        let result: BigInt = engine.eval("bigint(1) >> 1").unwrap();
        assert_eq!(result.to_string(), "0");

        // Negative shift amount should be rejected
        let result = engine.eval::<BigInt>("bigint(1) << -1");
        assert!(result.is_err(), "negative left-shift should be rejected");

        let result = engine.eval::<BigInt>("bigint(1) >> -1");
        assert!(result.is_err(), "negative right-shift should be rejected");
    }

    #[test]
    fn test_to_float() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: rhai::FLOAT = engine.eval("bigint(42).to_float()").unwrap();
        assert_eq!(result, 42.0);

        let result: rhai::FLOAT = engine.eval("bigint(-7).to_float()").unwrap();
        assert_eq!(result, -7.0);

        // value too large to be finite in f64
        let result = engine.eval::<rhai::FLOAT>(
            "bigint(\"999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\").to_float()"
        );
        assert!(result.is_err(), "overflow to infinity should be rejected");
    }
}
