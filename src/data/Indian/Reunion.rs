
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Indian/Reunion",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 13312, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1848886912), utc_offset: 14400, dst_offset: 0, name: "RET" },
    ],
};

