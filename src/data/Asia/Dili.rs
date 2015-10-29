
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Dili",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 30140, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1830414140), utc_offset: 28800, dst_offset: 0, name: "TLT" },
        Transition { occurs_at: Some(-879152400), utc_offset: 32400, dst_offset: 0, name: "JST" },
        Transition { occurs_at: Some(-766054800), utc_offset: 32400, dst_offset: 0, name: "TLT" },
        Transition { occurs_at: Some(199897200), utc_offset: 28800, dst_offset: 0, name: "WITA" },
        Transition { occurs_at: Some(969120000), utc_offset: 32400, dst_offset: 0, name: "TLT" },
    ],
};


