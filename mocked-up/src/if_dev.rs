/// Conditionally compiles an item if compiling a development build.
#[macro_export]
macro_rules! if_dev {
    ( $d:item ) => {
        #[cfg(debug_assertions)]
        $d
    };
}
