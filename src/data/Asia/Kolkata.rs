
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Kolkata",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: 21208,  // UTC offset 21208, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-2840162008),
            offset: 21200,  // UTC offset 21200, DST offset 0
            name: "HMT",
        },
        Transition {
            occurs_at: Some(-891582800),
            offset: 23400,  // UTC offset 23400, DST offset 0
            name: "BURT",
        },
        Transition {
            occurs_at: Some(-872058600),
            offset: 19800,  // UTC offset 19800, DST offset 0
            name: "IST",
        },
        Transition {
            occurs_at: Some(-862637400),
            offset: 23400,  // UTC offset 19800, DST offset 3600
            name: "IST",
        },
        Transition {
            occurs_at: Some(-764145000),
            offset: 19800,  // UTC offset 19800, DST offset 0
            name: "IST",
        },
    ],
};


