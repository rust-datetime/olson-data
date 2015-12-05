
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Indian/Mauritius",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 13800,  // UTC offset 13800, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1988164200, FixedTimespan {  // 1906-12-31T20:10:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MUT"),
        }),
        (403041600, FixedTimespan {  // 1982-10-09T20:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MUST"),
        }),
        (417034800, FixedTimespan {  // 1983-03-20T19:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MUT"),
        }),
        (1224972000, FixedTimespan {  // 2008-10-25T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("MUST"),
        }),
        (1238274000, FixedTimespan {  // 2009-03-28T21:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MUT"),
        }),
    ]},
};


