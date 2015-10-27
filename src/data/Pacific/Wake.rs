
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Wake",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 39988, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2177492788), utc_offset: 43200, dst_offset: 0, name: "WAKT" },
    ],
};

