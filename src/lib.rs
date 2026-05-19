#![doc = include_str!("../README.md")]

//! Arbitrary-precision BigInt support for Rhai scripts.
//!
//! Provides [`BigIntPackage`] (via `def_package!`) for registering BigInt
//! support into a Rhai [`Engine`](rhai::Engine).

use num_bigint::BigInt;
use rhai::{def_package, plugin::*};

/// Builds a "Cannot compare {lhs} with {rhs}; {advice}" runtime error.
#[cold]
#[inline(never)]
fn cross_type_cmp_err(
    lhs: &'static str,
    rhs: &'static str,
    advice: &'static str,
) -> Box<rhai::EvalAltResult> {
    format!("Cannot compare {lhs} with {rhs}; {advice}").into()
}

/// Generates an `#[export_module]` that raises a runtime error whenever a `BigInt`
/// is compared (via `==`, `!=`, `<`, `<=`, `>`, or `>=`) with `$t`, in either operand order.
macro_rules! bigint_cross_type_cmp_module {
    ($mod_name:ident, $t:ty, $type_name:literal, $advice:literal) => {
        #[export_module]
        pub(crate) mod $mod_name {
            use num_bigint::BigInt;
            use rhai::plugin::*;

            #[rhai_fn(name = "==", pure, return_raw)]
            pub fn eq_bigint(_l: &mut BigInt, _r: $t) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err("BigInt", $type_name, $advice))
            }

            #[rhai_fn(name = "!=", pure, return_raw)]
            pub fn ne_bigint(_l: &mut BigInt, _r: $t) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err("BigInt", $type_name, $advice))
            }

            #[rhai_fn(name = "==", pure, return_raw)]
            pub fn eq_type(_l: &mut $t, _r: BigInt) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err($type_name, "BigInt", $advice))
            }

            #[rhai_fn(name = "!=", pure, return_raw)]
            pub fn ne_type(_l: &mut $t, _r: BigInt) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err($type_name, "BigInt", $advice))
            }

            #[rhai_fn(name = "<", pure, return_raw)]
            pub fn lt_bigint(_l: &mut BigInt, _r: $t) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err("BigInt", $type_name, $advice))
            }

            #[rhai_fn(name = "<=", pure, return_raw)]
            pub fn le_bigint(_l: &mut BigInt, _r: $t) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err("BigInt", $type_name, $advice))
            }

            #[rhai_fn(name = ">", pure, return_raw)]
            pub fn gt_bigint(_l: &mut BigInt, _r: $t) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err("BigInt", $type_name, $advice))
            }

            #[rhai_fn(name = ">=", pure, return_raw)]
            pub fn ge_bigint(_l: &mut BigInt, _r: $t) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err("BigInt", $type_name, $advice))
            }

            #[rhai_fn(name = "<", pure, return_raw)]
            pub fn lt_type(_l: &mut $t, _r: BigInt) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err($type_name, "BigInt", $advice))
            }

            #[rhai_fn(name = "<=", pure, return_raw)]
            pub fn le_type(_l: &mut $t, _r: BigInt) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err($type_name, "BigInt", $advice))
            }

            #[rhai_fn(name = ">", pure, return_raw)]
            pub fn gt_type(_l: &mut $t, _r: BigInt) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err($type_name, "BigInt", $advice))
            }

            #[rhai_fn(name = ">=", pure, return_raw)]
            pub fn ge_type(_l: &mut $t, _r: BigInt) -> Result<bool, Box<rhai::EvalAltResult>> {
                Err(crate::cross_type_cmp_err($type_name, "BigInt", $advice))
            }
        }
    };
}

#[export_module]
mod bigint_functions {
    use num_bigint::{BigInt, Sign};
    use num_traits::{FromPrimitive, ToPrimitive, Zero};
    use rhai::INT;

    /// Creates a `BigInt` from an integer.
    pub fn parse_bigint(value: INT) -> BigInt {
        value.into()
    }

