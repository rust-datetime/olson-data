
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Adak",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 44001, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-3225356001), utc_offset: -36802, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2188950398), utc_offset: -39600, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-880196400), utc_offset: -39600, dst_offset: 3600, name: "NWT" },
        Transition { occurs_at: Some(-765374400), utc_offset: -39600, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-21466800), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(-5745600), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(9982800), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(25704000), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(41432400), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(57758400), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(73486800), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(89208000), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(104936400), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(120657600), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(126709200), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(152107200), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(162392400), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(183556800), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(199285200), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(215611200), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(230734800), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(247060800), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(262789200), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(278510400), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(294238800), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(309960000), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(325688400), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(341409600), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(357138000), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(372859200), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(388587600), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(404913600), utc_offset: -39600, dst_offset: 0, name: "BST" },
        Transition { occurs_at: Some(420037200), utc_offset: -39600, dst_offset: 3600, name: "BDT" },
        Transition { occurs_at: Some(436363200), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(452088000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(467809200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(483537600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(499258800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(514987200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(530708400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(544622400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(562158000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(576072000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(594212400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(607521600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(625662000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(638971200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(657111600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(671025600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(688561200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(702475200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(720010800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(733924800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(752065200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(765374400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(783514800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(796824000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(814964400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(828878400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(846414000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(860328000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(877863600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(891777600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(909313200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(923227200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(941367600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(954676800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(972817200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(986126400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1004266800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1018180800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1035716400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1049630400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1067166000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1081080000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1099220400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1112529600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1130670000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1143979200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1162119600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1173614400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1194174000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1205064000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1225623600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1236513600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1257073200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1268568000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1289127600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1300017600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1320577200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1331467200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1352026800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1362916800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1383476400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1394366400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1414926000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1425816000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1446375600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1457870400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1478430000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1489320000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1509879600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1520769600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1541329200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1552219200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1572778800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1583668800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1604228400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1615723200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1636282800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1647172800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1667732400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1678622400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1699182000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1710072000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1730631600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1741521600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1762081200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1772971200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1793530800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1805025600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1825585200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1836475200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1857034800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1867924800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1888484400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1899374400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1919934000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1930824000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1951383600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1962878400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(1983438000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(1994328000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2014887600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2025777600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2046337200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2057227200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2077786800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2088676800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2109236400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2120126400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2140686000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2152180800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2172740400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2183630400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2204190000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2215080000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2235639600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2246529600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2267089200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2277979200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2298538800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2309428800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2329988400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2341483200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2362042800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2372932800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2393492400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2404382400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2424942000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2435832000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2456391600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2467281600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2487841200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2499336000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2519895600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2530785600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2551345200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2562235200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2582794800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2593684800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2614244400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2625134400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2645694000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2656584000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2677143600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2688638400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2709198000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2720088000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2740647600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2751537600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2772097200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2782987200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2803546800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2814436800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2834996400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2846491200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2867050800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2877940800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2898500400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2909390400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2929950000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2940840000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2961399600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(2972289600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(2992849200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3003739200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3024298800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3035793600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3056353200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3067243200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3087802800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3098692800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3119252400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3130142400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3150702000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3161592000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3182151600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3193041600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3213601200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3225096000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3245655600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3256545600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3277105200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3287995200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3308554800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3319444800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3340004400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3350894400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3371454000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3382948800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3403508400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3414398400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3434958000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3445848000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3466407600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3477297600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3497857200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3508747200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3529306800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3540196800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3560756400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3572251200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3592810800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3603700800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3624260400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3635150400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3655710000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3666600000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3687159600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3698049600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3718609200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3730104000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3750663600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3761553600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3782113200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3793003200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3813562800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3824452800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3845012400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3855902400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3876462000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3887352000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3907911600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3919406400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3939966000), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3950856000), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(3971415600), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(3982305600), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(4002865200), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(4013755200), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(4034314800), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(4045204800), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(4065764400), utc_offset: -36000, dst_offset: 0, name: "HST" },
        Transition { occurs_at: Some(4076654400), utc_offset: -36000, dst_offset: 3600, name: "HDT" },
        Transition { occurs_at: Some(4097214000), utc_offset: -36000, dst_offset: 0, name: "HST" },
    ],
};

