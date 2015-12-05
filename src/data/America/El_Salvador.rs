
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/El_Salvador",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -14592,  // UTC offset -14592, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1546286208, FixedTimespan {  // 1921-01-01T04:03:12.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (547020000, FixedTimespan {  // 1987-05-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (559717200, FixedTimespan {  // 1987-09-27T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
        (578469600, FixedTimespan {  // 1988-05-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("CDT"),
        }),
        (591166800, FixedTimespan {  // 1988-09-25T05:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("CST"),
        }),
    ]},
};


