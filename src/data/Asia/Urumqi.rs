
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Urumqi",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: 21020,  // UTC offset 21020, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-1325483420),
            offset: 21600,  // UTC offset 21600, DST offset 0
            name: "XJT",
        },
    ],
};


