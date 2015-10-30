
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Santiago",
    transitions: &[
        Transition {
            occurs_at: None,
            offset: -11834,  // UTC offset -11834, DST offset 0
            name: "LMT",
        },
        Transition {
            occurs_at: Some(-2524509766),
            offset: -11834,  // UTC offset -11834, DST offset 0
            name: "SMT",
        },
        Transition {
            occurs_at: Some(-1892666566),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-1688410800),
            offset: -11834,  // UTC offset -11834, DST offset 0
            name: "SMT",
        },
        Transition {
            occurs_at: Some(-1619210566),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-1593806400),
            offset: -11834,  // UTC offset -11834, DST offset 0
            name: "SMT",
        },
        Transition {
            occurs_at: Some(-1335991366),
            offset: -14400,  // UTC offset -18000, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(-1317585600),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-1304362800),
            offset: -14400,  // UTC offset -18000, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(-1286049600),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-1272826800),
            offset: -14400,  // UTC offset -18000, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(-1254513600),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-1241290800),
            offset: -14400,  // UTC offset -18000, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(-1222977600),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-1209754800),
            offset: -14400,  // UTC offset -18000, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(-1191355200),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-1178132400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-870552000),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-865278000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-740520000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(-736376400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-718056000),
            offset: -18000,  // UTC offset -18000, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-713649600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-36619200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(-23922000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(-3355200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(7527600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(24465600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(37767600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(55915200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(69217200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(87969600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(100666800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(118209600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(132116400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(150868800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(163566000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(182318400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(195620400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(213768000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(227070000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(245217600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(258519600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(277272000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(289969200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(308721600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(321418800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(340171200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(353473200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(371620800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(384922800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(403070400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(416372400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(434520000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(447822000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(466574400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(479271600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(498024000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(510721200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(529473600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(545194800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(560923200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(574225200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(592372800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(605674800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(624427200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(637124400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(653457600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(668574000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(687326400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(700628400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(718776000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(732078000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(750225600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(763527600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(781675200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(794977200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(813729600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(826426800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(845179200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(859690800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(876628800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(889930800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(906868800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(923194800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(939528000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(952830000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(971582400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(984279600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1003032000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1015729200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1034481600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1047178800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1065931200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1079233200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1097380800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1110682800),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1128830400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1142132400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1160884800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1173582000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1192334400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1206846000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1223784000),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1237086000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1255233600),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1270350000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1286683200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1304823600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1313899200),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1335668400),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1346558400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1367118000),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1378612800),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1398567600),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1410062400),
            offset: -10800,  // UTC offset -14400, DST offset 3600
            name: "CLST",
        },
        Transition {
            occurs_at: Some(1430017200),
            offset: -14400,  // UTC offset -14400, DST offset 0
            name: "CLT",
        },
        Transition {
            occurs_at: Some(1430031600),
            offset: -10800,  // UTC offset -10800, DST offset 0
            name: "CLT",
        },
    ],
};


