
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Bougainville",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: 37336,  // UTC offset 37336, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-2840178136),
            offset: 35312,  // UTC offset 35312, DST offset 0
            name: "PMMT",
        },
        Transition {
            occurs_at: Some(-2366790512),
            offset: 36000,  // UTC offset 36000, DST offset 0
            name: "PGT",
        },
        Transition {
            occurs_at: Some(-868010400),
            offset: 32400,  // UTC offset 32400, DST offset 0
            name: "JST",
        },
        Transition {
            occurs_at: Some(-768906000),
            offset: 36000,  // UTC offset 36000, DST offset 0
            name: "PGT",
        },
        Transition {
            occurs_at: Some(1419696000),
            offset: 39600,  // UTC offset 39600, DST offset 0
            name: "BST",
        },
    ],
};


