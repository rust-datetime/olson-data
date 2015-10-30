
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Dhaka",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: 21700,  // UTC offset 21700, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-2524543300),
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
            offset: 23400,  // UTC offset 23400, DST offset 0
            name: "BURT",
        },
        Transition {
            occurs_at: Some(-576138600),
            offset: 21600,  // UTC offset 21600, DST offset 0
            name: "DACT",
        },
        Transition {
            occurs_at: Some(38772000),
            offset: 21600,  // UTC offset 21600, DST offset 0
            name: "BDT",
        },
        Transition {
            occurs_at: Some(1245430800),
            offset: 25200,  // UTC offset 21600, DST offset 3600
            name: "BDST",
        },
        Transition {
            occurs_at: Some(1262278800),
            offset: 21600,  // UTC offset 21600, DST offset 0
            name: "BDT",
        },
    ],
};


