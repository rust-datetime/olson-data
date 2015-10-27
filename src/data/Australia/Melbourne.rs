
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Australia/Melbourne",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 34792, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2364111592), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(-1672567140), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(-1665392400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(-883641600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(-876128400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(-860400000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(-844678800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(-828345600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(-813229200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(57686400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(67968000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(89136000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(100022400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(120585600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(131472000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(152035200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(162921600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(183484800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(194976000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(215539200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(226425600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(246988800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(257875200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(278438400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(289324800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(309888000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(320774400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(341337600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(352224000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(372787200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(384278400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(404841600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(415728000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(436291200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(447177600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(467740800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(478627200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(499190400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(511286400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(530035200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(542736000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(561484800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(574790400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(594144000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(606240000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(625593600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(637689600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(657043200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(667929600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(688492800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(699465600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(719942400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(731433600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(751996800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(762883200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(783446400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(796147200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(814896000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(828201600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(846345600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(859651200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(877795200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(891100800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(909244800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(922550400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(941299200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(954000000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(967305600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(985449600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1004198400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1017504000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1035648000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1048953600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1067097600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1080403200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1099152000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1111852800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1130601600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1143907200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1162051200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1174752000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1193500800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1207411200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1223136000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1238860800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1254585600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1270310400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1286035200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1301760000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1317484800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1333209600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1349539200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1365264000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1380988800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1396713600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1412438400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1428163200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1443888000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1459612800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1475337600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1491062400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1506787200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1522512000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1538841600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1554566400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1570291200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1586016000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1601740800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1617465600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1633190400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1648915200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1664640000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1680364800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1696089600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1712419200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1728144000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1743868800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1759593600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1775318400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1791043200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1806768000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1822492800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1838217600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1853942400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1869667200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1885996800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1901721600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1917446400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1933171200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1948896000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1964620800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(1980345600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(1996070400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2011795200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2027520000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2043244800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2058969600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2075299200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2091024000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2106748800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2122473600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2138198400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2153923200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2169648000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2185372800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2201097600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2216822400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2233152000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2248876800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2264601600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2280326400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2296051200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2311776000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2327500800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2343225600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2358950400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2374675200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2390400000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2406124800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2422454400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2438179200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2453904000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2469628800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2485353600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2501078400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2516803200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2532528000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2548252800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2563977600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2579702400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2596032000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2611756800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2627481600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2643206400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2658931200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2674656000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2690380800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2706105600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2721830400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2737555200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2753280000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2769609600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2785334400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2801059200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2816784000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2832508800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2848233600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2863958400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2879683200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2895408000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2911132800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2926857600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2942582400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2958912000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(2974636800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(2990361600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3006086400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3021811200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3037536000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3053260800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3068985600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3084710400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3100435200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3116764800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3132489600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3148214400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3163939200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3179664000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3195388800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3211113600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3226838400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3242563200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3258288000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3274012800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3289737600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3306067200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3321792000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3337516800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3353241600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3368966400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3384691200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3400416000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3416140800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3431865600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3447590400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3463315200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3479644800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3495369600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3511094400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3526819200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3542544000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3558268800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3573993600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3589718400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3605443200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3621168000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3636892800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3653222400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3668947200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3684672000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3700396800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3716121600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3731846400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3747571200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3763296000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3779020800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3794745600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3810470400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3826195200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3842524800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3858249600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3873974400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3889699200), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3905424000), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3921148800), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3936873600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3952598400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(3968323200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(3984048000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(4000377600), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(4016102400), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(4031827200), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(4047552000), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(4063276800), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
        Transition { occurs_at: Some(4079001600), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(4094726400), utc_offset: 36000, dst_offset: 3600, name: "AEDT" },
    ],
};

