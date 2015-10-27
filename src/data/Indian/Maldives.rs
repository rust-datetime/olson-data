
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Indian/Maldives",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 17640, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-315636840), utc_offset: 18000, dst_offset: 0, name: "MVT" },
    ],
};