    /// Creates a `BigInt` from a float by truncating toward zero.
    #[rhai_fn(name = "parse_bigint", return_raw)]
    pub fn parse_bigint_from_float(value: rhai::FLOAT) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        BigInt::from_f64(value)
            .ok_or_else(|| format!("Cannot convert {value} to BigInt: value must be finite").into())
    }

    /// Creates a `BigInt` from a string.
    #[rhai_fn(name = "parse_bigint", return_raw)]
    pub fn parse_bigint_from_str(value: String) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        value
            .parse::<BigInt>()
            .map_err(|e| format!("Cannot parse {value:?} as BigInt: {e}").into())
    }

    /// Converts an integer to a `BigInt` via method-call syntax: `42.to_bigint()`.
    #[rhai_fn(name = "to_bigint", pure)]
    pub fn int_to_bigint(value: &mut INT) -> BigInt {
        BigInt::from(*value)
    }

    /// Converts a float to a `BigInt` by truncating toward zero.
    /// Returns an error for non-finite values (infinity, NaN).
    #[rhai_fn(name = "to_bigint", return_raw, pure)]
    pub fn float_to_bigint(value: &mut rhai::FLOAT) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        BigInt::from_f64(*value)
            .ok_or_else(|| format!("Cannot convert {value} to BigInt: value must be finite").into())
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
    pub fn pow(base: &mut BigInt, exp: INT) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        // The `exp < 0` branch is intentionally kept separate from the
        // `u32::try_from` below. Both would reject negative values, but
        // `try_from` would produce a generic "too large" message. The explicit
        // branch gives users a clearer "must be non-negative" diagnostic.
        if exp < 0 {
            return Err(format!("Exponent must be non-negative, got {exp}").into());
        }
        let exp_u32 = u32::try_from(exp).map_err(|_| -> Box<rhai::EvalAltResult> {
            format!("Exponent {exp} is too large, must be at most {}", u32::MAX).into()
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

    fn validate_shift_amount(shift: INT) -> Result<u32, Box<rhai::EvalAltResult>> {
        // Explicit negative check for the same reason as in `pow`: `try_from`
        // would also reject negatives, but with a "too large" message instead
        // of the more accurate "must be non-negative" one.
        if shift < 0 {
            return Err(format!("Shift amount must be non-negative, got {shift}").into());
        }

        u32::try_from(shift).map_err(|_| -> Box<rhai::EvalAltResult> {
            format!("Shift amount is too large, must be at most {}", u32::MAX).into()
        })
    }

    /// Left-shifts a `BigInt` by `shift` bits. `shift` must be non-negative and fit in `u32`.
    #[rhai_fn(name = "<<", pure, return_raw)]
    pub fn shl(value: &mut BigInt, shift: INT) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        let shift_u32 = validate_shift_amount(shift)?;
        Ok(value.clone() << shift_u32)
    }

    /// Right-shifts a `BigInt` by `shift` bits. `shift` must be non-negative and fit in `u32`.
    #[rhai_fn(name = ">>", pure, return_raw)]
    pub fn shr(value: &mut BigInt, shift: INT) -> Result<BigInt, Box<rhai::EvalAltResult>> {
        let shift_u32 = validate_shift_amount(shift)?;
        Ok(value.clone() >> shift_u32)
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
            .ok_or_else(|| {
                "BigInt value is out of range for float (magnitude overflows to infinity)".into()
            })
    }
}

bigint_cross_type_cmp_module!(
    bigint_int_cmp,
    rhai::INT,
    "int",
    "wrap the int first: x == parse_bigint(42)"
);

bigint_cross_type_cmp_module!(
    bigint_float_cmp,
    rhai::FLOAT,
    "float",
    "convert the float to BigInt first via to_bigint() (truncates toward zero): x.to_bigint() == y"
);

bigint_cross_type_cmp_module!(
    bigint_string_cmp,
    rhai::ImmutableString,
    "string",
    r#"parse both sides first: parse_bigint(x) == parse_bigint(y)"#
);

bigint_cross_type_cmp_module!(
    bigint_bool_cmp,
    bool,
    "bool",
    "convert the BigInt to int first if a boolean check is needed: x != parse_bigint(0)"
);

