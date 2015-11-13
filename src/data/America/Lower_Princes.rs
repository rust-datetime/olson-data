
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "America/Lower_Princes",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -12253,  // UTC offset -12253, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1826742947, FixedTimespan {  // 1912-01-12T3-24-13 UTC
            offset: -12600,  // UTC offset -12600, DST offset 0
            is_dst: false,
            name:   "ANT",
        }),
        (-157753800, FixedTimespan {  // 1965-00-01T3-30-00 UTC
            offset: -14400,  // UTC offset -14400, DST offset 0
            is_dst: false,
            name:   "AST",
        }),
    ]},
};


