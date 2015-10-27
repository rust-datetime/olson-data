
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Australia/Perth",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 27804, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2337925404), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(-1672559940), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(-1665385200), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(-883634400), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(-876121200), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(-860392800), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(-844671600), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(152042400), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(162928800), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(436298400), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(447184800), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(690314400), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(699472800), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(1165082400), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(1174759200), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(1193508000), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(1206813600), utc_offset: 28800, dst_offset: 0, name: "AWST" },
        Transition { occurs_at: Some(1224957600), utc_offset: 28800, dst_offset: 3600, name: "AWDT" },
        Transition { occurs_at: Some(1238263200), utc_offset: 28800, dst_offset: 0, name: "AWST" },
    ],
};

