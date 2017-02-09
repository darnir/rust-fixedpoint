/// Compute the fixed point of a given function
///
/// Given a function `func`, compute the fixed point of the function such that `func(x, args) = x`
/// `args` is a reference to any object that the function uses as auxillary data
/// `maxiter` is used to limit the maximum number of iterations that the function will go through
/// before giving up. The default is 100.
/// If a `maxval` is provided, then the fixedpoint computation will stop when the value of the
/// function exceeds `maxval`. This is mostly useful in providing upper bounds on monotonic
/// functions.
pub fn fixedpoint<T, U>(func: &Fn(U, &T) -> U, x0: U, args: &T, maxiter: Option<u32>, maxval: Option<U>) -> Result<U, ()>
where U: std::cmp::PartialEq + std::cmp::PartialOrd + Copy {
    let mut itr = maxiter.unwrap_or(100);
    let mut x = x0;
    let mut val = func(x0, args);
    while val != x {
        x = val;
        val = func(x, args);
        itr -= 1;
        if itr == 0 || val > maxval.unwrap_or(val) {
            return Err(());
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
