
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Australia/Darwin",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: 31400,  // UTC offset 31400, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-2364108200),
            offset: 32400,  // UTC offset 32400, DST offset 0
            name: "ACST",
        },
        Transition {
            occurs_at: Some(-2230189200),
            offset: 34200,  // UTC offset 34200, DST offset 0
            name: "ACST",
        },
        Transition {
            occurs_at: Some(-1672565340),
            offset: 37800,  // UTC offset 34200, DST offset 3600
            name: "ACDT",
        },
        Transition {
            occurs_at: Some(-1665390600),
            offset: 34200,  // UTC offset 34200, DST offset 0
            name: "ACST",
        },
        Transition {
            occurs_at: Some(-883639800),
            offset: 37800,  // UTC offset 34200, DST offset 3600
            name: "ACDT",
        },
        Transition {
            occurs_at: Some(-876126600),
            offset: 34200,  // UTC offset 34200, DST offset 0
            name: "ACST",
        },
        Transition {
            occurs_at: Some(-860398200),
            offset: 37800,  // UTC offset 34200, DST offset 3600
            name: "ACDT",
        },
        Transition {
            occurs_at: Some(-844677000),
            offset: 34200,  // UTC offset 34200, DST offset 0
            name: "ACST",
        },
        Transition {
            occurs_at: Some(-828343800),
            offset: 37800,  // UTC offset 34200, DST offset 3600
            name: "ACDT",
        },
        Transition {
            occurs_at: Some(-813227400),
            offset: 34200,  // UTC offset 34200, DST offset 0
            name: "ACST",
        },
    ],
};


