use log::{error, trace, warn};
use std::fmt;

pub trait LogErr<T> {
    fn log_unwrap(self) -> T;
    fn log_expect(self, msg: &str) -> T;
    fn log_error(self, msg: &str) -> Self;
    fn log_warn(self) -> Self;
}
impl<T> LogErr<T> for Option<T> {
    fn log_expect(self, msg: &str) -> T {
        match self {
            Some(v) => v,
            None => {
                error!("{msg}");
                panic!();
            }
        }
    }
    fn log_unwrap(self) -> T {
        match self {
            Some(v) => v,
            None => {
                error!("called `Option::unwrap()` on a `None` value");
                panic!();
            }
        }
    }
    fn log_error(self, msg: &str) -> Self {
        if self.is_none() {
            error!("{msg}")
        }
        self
    }
    fn log_warn(self) -> Self {
        if self.is_none() {
            warn!("called `Option::log_warn()` on a `None` value")
        }
        self
    }
}
impl<T, E> LogErr<T> for Result<T, E>
where
    E: fmt::Display,
{
    fn log_expect(self, msg: &str) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                trace!("Treatment of error {e}");
                error!("{msg}");
                panic!();
            }
        }
    }
    fn log_unwrap(self) -> T {
        match self {
            Ok(v) => v,
            Err(e) => {
                error!("{e}");
                panic!();
            }
        }
    }
    fn log_error(self, msg: &str) -> Self {
        self.inspect_err(|e| error!("{msg} ({e})"))
    }
    fn log_warn(self) -> Self {
        self.inspect_err(|e| warn!("{e}"))
    }
}
