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

#[macro_export]
macro_rules! test_compile_error {
    () => {
        compile_error!("test_compile_error")
    };
}

#[cfg(feature = "test-feature")]
#[cfg_attr(feature = "test-feature", macro_export)]
macro_rules! test_feature_vec {
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