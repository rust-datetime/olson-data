
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Antarctica/DumontDUrville",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 0, dst_offset: 0, name: "zzz" },
        Transition { occurs_at: Some(-725846400), utc_offset: 36000, dst_offset: 0, name: "PMT" },
        Transition { occurs_at: Some(-566992800), utc_offset: 0, dst_offset: 0, name: "zzz" },
        Transition { occurs_at: Some(-415497600), utc_offset: 36000, dst_offset: 0, name: "DDUT" },
    ],
};

