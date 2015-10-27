
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Bissau",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -3460, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1830380540), utc_offset: -3600, dst_offset: 0, name: "WAT" },
        Transition { occurs_at: Some(157770000), utc_offset: 0, dst_offset: 0, name: "GMT" },
    ],
};

