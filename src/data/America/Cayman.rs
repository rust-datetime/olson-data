
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Cayman",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -16468,  // UTC offset -16468, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524505132, FixedTimespan {  // 1890-01-01T04:34:28.000 UTC
            offset: -17569,  // UTC offset -17569, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("KMT"),
        }),
        (-1827688031, FixedTimespan {  // 1912-02-01T04:52:49.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1457852400, FixedTimespan {  // 2016-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1478412000, FixedTimespan {  // 2016-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1489302000, FixedTimespan {  // 2017-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1509861600, FixedTimespan {  // 2017-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1520751600, FixedTimespan {  // 2018-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1541311200, FixedTimespan {  // 2018-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1552201200, FixedTimespan {  // 2019-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1572760800, FixedTimespan {  // 2019-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1583650800, FixedTimespan {  // 2020-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1604210400, FixedTimespan {  // 2020-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1615705200, FixedTimespan {  // 2021-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1636264800, FixedTimespan {  // 2021-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1647154800, FixedTimespan {  // 2022-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1667714400, FixedTimespan {  // 2022-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1678604400, FixedTimespan {  // 2023-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1699164000, FixedTimespan {  // 2023-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1710054000, FixedTimespan {  // 2024-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1730613600, FixedTimespan {  // 2024-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1741503600, FixedTimespan {  // 2025-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1762063200, FixedTimespan {  // 2025-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1772953200, FixedTimespan {  // 2026-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1793512800, FixedTimespan {  // 2026-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1805007600, FixedTimespan {  // 2027-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1825567200, FixedTimespan {  // 2027-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1836457200, FixedTimespan {  // 2028-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1857016800, FixedTimespan {  // 2028-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1867906800, FixedTimespan {  // 2029-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1888466400, FixedTimespan {  // 2029-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1899356400, FixedTimespan {  // 2030-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1919916000, FixedTimespan {  // 2030-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1930806000, FixedTimespan {  // 2031-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1951365600, FixedTimespan {  // 2031-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1962860400, FixedTimespan {  // 2032-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (1983420000, FixedTimespan {  // 2032-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (1994310000, FixedTimespan {  // 2033-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2014869600, FixedTimespan {  // 2033-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2025759600, FixedTimespan {  // 2034-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2046319200, FixedTimespan {  // 2034-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2057209200, FixedTimespan {  // 2035-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2077768800, FixedTimespan {  // 2035-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2088658800, FixedTimespan {  // 2036-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2109218400, FixedTimespan {  // 2036-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2120108400, FixedTimespan {  // 2037-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2140668000, FixedTimespan {  // 2037-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2152162800, FixedTimespan {  // 2038-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2172722400, FixedTimespan {  // 2038-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2183612400, FixedTimespan {  // 2039-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2204172000, FixedTimespan {  // 2039-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2215062000, FixedTimespan {  // 2040-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2235621600, FixedTimespan {  // 2040-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2246511600, FixedTimespan {  // 2041-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2267071200, FixedTimespan {  // 2041-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2277961200, FixedTimespan {  // 2042-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2298520800, FixedTimespan {  // 2042-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2309410800, FixedTimespan {  // 2043-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2329970400, FixedTimespan {  // 2043-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2341465200, FixedTimespan {  // 2044-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2362024800, FixedTimespan {  // 2044-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2372914800, FixedTimespan {  // 2045-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2393474400, FixedTimespan {  // 2045-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2404364400, FixedTimespan {  // 2046-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2424924000, FixedTimespan {  // 2046-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2435814000, FixedTimespan {  // 2047-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2456373600, FixedTimespan {  // 2047-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2467263600, FixedTimespan {  // 2048-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2487823200, FixedTimespan {  // 2048-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2499318000, FixedTimespan {  // 2049-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2519877600, FixedTimespan {  // 2049-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2530767600, FixedTimespan {  // 2050-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2551327200, FixedTimespan {  // 2050-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2562217200, FixedTimespan {  // 2051-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2582776800, FixedTimespan {  // 2051-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2593666800, FixedTimespan {  // 2052-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2614226400, FixedTimespan {  // 2052-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2625116400, FixedTimespan {  // 2053-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2645676000, FixedTimespan {  // 2053-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2656566000, FixedTimespan {  // 2054-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2677125600, FixedTimespan {  // 2054-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2688620400, FixedTimespan {  // 2055-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2709180000, FixedTimespan {  // 2055-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2720070000, FixedTimespan {  // 2056-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2740629600, FixedTimespan {  // 2056-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2751519600, FixedTimespan {  // 2057-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2772079200, FixedTimespan {  // 2057-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2782969200, FixedTimespan {  // 2058-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2803528800, FixedTimespan {  // 2058-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2814418800, FixedTimespan {  // 2059-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2834978400, FixedTimespan {  // 2059-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2846473200, FixedTimespan {  // 2060-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2867032800, FixedTimespan {  // 2060-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2877922800, FixedTimespan {  // 2061-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2898482400, FixedTimespan {  // 2061-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2909372400, FixedTimespan {  // 2062-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2929932000, FixedTimespan {  // 2062-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2940822000, FixedTimespan {  // 2063-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2961381600, FixedTimespan {  // 2063-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (2972271600, FixedTimespan {  // 2064-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (2992831200, FixedTimespan {  // 2064-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3003721200, FixedTimespan {  // 2065-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3024280800, FixedTimespan {  // 2065-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3035775600, FixedTimespan {  // 2066-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3056335200, FixedTimespan {  // 2066-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3067225200, FixedTimespan {  // 2067-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3087784800, FixedTimespan {  // 2067-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3098674800, FixedTimespan {  // 2068-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3119234400, FixedTimespan {  // 2068-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3130124400, FixedTimespan {  // 2069-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3150684000, FixedTimespan {  // 2069-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3161574000, FixedTimespan {  // 2070-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3182133600, FixedTimespan {  // 2070-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3193023600, FixedTimespan {  // 2071-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3213583200, FixedTimespan {  // 2071-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3225078000, FixedTimespan {  // 2072-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3245637600, FixedTimespan {  // 2072-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3256527600, FixedTimespan {  // 2073-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3277087200, FixedTimespan {  // 2073-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3287977200, FixedTimespan {  // 2074-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3308536800, FixedTimespan {  // 2074-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3319426800, FixedTimespan {  // 2075-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3339986400, FixedTimespan {  // 2075-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3350876400, FixedTimespan {  // 2076-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3371436000, FixedTimespan {  // 2076-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3382930800, FixedTimespan {  // 2077-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3403490400, FixedTimespan {  // 2077-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3414380400, FixedTimespan {  // 2078-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3434940000, FixedTimespan {  // 2078-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3445830000, FixedTimespan {  // 2079-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3466389600, FixedTimespan {  // 2079-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3477279600, FixedTimespan {  // 2080-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3497839200, FixedTimespan {  // 2080-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3508729200, FixedTimespan {  // 2081-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3529288800, FixedTimespan {  // 2081-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3540178800, FixedTimespan {  // 2082-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3560738400, FixedTimespan {  // 2082-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3572233200, FixedTimespan {  // 2083-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3592792800, FixedTimespan {  // 2083-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3603682800, FixedTimespan {  // 2084-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3624242400, FixedTimespan {  // 2084-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3635132400, FixedTimespan {  // 2085-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3655692000, FixedTimespan {  // 2085-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3666582000, FixedTimespan {  // 2086-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3687141600, FixedTimespan {  // 2086-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3698031600, FixedTimespan {  // 2087-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3718591200, FixedTimespan {  // 2087-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3730086000, FixedTimespan {  // 2088-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3750645600, FixedTimespan {  // 2088-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3761535600, FixedTimespan {  // 2089-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3782095200, FixedTimespan {  // 2089-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3792985200, FixedTimespan {  // 2090-03-12T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3813544800, FixedTimespan {  // 2090-11-05T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3824434800, FixedTimespan {  // 2091-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3844994400, FixedTimespan {  // 2091-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3855884400, FixedTimespan {  // 2092-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3876444000, FixedTimespan {  // 2092-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3887334000, FixedTimespan {  // 2093-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3907893600, FixedTimespan {  // 2093-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3919388400, FixedTimespan {  // 2094-03-14T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3939948000, FixedTimespan {  // 2094-11-07T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3950838000, FixedTimespan {  // 2095-03-13T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (3971397600, FixedTimespan {  // 2095-11-06T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (3982287600, FixedTimespan {  // 2096-03-11T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (4002847200, FixedTimespan {  // 2096-11-04T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (4013737200, FixedTimespan {  // 2097-03-10T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (4034296800, FixedTimespan {  // 2097-11-03T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (4045186800, FixedTimespan {  // 2098-03-09T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (4065746400, FixedTimespan {  // 2098-11-02T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
        (4076636400, FixedTimespan {  // 2099-03-08T07:00:00.000 UTC
            offset: -14400,  // UTC offset -18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EDT"),
        }),
        (4097196000, FixedTimespan {  // 2099-11-01T06:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EST"),
        }),
    ]},
};


