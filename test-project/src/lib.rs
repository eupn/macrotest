#[macro_export]
macro_rules! test_vec {
    () => {
        Vec::new()
    };
    ( $( $x:expr ),+ ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
