
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Cayenne",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -9040, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1846272560), utc_offset: -14400, dst_offset: 0, name: "GFT" },
        Transition { occurs_at: Some(-71092800), utc_offset: -10800, dst_offset: 0, name: "GFT" },
    ],
};

