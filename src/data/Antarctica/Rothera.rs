
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Antarctica/Rothera",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: 0,  // UTC offset 0, DST offset 0
            name: "zzz",
        },
        Transition {
            occurs_at: Some(218246400),
            offset: -10800,  // UTC offset -10800, DST offset 0
            name: "ROTT",
        },
    ],
};


