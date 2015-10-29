
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Africa/Monrovia",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 2588, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2776984988), utc_offset: 2588, dst_offset: 0, name: "MMT" },
        Transition { occurs_at: Some(-1604364188), utc_offset: 2670, dst_offset: 0, name: "LRT" },
        Transition { occurs_at: Some(73523730), utc_offset: 0, dst_offset: 0, name: "GMT" },
    ],
};


