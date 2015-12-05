
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Africa/Dakar",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 968,  // UTC offset 968, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1830384968, FixedTimespan {  // 1911-12-31T23:43:52.000 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("GMT"),
        }),
    ]},
};


