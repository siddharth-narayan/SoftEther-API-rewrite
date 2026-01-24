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
