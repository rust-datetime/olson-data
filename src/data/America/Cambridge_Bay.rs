
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Cambridge_Bay",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 0, dst_offset: 0, name: "zzz" },
        Transition { occurs_at: Some(-1577923200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-880210800), utc_offset: -25200, dst_offset: 3600, name: "MWT" },
        Transition { occurs_at: Some(-769395600), utc_offset: -25200, dst_offset: 3600, name: "MPT" },
        Transition { occurs_at: Some(-765388800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-147891600), utc_offset: -25200, dst_offset: 7200, name: "MDDT" },
        Transition { occurs_at: Some(-131562000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(325674000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(341395200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(357123600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(372844800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(388573200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(404899200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(420022800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(436348800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(452077200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(467798400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(483526800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(499248000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(514976400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(530697600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(544611600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(562147200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(576061200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(594201600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(607510800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(625651200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(638960400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(657100800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(671014800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(688550400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(702464400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(720000000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(733914000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(752054400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(765363600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(783504000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(796813200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(814953600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(828867600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(846403200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(860317200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(877852800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(891766800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(909302400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(923216400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(941356800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(954662400), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(972802800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(973400400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(986115600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1004256000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1018170000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1035705600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1049619600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1067155200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1081069200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1099209600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1112518800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1130659200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1143968400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1162108800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1173603600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1194163200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1205053200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1225612800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1236502800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1257062400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1268557200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1289116800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1300006800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1320566400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1331456400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1352016000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1362906000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1383465600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1394355600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1414915200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1425805200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1446364800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1457859600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1478419200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1489309200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1509868800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1520758800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1541318400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1552208400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1572768000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1583658000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1604217600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1615712400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1636272000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1647162000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1667721600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1678611600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1699171200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1710061200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1730620800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1741510800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1762070400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1772960400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1793520000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1805014800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1825574400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1836464400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1857024000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1867914000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1888473600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1899363600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1919923200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1930813200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1951372800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1962867600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(1983427200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(1994317200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2014876800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2025766800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2046326400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2057216400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2077776000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2088666000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2109225600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2120115600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2140675200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2152170000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2172729600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2183619600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2204179200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2215069200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2235628800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2246518800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2267078400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2277968400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2298528000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2309418000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2329977600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2341472400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2362032000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2372922000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2393481600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2404371600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2424931200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2435821200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2456380800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2467270800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2487830400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2499325200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2519884800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2530774800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2551334400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2562224400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2582784000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2593674000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2614233600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2625123600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2645683200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2656573200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2677132800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2688627600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2709187200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2720077200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2740636800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2751526800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2772086400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2782976400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2803536000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2814426000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2834985600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2846480400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2867040000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2877930000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2898489600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2909379600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2929939200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2940829200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2961388800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(2972278800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(2992838400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3003728400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3024288000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3035782800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3056342400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3067232400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3087792000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3098682000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3119241600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3130131600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3150691200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3161581200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3182140800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3193030800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3213590400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3225085200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3245644800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3256534800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3277094400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3287984400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3308544000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3319434000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3339993600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3350883600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3371443200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3382938000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3403497600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3414387600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3434947200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3445837200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3466396800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3477286800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3497846400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3508736400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3529296000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3540186000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3560745600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3572240400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3592800000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3603690000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3624249600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3635139600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3655699200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3666589200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3687148800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3698038800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3718598400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3730093200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3750652800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3761542800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3782102400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3792992400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3813552000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3824442000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3845001600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3855891600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3876451200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3887341200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3907900800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3919395600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3939955200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3950845200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(3971404800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(3982294800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4002854400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(4013744400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4034304000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(4045194000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4065753600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(4076643600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(4097203200), utc_offset: -25200, dst_offset: 0, name: "MST" },
    ],
};


