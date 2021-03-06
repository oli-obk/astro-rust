//! Some programming utilities

/// Returns a float rounded upto a certain number of decimal digits
pub fn round_upto_digits(float: f64, decimal_digits: i32) -> f64 {
    let d = 10_f64.powi(decimal_digits);
    (float * d).round() / d
}

/**
Evaluates a polynomial using Horner's method

# Arguments

* `$x`     : The independent variable
* `$c`     : The constant term
* `$($a),*`: Sequence of coefficient terms for `$x`
             in ascending powers of `$x`
**/
#[macro_use]
macro_rules! Horner_eval {
    ($x:expr, $c:expr, $($a:expr),*) => {
        {
            let mut y = $c;
            let mut u = 1.0;
            $(
                u *= $x;
                y += u * $a;
            )*
            y
        }
    }
}
