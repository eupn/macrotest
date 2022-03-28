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