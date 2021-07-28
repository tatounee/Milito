use yew::services::ConsoleService;


// The log! macro is copied from the crate seed
// See: https://docs.rs/seed/0.8.0/seed/macro.log.html

pub fn wrap_debug<T: std::fmt::Debug>(object: T) -> T {
    object
}

#[macro_export]
macro_rules! log {
    { $($expr:expr),* $(,)? } => {
        {
            let mut formatted_exprs = Vec::new();
            $(
                formatted_exprs.push(format!("{:#?}", $crate::utils::wrap_debug(&$expr)));
            )*
            $crate::utils::log_1(
                &formatted_exprs
                    .as_slice()
                    .join(" ")
            );
        }
     };
}

pub fn log_1(data_1: &str) {
    ConsoleService::log(data_1);
}
