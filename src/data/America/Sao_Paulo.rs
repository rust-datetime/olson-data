
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Sao_Paulo",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -10412, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1767215188), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-1206957600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-1191362400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-1175374800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-1159826400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-633819600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-622069200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-602283600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-591832800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-570747600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-560210400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-539125200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-531352800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-195426000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-184197600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-155163600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-150069600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-128898000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-121125600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-99954000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-89589600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(-68418000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(-57967200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(499748400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(511236000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(530593200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(540266400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(562129200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(571197600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(592974000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(602042400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(624423600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(634701600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(656478000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(666756000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(687927600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(697600800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(719982000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(728445600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(750826800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(761709600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(782276400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(793159200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(813726000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(824004000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(844570800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(856058400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(876106800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(888717600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(908074800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(919562400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(938919600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(951616800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(970974000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(982461600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1003028400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1013911200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1036292400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1045360800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1066532400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1076810400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1099364400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1108864800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1129431600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1140314400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1162695600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1172368800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1192330800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1203213600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1224385200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1234663200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1255834800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1266717600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1287284400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1298167200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1318734000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1330221600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1350788400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1361066400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1382238000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1392516000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1413687600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1424570400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1445137200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1456020000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1476586800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1487469600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1508036400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1518919200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1540090800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1550368800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1571540400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1581818400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1602990000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1613872800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1634439600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1645322400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1665889200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1677376800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1697338800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1708221600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1729393200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1739671200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1760842800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1771725600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1792292400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1803175200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1823742000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1834624800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1855191600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1866074400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1887246000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1897524000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1918695600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1928973600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1950145200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1960423200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(1981594800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(1992477600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2013044400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2024532000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2044494000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2055376800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2076548400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2086826400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2107998000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2118880800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2139447600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2150330400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2170897200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2181780000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2202346800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2213229600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2234401200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2244679200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2265850800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2276128800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2297300400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2307578400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2328750000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2339632800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2360199600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2371082400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2391649200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2402532000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2423703600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2433981600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2455153200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2465431200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2486602800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2497485600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2518052400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2528935200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2549502000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2560384800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2580951600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2591834400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2613006000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2623284000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2644455600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2654733600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2675905200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2686788000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2707354800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2718237600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2738804400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2749687200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2770858800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2781136800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2802308400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2812586400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2833758000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2844036000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2865207600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2876090400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2896657200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2907540000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2928106800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2938989600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2960161200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(2970439200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(2991610800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3001888800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3023060400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3033943200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3054510000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3065392800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3085959600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3096842400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3118014000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3128292000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3149463600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3159741600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3180913200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3191191200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3212362800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3223245600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3243812400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3254695200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3275262000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3286144800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3307316400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3317594400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3338766000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3349044000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3370215600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3381098400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3401665200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3412548000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3433114800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3443997600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3464564400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3475447200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3496618800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3506896800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3528068400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3538346400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3559518000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3570400800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3590967600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3601850400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3622417200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3633300000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3654471600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3664749600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3685921200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3696199200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3717370800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3727648800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3748820400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3759703200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3780270000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3791152800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3811719600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3822602400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3843774000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3854052000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3875223600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3885501600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3906673200), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3917556000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3938122800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3949005600), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(3969572400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(3980455200), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(4001626800), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(4011904800), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(4033076400), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(4043354400), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(4064526000), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
        Transition { occurs_at: Some(4074804000), utc_offset: -10800, dst_offset: 0, name: "BRT" },
        Transition { occurs_at: Some(4095975600), utc_offset: -10800, dst_offset: 3600, name: "BRST" },
    ],
};

