
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Asuncion",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -7760, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2524513840), utc_offset: -7760, dst_offset: 0, name: "AMT" },
        Transition { occurs_at: Some(-1206395440), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(86760000), utc_offset: -10800, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(134017200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(181368000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(194497200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(212990400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(226033200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(244526400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(257569200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(276062400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(291783600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(307598400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(323406000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(339220800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(354942000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(370756800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(386478000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(402292800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(418014000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(433828800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(449636400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(465451200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(481172400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(496987200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(512708400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(528523200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(544244400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(560059200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(575866800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(591681600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(607402800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(625032000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(638938800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(654753600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(670474800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(686721600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(699418800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(718257600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(733546800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(749448000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(762318000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(780984000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(793767600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(812520000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(825649200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(844574400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(856666800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(876024000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(888721200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(907473600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(920775600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(938923200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(952225200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(970372800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(983674800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1002427200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1018148400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1030852800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1049598000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1062907200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1081047600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1097985600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1110682800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1129435200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1142132400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1160884800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1173582000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1192939200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1205031600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1224388800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1236481200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1255838400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1270954800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1286078400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1302404400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1317528000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1333854000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1349582400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1364094000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1381032000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1395543600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1412481600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1426993200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1443931200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1459047600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1475380800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1490497200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1506830400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1521946800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1538884800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1553396400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1570334400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1584846000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1601784000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1616900400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1633233600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1648350000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1664683200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1679799600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1696132800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1711249200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1728187200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1742698800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1759636800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1774148400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1791086400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1806202800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1822536000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1837652400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1853985600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1869102000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1886040000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1900551600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1917489600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1932001200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1948939200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1964055600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(1980388800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(1995505200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2011838400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2026954800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2043288000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2058404400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2075342400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2089854000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2106792000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2121303600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2138241600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2153358000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2169691200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2184807600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2201140800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2216257200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2233195200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2247706800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2264644800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2279156400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2296094400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2310606000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2327544000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2342660400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2358993600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2374110000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2390443200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2405559600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2422497600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2437009200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2453947200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2468458800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2485396800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2500513200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2516846400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2531962800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2548296000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2563412400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2579745600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2594862000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2611800000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2626311600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2643249600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2657761200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2674699200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2689815600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2706148800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2721265200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2737598400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2752714800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2769652800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2784164400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2801102400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2815614000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2832552000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2847668400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2864001600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2879118000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2895451200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2910567600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2926900800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2942017200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2958955200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(2973466800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(2990404800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3004916400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3021854400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3036970800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3053304000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3068420400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3084753600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3099870000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3116808000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3131319600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3148257600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3162769200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3179707200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3194218800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3211156800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3226273200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3242606400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3257722800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3274056000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3289172400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3306110400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3320622000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3337560000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3352071600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3369009600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3384126000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3400459200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3415575600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3431908800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3447025200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3463358400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3478474800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3495412800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3509924400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3526862400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3541374000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3558312000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3573428400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3589761600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3604878000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3621211200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3636327600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3653265600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3667777200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3684715200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3699226800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3716164800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3731281200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3747614400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3762730800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3779064000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3794180400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3810513600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3825630000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3842568000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3857079600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3874017600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3888529200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3905467200), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3920583600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3936916800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3952033200), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(3968366400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(3983482800), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(4000420800), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(4014932400), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(4031870400), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(4046382000), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(4063320000), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
        Transition { occurs_at: Some(4077831600), utc_offset: -14400, dst_offset: 0, name: "PYT" },
        Transition { occurs_at: Some(4094769600), utc_offset: -14400, dst_offset: 3600, name: "PYST" },
    ],
};


