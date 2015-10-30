
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Guyana",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: -7640,  // UTC offset -7640, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-1730584360),
            offset: -8100,  // UTC offset -8100, DST offset 0
            name: "GBGT",
        },
        Transition {
            occurs_at: Some(-113694300),
            offset: -8100,  // UTC offset -8100, DST offset 0
            name: "GYT",
        },
        Transition {
            occurs_at: Some(176004900),
            offset: -10800,  // UTC offset -10800, DST offset 0
            name: "GYT",
        },
        Transition {
            occurs_at: Some(662698800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "GYT",
        },
    ],
};


