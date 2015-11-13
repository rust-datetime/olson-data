
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zone::{TimeZone, FixedTimespanSet, FixedTimespan};

pub const ZONE: TimeZone<'static> = TimeZone {
    name: "Africa/Nouakchott",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 968,  // UTC offset 968, DST offset 0
            is_dst: false,
            name:   "LMT",
        },
        rest: &[
        (-1830384968, FixedTimespan {  // 1911-11-31T23-43-52 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   "GMT",
        }),
    ]},
};


