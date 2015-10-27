
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Sakhalin",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 34248, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2031039048), utc_offset: 32400, dst_offset: 0, name: "JCST" },
        Transition { occurs_at: Some(-768560400), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(354891600), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(370699200), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(386427600), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(402235200), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(417963600), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(433771200), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(449586000), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(465318000), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(481042800), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(496767600), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(512492400), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(528217200), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(543942000), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(559666800), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(575391600), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(591116400), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(606841200), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(622566000), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(638290800), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(654620400), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(670345200), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(686073600), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(695750400), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(701784000), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(717505200), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(733244400), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(748969200), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(764694000), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(780418800), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(796143600), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(811868400), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(828198000), utc_offset: 39600, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(846342000), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(859647600), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(877795200), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(891100800), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(909244800), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(922550400), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(941299200), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(954000000), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(972748800), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(985449600), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1004198400), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1017504000), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1035648000), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1048953600), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1067097600), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1080403200), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1099152000), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1111852800), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1130601600), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1143302400), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1162051200), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1174752000), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1193500800), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1206806400), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1224950400), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1238256000), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1256400000), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1269705600), utc_offset: 36000, dst_offset: 3600, name: "SAKST" },
        Transition { occurs_at: Some(1288454400), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1301155200), utc_offset: 39600, dst_offset: 0, name: "SAKT" },
        Transition { occurs_at: Some(1414249200), utc_offset: 36000, dst_offset: 0, name: "SAKT" },
    ],
};

