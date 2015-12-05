
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Thimphu",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 21516,  // UTC offset 21516, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-706341516, FixedTimespan {  // 1947-08-14T18:01:24.000 UTC
            offset: 19800,  // UTC offset 19800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("IST"),
        }),
        (560025000, FixedTimespan {  // 1987-09-30T18:30:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("BTT"),
        }),
    ]},
};


