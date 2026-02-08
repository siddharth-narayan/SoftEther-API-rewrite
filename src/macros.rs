// This macro is meant to be called on a struct that is repr(C),
// contains C members at the beginning for compatibility,
// Rust members for implementation, and an c_compat() function
// to update C members to be inline with the true implementation
#[macro_export]
macro_rules! c_compat {
    (
        impl $impl_name:ty {
            $(
                $(#[$macros:meta])*
                $vis:vis fn $fn_name:ident (&mut self $(, $($args:tt)*)?) $(-> $ret:ty)? $fn_body:block
            )*
        }

    ) => {
        impl $impl_name {
            $(
                $(#[$macros])*
                $vis fn $fn_name(&mut self $(, $($args)*)?) $(-> $ret)? {
                    let __ret = (|| $fn_body)();
                    self.c_compat();
                    __ret
                }
            )*
        }
    };
}

/// Returns the first argument if any of the following arguments are null
#[macro_export]
macro_rules! nullcheck {
    ($ret:expr, $($ptr:expr),+ $(,)?) => {
        $(
            if ($ptr).is_null() {
                return $ret;
            }
        )+
    };
}

// Credit: https://users.rust-lang.org/t/extern-c-as-a-block/36626/3
#[macro_export]
macro_rules! c_export {
    ($( $(#[$meta:meta])* fn $name:ident ($($arg:ident: $arg_type:ty),*) $(-> $rtn_type:ty)? $body:block )*) => {
        $(
            $(#[$meta])*
            #[unsafe(no_mangle)]
            pub extern "C" fn $name($($arg: $arg_type),*) $(-> $rtn_type)? $body
        )*
    }
}