
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "Asia/Damascus",
    transitions: &[
        Transition { occurs_at: None, utc_offset: 8712, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1577931912), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1568592000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1554080400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1537142400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1522630800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1505692800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1491181200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-1474243200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-1459126800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-242265600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-228877200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-210556800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-197427600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-178934400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-165718800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-147398400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-134269200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-116467200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-102646800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-84326400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-71110800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-52704000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-39488400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(-21168000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(-7952400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(10368000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(23583600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(41904000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(55119600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(73526400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(86742000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(105062400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(118278000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(136598400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(149814000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(168134400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(181350000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(199756800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(212972400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(231292800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(241916400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(262828800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(273452400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(418694400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(433810800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(450316800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(465433200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(508896000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(529196400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(541555200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(562633200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(574387200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(594255600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(607305600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(623199600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(638928000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(654649200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(670456800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(686264400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(702684000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(717886800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(733096800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(748904400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(765151200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(780958800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(796687200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(812494800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(828309600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(844117200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(859759200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(875653200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(891208800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(907189200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(922917600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(938725200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(954540000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(970347600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(986076000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1001883600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1017612000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1033419600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1049148000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1064955600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1080770400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1096578000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1112306400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1128114000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1143842400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1158872400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1175205600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1193950800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1207260000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1225486800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1238104800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1256850000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1270159200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1288299600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1301608800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1319749200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1333058400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1351198800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1364508000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1382648400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1395957600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1414702800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1427407200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1446152400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1458856800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1477602000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1490911200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1509051600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1522360800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1540501200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1553810400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1571950800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1585260000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1604005200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1616709600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1635454800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1648159200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1666904400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1680213600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1698354000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1711663200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1729803600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1743112800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1761858000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1774562400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1793307600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1806012000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1824757200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1838066400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1856206800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1869516000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1887656400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1900965600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1919106000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1932415200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1951160400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1963864800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(1982610000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(1995314400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2014059600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2027368800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2045509200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2058818400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2076958800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2090268000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2109013200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2121717600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2140462800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2153167200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2171912400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2184616800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2203362000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2216671200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2234811600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2248120800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2266261200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2279570400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2298315600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2311020000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2329765200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2342469600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2361214800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2374524000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2392664400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2405973600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2424114000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2437423200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2455563600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2468872800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2487618000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2500322400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2519067600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2531772000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2550517200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2563826400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2581966800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2595276000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2613416400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2626725600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2645470800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2658175200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2676920400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2689624800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2708370000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2721679200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2739819600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2753128800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2771269200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2784578400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2802718800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2816028000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2834773200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2847477600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2866222800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2878927200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2897672400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2910981600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2929122000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2942431200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2960571600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(2973880800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(2992626000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3005330400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3024075600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3036780000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3055525200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3068229600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3086974800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3100284000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3118424400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3131733600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3149874000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3163183200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3181928400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3194632800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3213378000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3226082400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3244827600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3258136800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3276277200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3289586400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3307726800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3321036000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3339176400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3352485600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3371230800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3383935200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3402680400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3415384800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3434130000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3447439200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3465579600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3478888800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3497029200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3510338400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3529083600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3541788000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3560533200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3573237600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3591982800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3605292000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3623432400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3636741600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3654882000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3668191200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3686331600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3699640800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3718386000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3731090400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3749835600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3762540000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3781285200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3794594400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3812734800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3826044000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3844184400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3857493600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3876238800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3888943200), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3907688400), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3920392800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3939138000), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3951842400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(3970587600), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(3983896800), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4002037200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(4015346400), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4033486800), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(4046796000), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4065541200), utc_offset: 7200, dst_offset: 0, name: "EET" },
        Transition { occurs_at: Some(4078245600), utc_offset: 7200, dst_offset: 3600, name: "EEST" },
        Transition { occurs_at: Some(4096990800), utc_offset: 7200, dst_offset: 0, name: "EET" },
    ],
};

