
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Atlantic/Cape_Verde",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -1556,  // UTC offset -1556, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1988148844, FixedTimespan {  // 1907-01-01T00:25:56.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CVT"),
        }),
        (-862610400, FixedTimespan {  // 1942-09-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CVST"),
        }),
        (-764118000, FixedTimespan {  // 1945-10-15T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CVT"),
        }),
        (186120000, FixedTimespan {  // 1975-11-25T04:00:00.000 UTC
            offset: -3600,  // UTC offset -3600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CVT"),
        }),
    ]},
};


