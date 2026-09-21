
#[macro_export]
macro_rules! shared {
    ($name:ident, $ty:ty) => {

        static $name: std::sync::OnceLock<std::sync::Arc<$ty>> = std::sync::OnceLock::new();
    }
}


#[macro_export]
macro_rules! get_or_init {
    ($value:expr, $fun:expr) => {

        $value.get_or_init(|| std::sync::Arc::new($fun())).clone()
    }
}
