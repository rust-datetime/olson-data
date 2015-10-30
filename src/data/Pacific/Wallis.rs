
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Wallis",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: 44120,  // UTC offset 44120, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-2177496920),
            offset: 43200,  // UTC offset 43200, DST offset 0
            name: "WFT",
        },
    ],
};


