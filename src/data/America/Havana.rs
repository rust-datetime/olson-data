
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Havana",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -16232, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2524505368), utc_offset: -16224, dst_offset: 0, name: "HMT" },
        Transition { occurs_at: Some(-1402817376), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-1311534000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-1300996800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-933534000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-925675200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-902084400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-893620800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-870030000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-862171200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-775681200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-767822400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-744231600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-736372800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-144702000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-134251200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-113425200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-102542400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-86295600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-72907200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-54154800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-41457600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-21495600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-5774400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(9954000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(25675200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(41403600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(57729600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(73458000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(87364800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(104907600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(118900800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(136357200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(150436800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(167806800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(183528000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(199256400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(215582400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(230706000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(247032000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(263365200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(276667200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(290581200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(308721600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(322030800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(340171200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(358318800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(371620800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(389768400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(403070400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(421218000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(434520000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(452667600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(466574400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(484117200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(498024000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(511333200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(529473600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(542782800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(560923200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(574837200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(592372800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(606286800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(623822400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(638946000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(655876800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(671000400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(687330000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(702450000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(718779600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(733899600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(750229200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(765349200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(781678800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(796798800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(813128400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(828853200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(844578000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(860302800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(876632400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(891147600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(909291600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(922597200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(941346000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(954651600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(972795600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(986101200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1004245200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1018155600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1035694800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1049605200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1067144400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1080450000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1162098000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1173589200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1193547600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1205643600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1224997200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1236488400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1256446800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1268542800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1288501200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1300597200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1321160400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1333256400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1352005200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1362891600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1383454800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1394341200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1414904400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1425790800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1446354000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1457845200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1478408400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1489294800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1509858000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1520744400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1541307600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1552194000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1572757200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1583643600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1604206800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1615698000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1636261200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1647147600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1667710800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1678597200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1699160400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1710046800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1730610000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1741496400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1762059600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1772946000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1793509200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1805000400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1825563600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1836450000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1857013200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1867899600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1888462800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1899349200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1919912400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1930798800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1951362000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1962853200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(1983416400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(1994302800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2014866000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2025752400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2046315600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2057202000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2077765200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2088651600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2109214800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2120101200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2140664400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2152155600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2172718800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2183605200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2204168400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2215054800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2235618000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2246504400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2267067600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2277954000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2298517200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2309403600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2329966800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2341458000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2362021200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2372907600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2393470800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2404357200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2424920400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2435806800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2456370000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2467256400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2487819600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2499310800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2519874000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2530760400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2551323600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2562210000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2582773200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2593659600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2614222800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2625109200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2645672400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2656558800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2677122000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2688613200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2709176400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2720062800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2740626000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2751512400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2772075600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2782962000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2803525200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2814411600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2834974800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2846466000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2867029200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2877915600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2898478800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2909365200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2929928400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2940814800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2961378000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(2972264400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(2992827600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3003714000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3024277200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3035768400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3056331600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3067218000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3087781200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3098667600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3119230800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3130117200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3150680400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3161566800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3182130000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3193016400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3213579600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3225070800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3245634000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3256520400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3277083600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3287970000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3308533200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3319419600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3339982800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3350869200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3371432400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3382923600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3403486800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3414373200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3434936400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3445822800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3466386000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3477272400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3497835600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3508722000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3529285200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3540171600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3560734800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3572226000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3592789200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3603675600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3624238800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3635125200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3655688400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3666574800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3687138000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3698024400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3718587600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3730078800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3750642000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3761528400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3782091600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3792978000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3813541200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3824427600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3844990800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3855877200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3876440400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3887326800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3907890000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3919381200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3939944400), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3950830800), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(3971394000), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(3982280400), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4002843600), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(4013730000), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4034293200), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(4045179600), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4065742800), utc_offset: -18000, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(4076629200), utc_offset: -18000, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(4097192400), utc_offset: -18000, dst_offset: 0, name: "CST" },
    ],
};