def_package! {
    /// Arbitrary-precision BigInt for Rhai: `bigint()` constructor plus
    /// arithmetic (`+`, `-`, `*`, `/`, `%`), unary negation (`-`), and comparison operators.
    pub BigIntPackage(lib) {
        lib.set_custom_type::<BigInt>("BigInt");
        combine_with_exported_module!(lib, "bigint", bigint_functions);
        combine_with_exported_module!(lib, "bigint_int_cmp", bigint_int_cmp);
        combine_with_exported_module!(lib, "bigint_float_cmp", bigint_float_cmp);
        combine_with_exported_module!(lib, "bigint_string_cmp", bigint_string_cmp);
        combine_with_exported_module!(lib, "bigint_bool_cmp", bigint_bool_cmp);
    }
}

#[cfg(test)]
mod tests {
    use rhai::{packages::Package, Engine};

    use super::*;

    /// Asserts that `script` produces a runtime error whose message contains
    /// `expected_fragment`, giving a clear failure message if either the eval
    /// succeeds or the error text doesn't match.
    #[track_caller]
    fn assert_cmp_error(engine: &Engine, script: &str, expected_fragment: &str) {
        match engine.eval::<bool>(script) {
            Ok(v) => panic!("expected error for `{script}`, got Ok({v})"),
            Err(e) => assert!(
                e.to_string().contains(expected_fragment),
                "script `{script}`\n  expected fragment: {expected_fragment:?}\n  actual error:    {e}"
            ),
        }
    }

    #[test]
    fn test_rhai_integration() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: BigInt = engine.eval("parse_bigint(42)").unwrap();
        assert_eq!(result.to_string(), "42");

        let result: BigInt = engine
            .eval("parse_bigint(\"123456789012345678901234567890\")")
            .unwrap();
        assert_eq!(result.to_string(), "123456789012345678901234567890");

        let result: BigInt = engine.eval("parse_bigint(42) + parse_bigint(58)").unwrap();
        assert_eq!(result.to_string(), "100");

        let result: bool = engine.eval("parse_bigint(50) > parse_bigint(42)").unwrap();
        assert!(result);

        let result: bool = engine.eval("parse_bigint(42) == parse_bigint(42)").unwrap();
        assert!(result);

