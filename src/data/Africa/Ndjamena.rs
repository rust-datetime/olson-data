
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Africa/Ndjamena",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 3612,  // UTC offset 3612, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1830387612, FixedTimespan {  // 1911-12-31T22:59:48.000 UTC
            offset: 3600,  // UTC offset 3600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WAT"),
        }),
        (308703600, FixedTimespan {  // 1979-10-13T23:00:00.000 UTC
            offset: 7200,  // UTC offset 3600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("WAST"),
        }),
        (321314400, FixedTimespan {  // 1980-03-07T22:00:00.000 UTC
            offset: 3600,  // UTC offset 3600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("WAT"),
        }),
    ]},
};


