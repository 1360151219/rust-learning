#[macro_export]
macro_rules! mvec {
    ( $($x:expr);* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(dbg!($x));
            )*
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! four {
    () => {
        1 + 3
    };
}
#[macro_export]
macro_rules! gibberish {
    (4 fn ['spang "whammo"] @_@) => {
        1 + four!()
    };
}

#[macro_export]
macro_rules! multi_add {
    ($($e:expr),+) => {
        {
            let mut res = 0;
            $(
                res += $e;
            )*
            res
        }
    };
}

#[macro_export]
macro_rules! repeat_two {
    ($($i:literal)*, $($i2:literal)*) => {
        {
            let mut v = vec![];
            $(
                v.push($i);
                v.push($i2);
            )*
            v
        }
    }
}
