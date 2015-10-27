
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Australia/Lord_Howe",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 38180, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2364114980), utc_offset: 36000, dst_offset: 0, name: "AEST" },
        Transition { occurs_at: Some(352216800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(372785400), utc_offset: 37800, dst_offset: 3600, name: "LHDT" },
        Transition { occurs_at: Some(384273000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(404839800), utc_offset: 37800, dst_offset: 3600, name: "LHDT" },
        Transition { occurs_at: Some(415722600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(436289400), utc_offset: 37800, dst_offset: 3600, name: "LHDT" },
        Transition { occurs_at: Some(447172200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(467739000), utc_offset: 37800, dst_offset: 3600, name: "LHDT" },
        Transition { occurs_at: Some(478621800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(499188600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(511282800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(530033400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(542732400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(562087800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(574786800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(594142200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(606236400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(625591800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(636476400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(657041400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(667926000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(688491000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(699462000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(719940600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(731430000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(751995000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(762879600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(783444600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(794329200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(814894200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(828198000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(846343800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(859647600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(877793400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(891097200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(909243000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(922546800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(941297400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(953996400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(967303800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(985446000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1004196600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1017500400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1035646200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1048950000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1067095800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1080399600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1099150200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1111849200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1130599800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1143903600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1162049400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1174748400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1193499000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1207407600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1223134200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1238857200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1254583800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1270306800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1286033400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1301756400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1317483000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1333206000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1349537400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1365260400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1380987000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1396710000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1412436600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1428159600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1443886200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1459609200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1475335800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1491058800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1506785400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1522508400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1538839800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1554562800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1570289400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1586012400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1601739000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1617462000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1633188600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1648911600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1664638200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1680361200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1696087800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1712415600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1728142200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1743865200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1759591800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1775314800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1791041400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1806764400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1822491000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1838214000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1853940600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1869663600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1885995000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1901718000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1917444600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1933167600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1948894200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1964617200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(1980343800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(1996066800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2011793400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2027516400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2043243000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2058966000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2075297400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2091020400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2106747000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2122470000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2138196600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2153919600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2169646200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2185369200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2201095800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2216818800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2233150200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2248873200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2264599800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2280322800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2296049400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2311772400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2327499000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2343222000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2358948600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2374671600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2390398200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2406121200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2422452600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2438175600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2453902200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2469625200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2485351800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2501074800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2516801400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2532524400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2548251000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2563974000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2579700600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2596028400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2611755000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2627478000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2643204600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2658927600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2674654200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2690377200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2706103800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2721826800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2737553400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2753276400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2769607800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2785330800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2801057400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2816780400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2832507000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2848230000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2863956600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2879679600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2895406200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2911129200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2926855800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2942578800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2958910200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(2974633200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(2990359800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3006082800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3021809400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3037532400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3053259000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3068982000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3084708600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3100431600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3116763000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3132486000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3148212600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3163935600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3179662200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3195385200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3211111800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3226834800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3242561400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3258284400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3274011000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3289734000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3306065400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3321788400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3337515000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3353238000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3368964600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3384687600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3400414200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3416137200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3431863800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3447586800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3463313400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3479641200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3495367800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3511090800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3526817400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3542540400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3558267000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3573990000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3589716600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3605439600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3621166200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3636889200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3653220600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3668943600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3684670200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3700393200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3716119800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3731842800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3747569400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3763292400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3779019000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3794742000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3810468600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3826191600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3842523000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3858246000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3873972600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3889695600), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3905422200), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3921145200), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3936871800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3952594800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(3968321400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(3984044400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(4000375800), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(4016098800), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(4031825400), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(4047548400), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(4063275000), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
        Transition { occurs_at: Some(4078998000), utc_offset: 37800, dst_offset: 0, name: "LHST" },
        Transition { occurs_at: Some(4094724600), utc_offset: 37800, dst_offset: 1800, name: "LHDT" },
    ],
};

