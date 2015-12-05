
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Indian/Mahe",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 13308,  // UTC offset 13308, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2006653308, FixedTimespan {  // 1906-05-31T20:18:12.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SCT"),
        }),
    ]},
};


