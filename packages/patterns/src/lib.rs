#[macro_export]
macro_rules! mvec {
    ( $( $x:expr );* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(dbg!($x));
            )*
            temp_vec
        }
    };
}
