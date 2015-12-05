
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Bogota",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -11024,  // UTC offset -11024, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2707678576, FixedTimespan {  // 1884-03-13T03:03:44.000 UTC
            offset: -11024,  // UTC offset -11024, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BMT"),
        }),
        (-1739048176, FixedTimespan {  // 1914-11-23T03:03:44.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("COT"),
        }),
        (704869200, FixedTimespan {  // 1992-05-03T05:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("COST"),
        }),
        (733896000, FixedTimespan {  // 1993-04-04T04:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("COT"),
        }),
    ]},
};


