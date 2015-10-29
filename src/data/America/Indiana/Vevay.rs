
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Indiana/Vevay",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -15584, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2717652032), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-1633276800), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-1615136400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-1601827200), utc_offset: -21600, dst_offset: 3600, name: "CDT" },
        Transition { occurs_at: Some(-1583686800), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-880214400), utc_offset: -21600, dst_offset: 3600, name: "CWT" },
        Transition { occurs_at: Some(-769395600), utc_offset: -21600, dst_offset: 3600, name: "CPT" },
        Transition { occurs_at: Some(-765392400), utc_offset: -21600, dst_offset: 0, name: "CST" },
        Transition { occurs_at: Some(-495043200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(-21488400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(-5767200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(9961200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(25682400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(41410800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(57736800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(73465200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(89186400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1143961200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1162101600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1173596400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1194156000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1205046000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1225605600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1236495600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1257055200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1268550000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1289109600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1299999600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1320559200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1331449200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1352008800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1362898800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1383458400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1394348400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1414908000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1425798000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1446357600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1457852400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1478412000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1489302000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1509861600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1520751600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1541311200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1552201200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1572760800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1583650800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1604210400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1615705200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1636264800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1647154800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1667714400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1678604400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1699164000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1710054000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1730613600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1741503600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1762063200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1772953200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1793512800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1805007600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1825567200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1836457200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1857016800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1867906800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1888466400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1899356400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1919916000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1930806000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1951365600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1962860400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(1983420000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(1994310000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2014869600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2025759600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2046319200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2057209200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2077768800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2088658800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2109218400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2120108400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2140668000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2152162800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2172722400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2183612400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2204172000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2215062000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2235621600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2246511600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2267071200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2277961200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2298520800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2309410800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2329970400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2341465200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2362024800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2372914800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2393474400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2404364400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2424924000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2435814000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2456373600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2467263600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2487823200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2499318000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2519877600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2530767600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2551327200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2562217200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2582776800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2593666800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2614226400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2625116400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2645676000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2656566000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2677125600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2688620400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2709180000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2720070000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2740629600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2751519600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2772079200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2782969200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2803528800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2814418800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2834978400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2846473200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2867032800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2877922800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2898482400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2909372400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2929932000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2940822000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2961381600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(2972271600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(2992831200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3003721200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3024280800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3035775600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3056335200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3067225200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3087784800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3098674800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3119234400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3130124400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3150684000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3161574000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3182133600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3193023600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3213583200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3225078000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3245637600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3256527600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3277087200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3287977200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3308536800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3319426800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3339986400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3350876400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3371436000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3382930800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3403490400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3414380400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3434940000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3445830000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3466389600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3477279600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3497839200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3508729200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3529288800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3540178800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3560738400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3572233200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3592792800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3603682800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3624242400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3635132400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3655692000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3666582000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3687141600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3698031600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3718591200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3730086000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3750645600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3761535600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3782095200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3792985200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3813544800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3824434800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3844994400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3855884400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3876444000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3887334000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3907893600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3919388400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3939948000), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3950838000), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(3971397600), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(3982287600), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(4002847200), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(4013737200), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(4034296800), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(4045186800), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(4065746400), utc_offset: -18000, dst_offset: 0, name: "EST" },
        Transition { occurs_at: Some(4076636400), utc_offset: -18000, dst_offset: 3600, name: "EDT" },
        Transition { occurs_at: Some(4097196000), utc_offset: -18000, dst_offset: 0, name: "EST" },
    ],
};


