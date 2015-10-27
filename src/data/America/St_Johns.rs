
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/St_Johns",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -8948, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-1664134252), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1650141052), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1632079852), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1615149052), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1598653852), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1590103852), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1567290652), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1551569452), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1535841052), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1520119852), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1503786652), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1488670252), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1472337052), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1457220652), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1440887452), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1425771052), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1409437852), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1394321452), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1377988252), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1362267052), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1346538652), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1330817452), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1314484252), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1299367852), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1283034652), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1267918252), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1251585052), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1236468652), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1220135452), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1205019052), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1188685852), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1172964652), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1156631452), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1141515052), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1125181852), utc_offset: -8948, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1110065452), utc_offset: -8948, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1096925452), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1093732200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1078615800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1061674200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1048977000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-1030224600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-1017527400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-998775000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-986077800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-966720600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-954628200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-935271000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-922573800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-903821400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-891124200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-872371800), utc_offset: -9000, dst_offset: 3600, name: "NWT" },
        Transition { occurs_at: Some(-765405000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-746047800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-733350600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-714598200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-701901000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-683148600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-670451400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-651699000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-639001800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-619644600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-606947400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-589404600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-576102600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-557955000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-544653000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-526505400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-513203400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-495055800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-481753800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-463606200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-450304200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-431551800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-418249800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-400102200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-386800200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-368652600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-355350600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-337203000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-323901000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-305753400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-289427400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-273699000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-257977800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-242249400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-226528200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-210799800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-195078600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-179350200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-163629000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-147900600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-131574600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-116451000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-100125000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-84396600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-68675400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-52947000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-37225800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(-21497400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(-5776200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(9952200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(25673400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(41401800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(57727800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(73456200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(89177400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(104905800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(120627000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(136355400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(152076600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(167805000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(183526200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(199254600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(215580600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(230704200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(247030200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(262758600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(278479800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(294208200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(309929400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(325657800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(341379000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(357107400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(372828600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(388557000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(404883000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(420006600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(436332600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(452061000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(467782200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(483510600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(499231800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(514960200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(530681400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(544588260), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(562123860), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(576037860), utc_offset: -9000, dst_offset: 7200, name: "NDDT" },
        Transition { occurs_at: Some(594174660), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(607487460), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(625627860), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(638937060), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(657077460), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(670991460), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(688527060), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(702441060), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(719976660), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(733890660), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(752031060), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(765340260), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(783480660), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(796789860), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(814930260), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(828844260), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(846379860), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(860293860), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(877829460), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(891743460), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(909279060), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(923193060), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(941333460), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(954642660), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(972783060), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(986092260), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1004232660), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1018146660), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1035682260), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1049596260), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1067131860), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1081045860), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1099186260), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1112495460), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1130635860), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1143945060), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1162085460), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1173580260), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1194139860), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1205029860), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1225589460), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1236479460), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1257039060), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1268533860), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1289093460), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1299983460), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1320550200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1331440200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1351999800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1362889800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1383449400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1394339400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1414899000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1425789000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1446348600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1457843400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1478403000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1489293000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1509852600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1520742600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1541302200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1552192200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1572751800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1583641800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1604201400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1615696200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1636255800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1647145800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1667705400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1678595400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1699155000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1710045000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1730604600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1741494600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1762054200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1772944200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1793503800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1804998600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1825558200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1836448200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1857007800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1867897800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1888457400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1899347400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1919907000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1930797000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1951356600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1962851400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(1983411000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(1994301000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2014860600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2025750600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2046310200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2057200200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2077759800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2088649800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2109209400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2120099400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2140659000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2152153800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2172713400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2183603400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2204163000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2215053000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2235612600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2246502600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2267062200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2277952200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2298511800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2309401800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2329961400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2341456200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2362015800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2372905800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2393465400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2404355400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2424915000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2435805000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2456364600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2467254600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2487814200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2499309000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2519868600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2530758600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2551318200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2562208200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2582767800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2593657800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2614217400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2625107400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2645667000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2656557000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2677116600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2688611400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2709171000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2720061000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2740620600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2751510600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2772070200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2782960200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2803519800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2814409800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2834969400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2846464200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2867023800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2877913800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2898473400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2909363400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2929923000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2940813000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2961372600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(2972262600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(2992822200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3003712200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3024271800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3035766600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3056326200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3067216200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3087775800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3098665800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3119225400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3130115400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3150675000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3161565000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3182124600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3193014600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3213574200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3225069000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3245628600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3256518600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3277078200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3287968200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3308527800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3319417800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3339977400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3350867400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3371427000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3382921800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3403481400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3414371400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3434931000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3445821000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3466380600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3477270600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3497830200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3508720200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3529279800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3540169800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3560729400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3572224200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3592783800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3603673800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3624233400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3635123400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3655683000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3666573000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3687132600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3698022600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3718582200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3730077000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3750636600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3761526600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3782086200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3792976200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3813535800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3824425800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3844985400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3855875400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3876435000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3887325000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3907884600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3919379400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3939939000), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3950829000), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(3971388600), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(3982278600), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(4002838200), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(4013728200), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(4034287800), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(4045177800), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(4065737400), utc_offset: -9000, dst_offset: 0, name: "NST" },
        Transition { occurs_at: Some(4076627400), utc_offset: -9000, dst_offset: 3600, name: "NDT" },
        Transition { occurs_at: Some(4097187000), utc_offset: -9000, dst_offset: 0, name: "NST" },
    ],
};

