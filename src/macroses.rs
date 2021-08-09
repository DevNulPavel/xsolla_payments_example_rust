#[macro_export]
macro_rules! unwrap_ok_or_else {
    ($r:expr, |$err_val:ident| $or:expr ) => {
        match $r {
            Ok(data) => data,
            Err($err_val) => $or,
        }
    };
}

#[macro_export]
macro_rules! unwrap_some_or_else {
    ($r:expr, || $or:expr ) => {
        match $r {
            Some(data) => data,
            None => $or,
        }
    };
}

#[macro_export]
macro_rules! ok_or_else {
    ($r:expr, |$err_val:ident| $or:expr ) => {
        if let Err($err_val) = $r {
            $or
        }
    };
}
