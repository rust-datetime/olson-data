
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Jakarta",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 25632, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1451719200), utc_offset: 26400, dst_offset: 0, name: "JAVT" },
        Transition { occurs_at: Some(-1172906400), utc_offset: 27000, dst_offset: 0, name: "WIB" },
        Transition { occurs_at: Some(-876641400), utc_offset: 32400, dst_offset: 0, name: "JST" },
        Transition { occurs_at: Some(-766054800), utc_offset: 27000, dst_offset: 0, name: "WIB" },
        Transition { occurs_at: Some(-683883000), utc_offset: 28800, dst_offset: 0, name: "WIB" },
        Transition { occurs_at: Some(-620812800), utc_offset: 27000, dst_offset: 0, name: "WIB" },
        Transition { occurs_at: Some(-189415800), utc_offset: 25200, dst_offset: 0, name: "WIB" },
    ],
};

