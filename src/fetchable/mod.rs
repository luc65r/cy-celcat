pub mod calendar;
pub mod event;

use serde::{Deserialize, Serialize};

pub trait Fetchable: for<'de> Deserialize<'de> {
    type Request: Serialize;

    const METHOD_NAME: &'static str;
}
