
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Anchorage",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 50424, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-3225362424), utc_offset: -28824, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2188958376), utc_offset: -36000, dst_offset: 0, name: "CAT" },
        Transition { occurs_at: Some(-880200000), utc_offset: -36000, dst_offset: 3600, name: "CAWT" },
        Transition { occurs_at: Some(-769363200), utc_offset: -36000, dst_offset: 3600, name: "CAPT" },
        Transition { occurs_at: Some(-765378000), utc_offset: -36000, dst_offset: 0, name: "CAT" },
        Transition { occurs_at: Some(-86882400), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(-21470400), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(-5749200), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(9979200), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(25700400), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(41428800), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(57754800), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(73483200), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(89204400), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(104932800), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(120654000), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(126705600), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(152103600), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(162388800), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(183553200), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(199281600), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(215607600), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(230731200), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(247057200), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(262785600), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(278506800), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(294235200), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(309956400), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(325684800), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(341406000), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(357134400), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(372855600), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(388584000), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(404910000), utc_offset: -36000, dst_offset: 0, name: "AHST" },
        Transition { occurs_at: Some(420033600), utc_offset: -36000, dst_offset: 3600, name: "AHDT" },
        Transition { occurs_at: Some(436359600), utc_offset: -32400, dst_offset: 0, name: "YST" },
        Transition { occurs_at: Some(439030800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(452084400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(467805600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(483534000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(499255200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(514983600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(530704800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(544618800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(562154400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(576068400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(594208800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(607518000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(625658400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(638967600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(657108000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(671022000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(688557600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(702471600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(720007200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(733921200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(752061600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(765370800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(783511200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(796820400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(814960800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(828874800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(846410400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(860324400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(877860000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(891774000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(909309600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(923223600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(941364000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(954673200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(972813600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(986122800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1004263200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1018177200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1035712800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1049626800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1067162400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1081076400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1099216800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1112526000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1130666400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1143975600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1162116000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1173610800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1194170400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1205060400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1225620000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1236510000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1257069600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1268564400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1289124000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1300014000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1320573600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1331463600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1352023200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1362913200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1383472800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1394362800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1414922400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1425812400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1446372000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1457866800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1478426400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1489316400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1509876000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1520766000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1541325600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1552215600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1572775200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1583665200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1604224800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1615719600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1636279200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1647169200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1667728800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1678618800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1699178400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1710068400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1730628000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1741518000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1762077600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1772967600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1793527200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1805022000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1825581600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1836471600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1857031200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1867921200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1888480800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1899370800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1919930400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1930820400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1951380000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1962874800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(1983434400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(1994324400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2014884000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2025774000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2046333600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2057223600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2077783200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2088673200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2109232800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2120122800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2140682400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2152177200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2172736800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2183626800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2204186400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2215076400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2235636000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2246526000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2267085600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2277975600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2298535200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2309425200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2329984800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2341479600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2362039200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2372929200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2393488800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2404378800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2424938400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2435828400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2456388000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2467278000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2487837600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2499332400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2519892000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2530782000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2551341600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2562231600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2582791200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2593681200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2614240800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2625130800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2645690400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2656580400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2677140000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2688634800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2709194400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2720084400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2740644000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2751534000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2772093600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2782983600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2803543200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2814433200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2834992800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2846487600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2867047200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2877937200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2898496800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2909386800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2929946400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2940836400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2961396000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(2972286000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(2992845600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3003735600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3024295200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3035790000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3056349600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3067239600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3087799200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3098689200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3119248800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3130138800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3150698400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3161588400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3182148000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3193038000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3213597600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3225092400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3245652000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3256542000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3277101600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3287991600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3308551200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3319441200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3340000800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3350890800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3371450400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3382945200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3403504800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3414394800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3434954400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3445844400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3466404000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3477294000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3497853600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3508743600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3529303200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3540193200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3560752800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3572247600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3592807200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3603697200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3624256800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3635146800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3655706400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3666596400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3687156000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3698046000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3718605600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3730100400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3750660000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3761550000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3782109600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3792999600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3813559200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3824449200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3845008800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3855898800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3876458400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3887348400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3907908000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3919402800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3939962400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3950852400), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(3971412000), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(3982302000), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(4002861600), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(4013751600), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(4034311200), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(4045201200), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(4065760800), utc_offset: -32400, dst_offset: 0, name: "AKST" },
        Transition { occurs_at: Some(4076650800), utc_offset: -32400, dst_offset: 3600, name: "AKDT" },
        Transition { occurs_at: Some(4097210400), utc_offset: -32400, dst_offset: 0, name: "AKST" },
    ],
};


