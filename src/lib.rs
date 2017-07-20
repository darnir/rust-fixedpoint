//! # FixedPoint
//!
//! A simple library for computing the fixed point of a given function
//!
//! ## Example
//! ```rust
//! extern crate fixedpoint;
//!
//! use fixedpoint::fixedpoint;
//!
//! fn func_with_fixed_point(num: u32, param: &u32) -> u32 {
//!     150 + (((num as f32 / param.clone() as f32).ceil() as u32)*100)
//! }
//!
//! fn main() {
//!      let val = fixedpoint(&func_with_fixed_point, 0, &150, None, None).unwrap();
//!      println!("Fixed Point of function exists at: {}", val);
//! }
//! ```


// Clippy Lints
// #![warn(cast_possible_truncation)]
// #![warn(cast_possible_wrap)]
// #![warn(cast_precision_loss)]
// #![warn(cast_sign_loss)]
// #![warn(empty_enum)]
// #![warn(enum_glob_use)]
// #![warn(filter_map)]
// #![warn(if_not_else)]
// #![warn(indexing_slicing)]
// #![warn(invalid_upcast_comparisons)]
// #![warn(items_after_statements)]
// #![warn(missing_docs_in_private_items)]
// #![warn(mut_mut)]
// #![warn(nonminimal_bool)]
// #![warn(option_map_unwrap_or)]
// #![warn(option_map_unwrap_or_else)]
// #![warn(option_unwrap_used)]
// #![warn(pub_enum_variant_names)]
// #![warn(result_unwrap_used)]
// #![warn(shadow_reuse)]
// #![warn(shadow_same)]
// #![warn(shadow_unrelated)]
// #![warn(similar_names)]
// #![warn(single_match_else)]
// #![warn(stutter)]
// #![warn(wrong_pub_self_convention)]

#![warn(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;

#[allow(missing_docs)]
pub mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        errors {
            ValueLimit
            IterLimit(limit: usize) {
                description("Max iteration limit exceeded")
                display("Could not converge function after {} iterations", limit)
            }
        }
    }
}

use errors::*;


/// Compute the fixed point of a given function
///
/// Given a function `func`, compute the fixed point of the function such that `func(x, args) = x`
/// `args` is a reference to any object that the function uses as auxillary data
/// `maxiter` is used to limit the maximum number of iterations that the function will go through
/// before giving up. The default is 100.
/// If a `maxval` is provided, then the fixedpoint computation will stop when the value of the
/// function exceeds `maxval`. This is mostly useful in providing upper bounds on monotonic
/// functions.
///
/// The `args` parameter is generic. So you can pass arbitrary data types to it that only the
/// provided function needs to know how to interpret. This allows you to pass a `Option`, `struct`,
/// `Result` or any other type of complex object as well.
pub fn fixedpoint<T, U>(func: &Fn(U, &T) -> U, x0: U, args: &T, maxiter: Option<usize>, maxval: Option<U>) -> Result<U>
where U: PartialEq + PartialOrd + Copy + std::fmt::Debug {
    let maxiter = maxiter.unwrap_or(100);
    let mut itr = maxiter;
    let mut x = x0;
    let mut val = func(x0, args);
    while val != x {
        trace!("Iteration: {}", maxiter - itr);
        x = val;
        val = func(x, args);
        trace!("\tx: {:?}; F(x): {:?}", x, val);
        itr -= 1;
        if itr == 0 {
            debug!("Max Iterations reached. Last value: {:?}", val);
            bail!(ErrorKind::IterLimit(maxiter));
        } else if val > maxval.unwrap_or(val) {
            debug!("Maximum value of function reached. Last value: {:?}", val);
            bail!(ErrorKind::ValueLimit);
        }
    };
    Ok(val)
}

#[cfg(test)]
mod tests {
    use fixedpoint;

    fn func_with_fixed_point(num: u32, param: &u32) -> u32 {
        150 + (((num as f32 / param.clone() as f32).ceil() as u32)*100)
    }

    #[test]
    fn basic_tests() {
        assert_eq!(fixedpoint(&func_with_fixed_point, 0, &150, None, None).unwrap(), 450);
        assert_eq!(fixedpoint(&func_with_fixed_point, 0, &150, None, Some(400)).is_err(), true);
        assert_eq!(fixedpoint(&func_with_fixed_point, 0, &10, Some(5), None).is_err(), true);
    }

    #[test]
    #[should_panic(expected = "attempt to multiply with overflow")]
    fn maxiter() {
        fixedpoint(&func_with_fixed_point, 0, &10, Some(10), None).unwrap_or(0);
    }
}
