
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Niue",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -38420, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2177414380), utc_offset: -38400, dst_offset: 0, name: "NUT" },
        Transition { occurs_at: Some(-599577600), utc_offset: -37800, dst_offset: 0, name: "NUT" },
        Transition { occurs_at: Some(276085800), utc_offset: -39600, dst_offset: 0, name: "NUT" },
    ],
};