        let result: bool = engine
            .eval("parse_bigint(42) != parse_bigint(100)")
            .unwrap();
        assert!(result);
    }

    // Integer literals like 1_000_000_000_000_000_000 exceed i32::MAX and are
    // rejected by the Rhai parser under `only_i32`. Use `parse_bigint("…")` for
    // large-value tests that must run in both configurations.
    #[cfg(not(feature = "only_i32"))]
    #[test]
    fn test_core_arithmetic_i64_literals() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: BigInt = engine
            .eval("parse_bigint(1000000000000000000) + parse_bigint(2000000000000000000)")
            .unwrap();
        assert_eq!(result.to_string(), "3000000000000000000");

        let result: BigInt = engine
            .eval("parse_bigint(5000000000000000000) - parse_bigint(1000000000000000000)")
            .unwrap();
        assert_eq!(result.to_string(), "4000000000000000000");

        let result: BigInt = engine
            .eval("parse_bigint(1000000) * parse_bigint(1000000)")
            .unwrap();
        assert_eq!(result.to_string(), "1000000000000");

        let result: BigInt = engine
            .eval("parse_bigint(1000000000000) / parse_bigint(1000000)")
            .unwrap();
        assert_eq!(result.to_string(), "1000000");

        let result: BigInt = engine.eval("parse_bigint(10) % parse_bigint(3)").unwrap();
        assert_eq!(result.to_string(), "1");

        let result: BigInt = engine.eval("-parse_bigint(42)").unwrap();
        assert_eq!(result.to_string(), "-42");
    }

    // Verify that INT-dispatch functions (`parse_bigint(INT)`, `.to_bigint()`,
    // `**`, `<<`) work correctly at i32::MAX and i32::MIN boundaries.
    #[cfg(feature = "only_i32")]
    #[test]
    fn test_only_i32_int_dispatch_boundaries() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // parse_bigint(INT) at i32::MAX / i32::MIN
        let result: BigInt = engine.eval("parse_bigint(2147483647)").unwrap();
        assert_eq!(result.to_string(), "2147483647");

        let result: BigInt = engine.eval("parse_bigint(-2147483648)").unwrap();
        assert_eq!(result.to_string(), "-2147483648");

        // int.to_bigint() at i32::MAX
        let result: BigInt = engine.eval("(2147483647).to_bigint()").unwrap();
        assert_eq!(result.to_string(), "2147483647");

        // pow with INT exponent: 2 ** 30 (largest safe i32 exponent)
        let result: BigInt = engine.eval("parse_bigint(2) ** 30").unwrap();
        assert_eq!(result.to_string(), "1073741824");

        // shift with INT shift amount: 1 << 31
        let result: BigInt = engine.eval("parse_bigint(1) << 31").unwrap();
        assert_eq!(result.to_string(), "2147483648");
    }

    // Equivalent arithmetic coverage using string-based construction so that
    // large values work even when rhai::INT = i32 (the `only_i32` feature).
    #[cfg(feature = "only_i32")]
    #[test]
    fn test_core_arithmetic_string_literals() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: BigInt = engine
            .eval(r#"parse_bigint("1000000000000000000") + parse_bigint("2000000000000000000")"#)
            .unwrap();
        assert_eq!(result.to_string(), "3000000000000000000");

        let result: BigInt = engine
            .eval(r#"parse_bigint("5000000000000000000") - parse_bigint("1000000000000000000")"#)
            .unwrap();
        assert_eq!(result.to_string(), "4000000000000000000");

        let result: BigInt = engine
            .eval(r#"parse_bigint("1000000") * parse_bigint("1000000")"#)
            .unwrap();
        assert_eq!(result.to_string(), "1000000000000");

        let result: BigInt = engine
            .eval(r#"parse_bigint("1000000000000") / parse_bigint("1000000")"#)
            .unwrap();
        assert_eq!(result.to_string(), "1000000");

        let result: BigInt = engine
            .eval(r#"parse_bigint("10") % parse_bigint("3")"#)
            .unwrap();
        assert_eq!(result.to_string(), "1");

        let result: BigInt = engine.eval(r#"-parse_bigint("42")"#).unwrap();
        assert_eq!(result.to_string(), "-42");
    }

    #[test]
    fn test_error_handling() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result = engine.eval::<BigInt>("parse_bigint(\"not_a_number\")");
        assert!(result.is_err());

        let result = engine.eval::<BigInt>("parse_bigint(42) / parse_bigint(0)");
        assert!(result.is_err());

        let result = engine.eval::<BigInt>("parse_bigint(42) % parse_bigint(0)");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_bigint_from_f64() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // fractional part is truncated toward zero
        let result: BigInt = engine.eval("parse_bigint(1.5)").unwrap();
        assert_eq!(result.to_string(), "1");

        let result: BigInt = engine.eval("parse_bigint(-2.9)").unwrap();
        assert_eq!(result.to_string(), "-2");

        // exactly representable whole-number floats convert exactly
        let result: BigInt = engine.eval("parse_bigint(42.0)").unwrap();
        assert_eq!(result.to_string(), "42");

        // large float that exceeds i64 range
        let result: BigInt = engine.eval("parse_bigint(1e30)").unwrap();
        assert_eq!(result.to_string(), "1000000000000000019884624838656");
    }

    #[test]
    fn test_parse_bigint_from_f64_errors() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result = engine.eval::<BigInt>("parse_bigint(1.0 / 0.0)");
        assert!(result.is_err(), "infinity should be rejected");

        let result = engine.eval::<BigInt>("parse_bigint(0.0 / 0.0)");
        assert!(result.is_err(), "NaN should be rejected");
    }

    #[test]
    fn test_to_string() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: String = engine.eval("parse_bigint(42).to_string()").unwrap();
        assert_eq!(result, "42");

        let result: String = engine.eval("parse_bigint(-99).to_string()").unwrap();
        assert_eq!(result, "-99");

        let result: String = engine
            .eval("parse_bigint(\"123456789012345678901234567890\").to_string()")
            .unwrap();
        assert_eq!(result, "123456789012345678901234567890");
    }

    #[test]
    fn test_to_hex() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: String = engine.eval("parse_bigint(255).to_hex()").unwrap();
        assert_eq!(result, "0xff");

        let result: String = engine.eval("parse_bigint(0).to_hex()").unwrap();
        assert_eq!(result, "0x0");

        let result: String = engine.eval("parse_bigint(-255).to_hex()").unwrap();
        assert_eq!(result, "-0xff");

        let result: String = engine.eval("parse_bigint(256).to_hex()").unwrap();
        assert_eq!(result, "0x100");
    }

    #[test]
    fn test_exponentiation() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: BigInt = engine.eval("parse_bigint(2) ** 10").unwrap();
        assert_eq!(result.to_string(), "1024");

        let result: BigInt = engine.eval("parse_bigint(10) ** 18").unwrap();
        assert_eq!(result.to_string(), "1000000000000000000");

        let result: BigInt = engine.eval("parse_bigint(-3) ** 3").unwrap();
        assert_eq!(result.to_string(), "-27");

        let result: BigInt = engine.eval("parse_bigint(5) ** 0").unwrap();
        assert_eq!(result.to_string(), "1");

        // negative exponent should be rejected
        let result = engine.eval::<BigInt>("parse_bigint(2) ** -1");
        assert!(result.is_err(), "negative exponent should be rejected");
    }

    #[test]
    fn test_to_bigint_method() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // from integer
        let result: BigInt = engine.eval("42.to_bigint()").unwrap();
        assert_eq!(result.to_string(), "42");

        let result: BigInt = engine.eval("(-99).to_bigint()").unwrap();
        assert_eq!(result.to_string(), "-99");

        let result: BigInt = engine.eval("0.to_bigint()").unwrap();
        assert_eq!(result.to_string(), "0");

        // from float — truncates toward zero
        let result: BigInt = engine.eval("1.5.to_bigint()").unwrap();
        assert_eq!(result.to_string(), "1");

        let result: BigInt = engine.eval("(-2.9).to_bigint()").unwrap();
        assert_eq!(result.to_string(), "-2");

        let result: BigInt = engine.eval("1e30.to_bigint()").unwrap();
        assert_eq!(result.to_string(), "1000000000000000019884624838656");

        // non-finite floats should be rejected
        let result = engine.eval::<BigInt>("(1.0 / 0.0).to_bigint()");
        assert!(result.is_err(), "infinity should be rejected");

        let result = engine.eval::<BigInt>("(0.0 / 0.0).to_bigint()");
        assert!(result.is_err(), "NaN should be rejected");
    }

    #[test]
    fn test_bitwise_operators() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // AND
        let result: BigInt = engine
            .eval("parse_bigint(0b1100) & parse_bigint(0b1010)")
            .unwrap();
        assert_eq!(result.to_string(), "8"); // 0b1000

        let result: BigInt = engine.eval("parse_bigint(255) & parse_bigint(15)").unwrap();
        assert_eq!(result.to_string(), "15");

        // OR
        let result: BigInt = engine
            .eval("parse_bigint(0b1100) | parse_bigint(0b1010)")
            .unwrap();
        assert_eq!(result.to_string(), "14"); // 0b1110

        let result: BigInt = engine.eval("parse_bigint(240) | parse_bigint(15)").unwrap();
        assert_eq!(result.to_string(), "255");

        // XOR
        let result: BigInt = engine
            .eval("parse_bigint(0b1100) ^ parse_bigint(0b1010)")
            .unwrap();
        assert_eq!(result.to_string(), "6"); // 0b0110

        let result: BigInt = engine
            .eval("parse_bigint(255) ^ parse_bigint(255)")
            .unwrap();
        assert_eq!(result.to_string(), "0");

        // Left shift
        let result: BigInt = engine.eval("parse_bigint(1) << 10").unwrap();
        assert_eq!(result.to_string(), "1024");

        let result: BigInt = engine.eval("parse_bigint(1) << 64").unwrap();
        assert_eq!(result.to_string(), "18446744073709551616");

        // Right shift
        let result: BigInt = engine.eval("parse_bigint(1024) >> 3").unwrap();
        assert_eq!(result.to_string(), "128");

        let result: BigInt = engine.eval("parse_bigint(1) >> 1").unwrap();
        assert_eq!(result.to_string(), "0");

        // Negative shift amount should be rejected
        let result = engine.eval::<BigInt>("parse_bigint(1) << -1");
        assert!(result.is_err(), "negative left-shift should be rejected");

        let result = engine.eval::<BigInt>("parse_bigint(1) >> -1");
        assert!(result.is_err(), "negative right-shift should be rejected");
    }

    #[test]
    fn test_cross_type_equality_errors() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // BigInt == int (both directions)
        assert_cmp_error(
            &engine,
            "parse_bigint(42) == 42",
            "Cannot compare BigInt with int",
        );
        assert_cmp_error(
            &engine,
            "parse_bigint(42) != 42",
            "Cannot compare BigInt with int",
        );
        assert_cmp_error(
            &engine,
            "42 == parse_bigint(42)",
            "Cannot compare int with BigInt",
        );
        assert_cmp_error(
            &engine,
            "42 != parse_bigint(42)",
            "Cannot compare int with BigInt",
        );

        // BigInt == float (both directions)
        assert_cmp_error(
            &engine,
            "parse_bigint(42) == 42.0",
            "Cannot compare BigInt with float",
        );
        assert_cmp_error(
            &engine,
            "parse_bigint(42) != 42.0",
            "Cannot compare BigInt with float",
        );
        assert_cmp_error(
            &engine,
            "42.0 == parse_bigint(42)",
            "Cannot compare float with BigInt",
        );
        assert_cmp_error(
            &engine,
            "42.0 != parse_bigint(42)",
            "Cannot compare float with BigInt",
        );

        // BigInt == string (both directions)
        assert_cmp_error(
            &engine,
            r#"parse_bigint(42) == "42""#,
            "Cannot compare BigInt with string",
        );
        assert_cmp_error(
            &engine,
            r#"parse_bigint(42) != "42""#,
            "Cannot compare BigInt with string",
        );
        assert_cmp_error(
            &engine,
            r#""42" == parse_bigint(42)"#,
            "Cannot compare string with BigInt",
        );
        assert_cmp_error(
            &engine,
            r#""42" != parse_bigint(42)"#,
            "Cannot compare string with BigInt",
        );

        // BigInt == bool (both directions)
        assert_cmp_error(
            &engine,
            "parse_bigint(1) == true",
            "Cannot compare BigInt with bool",
        );
        assert_cmp_error(
            &engine,
            "parse_bigint(1) != true",
            "Cannot compare BigInt with bool",
        );
        assert_cmp_error(
            &engine,
            "true == parse_bigint(1)",
            "Cannot compare bool with BigInt",
        );
        assert_cmp_error(
            &engine,
            "true != parse_bigint(1)",
            "Cannot compare bool with BigInt",
        );

        // BigInt == BigInt still works correctly
        assert!(engine
            .eval::<bool>("parse_bigint(42) == parse_bigint(42)")
            .unwrap());
        assert!(!engine
            .eval::<bool>("parse_bigint(42) != parse_bigint(42)")
            .unwrap());
    }

    #[test]
    fn test_cross_type_ordering_errors() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // int — all four operators, both directions
        for op in ["<", "<=", ">", ">="] {
            assert_cmp_error(
                &engine,
                &format!("parse_bigint(42) {op} 42"),
                "Cannot compare BigInt with int",
            );
            assert_cmp_error(
                &engine,
                &format!("42 {op} parse_bigint(42)"),
                "Cannot compare int with BigInt",
            );
        }

        // float — all four operators, both directions
        for op in ["<", "<=", ">", ">="] {
            assert_cmp_error(
                &engine,
                &format!("parse_bigint(42) {op} 42.0"),
                "Cannot compare BigInt with float",
            );
            assert_cmp_error(
                &engine,
                &format!("42.0 {op} parse_bigint(42)"),
                "Cannot compare float with BigInt",
            );
        }

        // string — all four operators, both directions
        for op in ["<", "<=", ">", ">="] {
            assert_cmp_error(
                &engine,
                &format!(r#"parse_bigint(42) {op} "42""#),
                "Cannot compare BigInt with string",
            );
            assert_cmp_error(
                &engine,
                &format!(r#""42" {op} parse_bigint(42)"#),
                "Cannot compare string with BigInt",
            );
        }

        // bool — all four operators, both directions
        for op in ["<", "<=", ">", ">="] {
            assert_cmp_error(
                &engine,
                &format!("parse_bigint(1) {op} true"),
                "Cannot compare BigInt with bool",
            );
            assert_cmp_error(
                &engine,
                &format!("true {op} parse_bigint(1)"),
                "Cannot compare bool with BigInt",
            );
        }

        // BigInt ordering among themselves still works
        assert!(engine
            .eval::<bool>("parse_bigint(1) < parse_bigint(2)")
            .unwrap());
        assert!(engine
            .eval::<bool>("parse_bigint(2) > parse_bigint(1)")
            .unwrap());
        assert!(engine
            .eval::<bool>("parse_bigint(1) <= parse_bigint(1)")
            .unwrap());
        assert!(engine
            .eval::<bool>("parse_bigint(1) >= parse_bigint(1)")
            .unwrap());
    }

    /// All string-producing expressions in Rhai yield `ImmutableString` at
    /// runtime, so every variant hits the same cross-type guard.
    #[test]
    fn test_cross_type_string_variants() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        // String literal (base case — already in the general test, repeated here
        // for clarity as the baseline for the variants below)
        assert_cmp_error(
            &engine,
            r#"parse_bigint(42) == "42""#,
            "Cannot compare BigInt with string",
        );

        // String stored in a variable
        assert_cmp_error(
            &engine,
            r#"let s = "42"; parse_bigint(42) == s"#,
            "Cannot compare BigInt with string",
        );
        assert_cmp_error(
            &engine,
            r#"let s = "42"; s == parse_bigint(42)"#,
            "Cannot compare string with BigInt",
        );

        // String returned from a built-in function call (to_string on a plain int)
        assert_cmp_error(
            &engine,
            "parse_bigint(42) == 42.to_string()",
            "Cannot compare BigInt with string",
        );
        assert_cmp_error(
            &engine,
            "42.to_string() == parse_bigint(42)",
            "Cannot compare string with BigInt",
        );

        // String produced by to_string() on a BigInt itself
        assert_cmp_error(
            &engine,
            "parse_bigint(42) == parse_bigint(42).to_string()",
            "Cannot compare BigInt with string",
        );

        // Template string / interpolation
        assert_cmp_error(
            &engine,
            "parse_bigint(42) == `${42}`",
            "Cannot compare BigInt with string",
        );
        assert_cmp_error(
            &engine,
            "`${42}` == parse_bigint(42)",
            "Cannot compare string with BigInt",
        );
    }

    #[test]
    fn test_to_float() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let result: rhai::FLOAT = engine.eval("parse_bigint(42).to_float()").unwrap();
        assert_eq!(result, 42.0);

        let result: rhai::FLOAT = engine.eval("parse_bigint(-7).to_float()").unwrap();
        assert_eq!(result, -7.0);

        // value too large to be finite in f64
        let result = engine.eval::<rhai::FLOAT>(
            "parse_bigint(\"999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999\").to_float()"
        );
        assert!(result.is_err(), "overflow to infinity should be rejected");
    }

    // Verify that all BigInt functions are visible via Rhai's metadata API.
    #[cfg(feature = "metadata")]
    #[test]
    fn test_metadata_signatures() {
        let mut engine = Engine::new();
        BigIntPackage::new().register_into_engine(&mut engine);

        let sigs = engine.gen_fn_signatures(false);
        let sigs_str = sigs.join("\n");

        let has_bigint_signature = |name: &str| {
            sigs.iter()
                .any(|sig| sig.contains(name) && sig.contains("BigInt"))
        };

        for name in &["to_bigint", "to_string", "to_hex", "to_float"] {
            assert!(
                has_bigint_signature(name),
                "expected BigInt-specific `{name}` signature in metadata, got:\n{sigs_str}"
            );
        }

        let parse_bigint_sigs: Vec<_> = sigs
            .iter()
            .filter(|sig| sig.contains("parse_bigint"))
            .collect();

        assert!(
            parse_bigint_sigs.len() >= 2,
            "expected multiple `parse_bigint` overloads in metadata, got:\n{sigs_str}"
        );

        assert!(
            parse_bigint_sigs.iter().any(|sig| {
                sig.contains("BigInt") && (sig.contains("INT") || sig.contains("i64"))
            }),
            "expected integer `parse_bigint` overload returning BigInt, got:\n{sigs_str}"
        );

        assert!(
            parse_bigint_sigs.iter().any(|sig| {
                sig.contains("BigInt")
                    && (sig.contains("string")
                        || sig.contains("String")
                        || sig.contains("ImmutableString")
                        || sig.contains("&str"))
            }),
            "expected string `parse_bigint` overload returning BigInt, got:\n{sigs_str}"
        );
    }
}
