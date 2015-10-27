
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Brunei",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 27580, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1383464380), utc_offset: 27000, dst_offset: 0, name: "BNT" },
        Transition { occurs_at: Some(-1167636600), utc_offset: 28800, dst_offset: 0, name: "BNT" },
    ],
};

