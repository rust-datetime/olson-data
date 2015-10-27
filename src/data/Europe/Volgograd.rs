
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Europe/Volgograd",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 10660, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1577761060), utc_offset: 10800, dst_offset: 0, name: "TSAT" },
        Transition { occurs_at: Some(-1247540400), utc_offset: 14400, dst_offset: 0, name: "STAT" },
        Transition { occurs_at: Some(354916800), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(370724400), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(386452800), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(402260400), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(417988800), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(433796400), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(449611200), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(465343200), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(481068000), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(496792800), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(512517600), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(528242400), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(543967200), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(559692000), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(575416800), utc_offset: 14400, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(591141600), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(606866400), utc_offset: 10800, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(622594800), utc_offset: 10800, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(638319600), utc_offset: 10800, dst_offset: 3600, name: "VOLST" },
        Transition { occurs_at: Some(654649200), utc_offset: 10800, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(670374000), utc_offset: 14400, dst_offset: 0, name: "VOLT" },
        Transition { occurs_at: Some(701820000), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(717534000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(733273200), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(748998000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(764722800), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(780447600), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(796172400), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(811897200), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(828226800), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(846370800), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(859676400), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(877820400), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(891126000), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(909270000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(922575600), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(941324400), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(954025200), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(972774000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(985474800), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1004223600), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1017529200), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1035673200), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1048978800), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1067122800), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1080428400), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1099177200), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1111878000), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1130626800), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1143327600), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1162076400), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1174777200), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1193526000), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1206831600), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1224975600), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1238281200), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1256425200), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1269730800), utc_offset: 10800, dst_offset: 3600, name: "MSD" },
        Transition { occurs_at: Some(1288479600), utc_offset: 10800, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1301180400), utc_offset: 14400, dst_offset: 0, name: "MSK" },
        Transition { occurs_at: Some(1414274400), utc_offset: 10800, dst_offset: 0, name: "MSK" },
    ],
};

