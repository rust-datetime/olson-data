
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Paramaribo",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -8360, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1861911640), utc_offset: -8348, dst_offset: 0, name: "PMT" },
        Transition { occurs_at: Some(-1104529252), utc_offset: -8364, dst_offset: 0, name: "PMT" },
        Transition { occurs_at: Some(-765322836), utc_offset: -9000, dst_offset: 0, name: "NEGT" },
        Transition { occurs_at: Some(465445800), utc_offset: -10800, dst_offset: 0, name: "SRT" },
    ],
};

