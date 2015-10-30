
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Blanc-Sablon",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: -7892,  // UTC offset -7892, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-2713902508),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "AST",
        },
        Transition {
            occurs_at: Some(-1632074400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "ADT",
        },
        Transition {
            occurs_at: Some(-1615143600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "AST",
        },
        Transition {
            occurs_at: Some(-880221600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "AWT",
        },
        Transition {
            occurs_at: Some(-769395600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "APT",
        },
        Transition {
            occurs_at: Some(-765399600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "AST",
        },
    ],
};


