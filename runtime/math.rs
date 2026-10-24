macro_rules! math {
    ($($name:ident => $function:ident($($argument:ident),+);)+) => {
        $(
            #[unsafe(no_mangle)]
            pub extern "C" fn $name($($argument: f64),+) -> f64 {
                libm::$function($($argument),+)
            }
        )+
    };
}

math! {
    nassau_math_sin => sin(x);
    nassau_math_cos => cos(x);
    nassau_math_tan => tan(x);
    nassau_math_asin => asin(x);
    nassau_math_acos => acos(x);
    nassau_math_atan => atan(x);
    nassau_math_atan2 => atan2(y, x);
    nassau_math_exp => exp(x);
    nassau_math_pow => pow(x, y);
    nassau_math_log => log(x);
    nassau_math_log10 => log10(x);
    nassau_math_sinh => sinh(x);
    nassau_math_cosh => cosh(x);
    nassau_math_tanh => tanh(x);
}
