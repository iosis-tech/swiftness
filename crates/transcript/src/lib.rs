#![no_std]

extern crate alloc;

pub mod transcript;

#[cfg(test)]
pub mod tests;

#[macro_export]
macro_rules! ensure {
    ($condition:expr, $err:expr) => {
        if !$condition {
            return Err($err);
        }
    };
}

#[macro_export]
macro_rules! assure {
    ($condition:expr, $err:expr) => {
        if !$condition {
            Err($err)
        } else {
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! felt {
    ($from:expr) => {
        Felt::from($from)
    };
}