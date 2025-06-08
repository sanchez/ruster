use crate::{Date, Time, TimeZone};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DateTime {
    date: Date,
    time: Time,
    timezone: TimeZone,
}
