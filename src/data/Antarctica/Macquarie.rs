
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Antarctica/Macquarie",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("zzz"),
        },
        rest: &[
        (-2214259200, FixedTimespan {  // 1899-11-01T00:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-1680508800, FixedTimespan {  // 1916-09-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (-1665392400, FixedTimespan {  // 1917-03-24T15:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-1601719200, FixedTimespan {  // 1919-03-31T14:00:00.000 UTC
            offset: 0,  // UTC offset 0, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("zzz"),
        }),
        (-687052800, FixedTimespan {  // 1948-03-25T00:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-71136000, FixedTimespan {  // 1967-09-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (-55411200, FixedTimespan {  // 1968-03-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-37267200, FixedTimespan {  // 1968-10-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (-25776000, FixedTimespan {  // 1969-03-08T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (-5817600, FixedTimespan {  // 1969-10-25T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (5673600, FixedTimespan {  // 1970-03-07T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (25632000, FixedTimespan {  // 1970-10-24T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (37728000, FixedTimespan {  // 1971-03-13T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (57686400, FixedTimespan {  // 1971-10-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (67968000, FixedTimespan {  // 1972-02-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (89136000, FixedTimespan {  // 1972-10-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (100022400, FixedTimespan {  // 1973-03-03T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (120585600, FixedTimespan {  // 1973-10-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (131472000, FixedTimespan {  // 1974-03-02T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (152035200, FixedTimespan {  // 1974-10-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (162921600, FixedTimespan {  // 1975-03-01T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (183484800, FixedTimespan {  // 1975-10-25T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (194976000, FixedTimespan {  // 1976-03-06T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (215539200, FixedTimespan {  // 1976-10-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (226425600, FixedTimespan {  // 1977-03-05T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (246988800, FixedTimespan {  // 1977-10-29T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (257875200, FixedTimespan {  // 1978-03-04T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (278438400, FixedTimespan {  // 1978-10-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (289324800, FixedTimespan {  // 1979-03-03T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (309888000, FixedTimespan {  // 1979-10-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (320774400, FixedTimespan {  // 1980-03-01T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (341337600, FixedTimespan {  // 1980-10-25T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (352224000, FixedTimespan {  // 1981-02-28T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (372787200, FixedTimespan {  // 1981-10-24T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (386092800, FixedTimespan {  // 1982-03-27T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (404841600, FixedTimespan {  // 1982-10-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (417542400, FixedTimespan {  // 1983-03-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (436291200, FixedTimespan {  // 1983-10-29T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (447177600, FixedTimespan {  // 1984-03-03T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (467740800, FixedTimespan {  // 1984-10-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (478627200, FixedTimespan {  // 1985-03-02T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (499190400, FixedTimespan {  // 1985-10-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (510076800, FixedTimespan {  // 1986-03-01T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (530035200, FixedTimespan {  // 1986-10-18T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (542736000, FixedTimespan {  // 1987-03-14T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (562089600, FixedTimespan {  // 1987-10-24T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (574790400, FixedTimespan {  // 1988-03-19T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (594144000, FixedTimespan {  // 1988-10-29T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (606240000, FixedTimespan {  // 1989-03-18T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (625593600, FixedTimespan {  // 1989-10-28T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (637689600, FixedTimespan {  // 1990-03-17T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (657043200, FixedTimespan {  // 1990-10-27T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (670348800, FixedTimespan {  // 1991-03-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (686678400, FixedTimespan {  // 1991-10-05T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (701798400, FixedTimespan {  // 1992-03-28T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (718128000, FixedTimespan {  // 1992-10-03T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (733248000, FixedTimespan {  // 1993-03-27T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (749577600, FixedTimespan {  // 1993-10-02T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (764697600, FixedTimespan {  // 1994-03-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (781027200, FixedTimespan {  // 1994-10-01T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (796147200, FixedTimespan {  // 1995-03-25T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (812476800, FixedTimespan {  // 1995-09-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (828201600, FixedTimespan {  // 1996-03-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (844531200, FixedTimespan {  // 1996-10-05T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (859651200, FixedTimespan {  // 1997-03-29T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (875980800, FixedTimespan {  // 1997-10-04T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (891100800, FixedTimespan {  // 1998-03-28T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (907430400, FixedTimespan {  // 1998-10-03T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (922550400, FixedTimespan {  // 1999-03-27T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (938880000, FixedTimespan {  // 1999-10-02T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (954000000, FixedTimespan {  // 2000-03-25T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (967305600, FixedTimespan {  // 2000-08-26T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (985449600, FixedTimespan {  // 2001-03-24T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1002384000, FixedTimespan {  // 2001-10-06T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1017504000, FixedTimespan {  // 2002-03-30T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1033833600, FixedTimespan {  // 2002-10-05T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1048953600, FixedTimespan {  // 2003-03-29T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1065283200, FixedTimespan {  // 2003-10-04T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1080403200, FixedTimespan {  // 2004-03-27T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1096732800, FixedTimespan {  // 2004-10-02T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1111852800, FixedTimespan {  // 2005-03-26T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1128182400, FixedTimespan {  // 2005-10-01T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1143907200, FixedTimespan {  // 2006-04-01T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1159632000, FixedTimespan {  // 2006-09-30T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1174752000, FixedTimespan {  // 2007-03-24T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1191686400, FixedTimespan {  // 2007-10-06T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1207411200, FixedTimespan {  // 2008-04-05T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1223136000, FixedTimespan {  // 2008-10-04T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1238860800, FixedTimespan {  // 2009-04-04T16:00:00.000 UTC
            offset: 36000,  // UTC offset 36000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AEST"),
        }),
        (1254585600, FixedTimespan {  // 2009-10-03T16:00:00.000 UTC
            offset: 39600,  // UTC offset 36000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AEDT"),
        }),
        (1270310400, FixedTimespan {  // 2010-04-03T16:00:00.000 UTC
            offset: 39600,  // UTC offset 39600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("MIST"),
        }),
    ]},
};


