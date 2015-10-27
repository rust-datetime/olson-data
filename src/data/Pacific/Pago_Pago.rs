
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Pacific/Pago_Pago",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 45432, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2855738232), utc_offset: -38232, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1861881768), utc_offset: -39600, dst_offset: 0, name: "NST" },
    ],
};

