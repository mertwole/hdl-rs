#[macro_export]
macro_rules! concat_buses {
    ($only_one:expr) => {
        $only_one
    };
    ($first:expr, $($rest:expr),+) => {
        $crate::api::bus::BusOps::append($first, ($crate::concat_buses!($($rest),*)))
    };
}
