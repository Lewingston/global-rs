
#[macro_export]
macro_rules! shared {
    ($name:ident, $ty:ty) => {

        thread_local! {
            static $name: std::cell::OnceCell<std::sync::Arc<$ty>> = std::cell::OnceCell::new()
        }
    }
}


#[macro_export]
macro_rules! get_or_init {
    ($value:expr, $fun:expr) => {

        $value.with(|global| {
            global.get_or_init(|| std::sync::Arc::new($fun())).clone()
        })
    }
}
