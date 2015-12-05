
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Pacific/Easter",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -24152,  // UTC offset -24152, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-2524497448, FixedTimespan {  // 1890-01-01T06:42:32.000 UTC
            offset: -24152,  // UTC offset -24152, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EMT"),
        }),
        (-1178126248, FixedTimespan {  // 1932-09-01T06:42:32.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (-36619200, FixedTimespan {  // 1968-11-03T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (-23922000, FixedTimespan {  // 1969-03-30T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (-3355200, FixedTimespan {  // 1969-11-23T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (7527600, FixedTimespan {  // 1970-03-29T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (24465600, FixedTimespan {  // 1970-10-11T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (37767600, FixedTimespan {  // 1971-03-14T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (55915200, FixedTimespan {  // 1971-10-10T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (69217200, FixedTimespan {  // 1972-03-12T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (87969600, FixedTimespan {  // 1972-10-15T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (100666800, FixedTimespan {  // 1973-03-11T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (118209600, FixedTimespan {  // 1973-09-30T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (132116400, FixedTimespan {  // 1974-03-10T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (150868800, FixedTimespan {  // 1974-10-13T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (163566000, FixedTimespan {  // 1975-03-09T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (182318400, FixedTimespan {  // 1975-10-12T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (195620400, FixedTimespan {  // 1976-03-14T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (213768000, FixedTimespan {  // 1976-10-10T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (227070000, FixedTimespan {  // 1977-03-13T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (245217600, FixedTimespan {  // 1977-10-09T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (258519600, FixedTimespan {  // 1978-03-12T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (277272000, FixedTimespan {  // 1978-10-15T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (289969200, FixedTimespan {  // 1979-03-11T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (308721600, FixedTimespan {  // 1979-10-14T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (321418800, FixedTimespan {  // 1980-03-09T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (340171200, FixedTimespan {  // 1980-10-12T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (353473200, FixedTimespan {  // 1981-03-15T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (371620800, FixedTimespan {  // 1981-10-11T04:00:00.000 UTC
            offset: -21600,  // UTC offset -25200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (384922800, FixedTimespan {  // 1982-03-14T03:00:00.000 UTC
            offset: -25200,  // UTC offset -25200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (384948000, FixedTimespan {  // 1982-03-14T10:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (403070400, FixedTimespan {  // 1982-10-10T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (416372400, FixedTimespan {  // 1983-03-13T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (434520000, FixedTimespan {  // 1983-10-09T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (447822000, FixedTimespan {  // 1984-03-11T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (466574400, FixedTimespan {  // 1984-10-14T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (479271600, FixedTimespan {  // 1985-03-10T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (498024000, FixedTimespan {  // 1985-10-13T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (510721200, FixedTimespan {  // 1986-03-09T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (529473600, FixedTimespan {  // 1986-10-12T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (545194800, FixedTimespan {  // 1987-04-12T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (560923200, FixedTimespan {  // 1987-10-11T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (574225200, FixedTimespan {  // 1988-03-13T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (592372800, FixedTimespan {  // 1988-10-09T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (605674800, FixedTimespan {  // 1989-03-12T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (624427200, FixedTimespan {  // 1989-10-15T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (637124400, FixedTimespan {  // 1990-03-11T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (653457600, FixedTimespan {  // 1990-09-16T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (668574000, FixedTimespan {  // 1991-03-10T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (687326400, FixedTimespan {  // 1991-10-13T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (700628400, FixedTimespan {  // 1992-03-15T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (718776000, FixedTimespan {  // 1992-10-11T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (732078000, FixedTimespan {  // 1993-03-14T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (750225600, FixedTimespan {  // 1993-10-10T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (763527600, FixedTimespan {  // 1994-03-13T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (781675200, FixedTimespan {  // 1994-10-09T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (794977200, FixedTimespan {  // 1995-03-12T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (813729600, FixedTimespan {  // 1995-10-15T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (826426800, FixedTimespan {  // 1996-03-10T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (845179200, FixedTimespan {  // 1996-10-13T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (859690800, FixedTimespan {  // 1997-03-30T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (876628800, FixedTimespan {  // 1997-10-12T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (889930800, FixedTimespan {  // 1998-03-15T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (906868800, FixedTimespan {  // 1998-09-27T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (923194800, FixedTimespan {  // 1999-04-04T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (939528000, FixedTimespan {  // 1999-10-10T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (952830000, FixedTimespan {  // 2000-03-12T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (971582400, FixedTimespan {  // 2000-10-15T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (984279600, FixedTimespan {  // 2001-03-11T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1003032000, FixedTimespan {  // 2001-10-14T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1015729200, FixedTimespan {  // 2002-03-10T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1034481600, FixedTimespan {  // 2002-10-13T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1047178800, FixedTimespan {  // 2003-03-09T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1065931200, FixedTimespan {  // 2003-10-12T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1079233200, FixedTimespan {  // 2004-03-14T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1097380800, FixedTimespan {  // 2004-10-10T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1110682800, FixedTimespan {  // 2005-03-13T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1128830400, FixedTimespan {  // 2005-10-09T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1142132400, FixedTimespan {  // 2006-03-12T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1160884800, FixedTimespan {  // 2006-10-15T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1173582000, FixedTimespan {  // 2007-03-11T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1192334400, FixedTimespan {  // 2007-10-14T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1206846000, FixedTimespan {  // 2008-03-30T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1223784000, FixedTimespan {  // 2008-10-12T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1237086000, FixedTimespan {  // 2009-03-15T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1255233600, FixedTimespan {  // 2009-10-11T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1270350000, FixedTimespan {  // 2010-04-04T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1286683200, FixedTimespan {  // 2010-10-10T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1304823600, FixedTimespan {  // 2011-05-08T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1313899200, FixedTimespan {  // 2011-08-21T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1335668400, FixedTimespan {  // 2012-04-29T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1346558400, FixedTimespan {  // 2012-09-02T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1367118000, FixedTimespan {  // 2013-04-28T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1378612800, FixedTimespan {  // 2013-09-08T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1398567600, FixedTimespan {  // 2014-04-27T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1410062400, FixedTimespan {  // 2014-09-07T04:00:00.000 UTC
            offset: -18000,  // UTC offset -21600, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("EASST"),
        }),
        (1430017200, FixedTimespan {  // 2015-04-26T03:00:00.000 UTC
            offset: -21600,  // UTC offset -21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
        (1430038800, FixedTimespan {  // 2015-04-26T09:00:00.000 UTC
            offset: -18000,  // UTC offset -18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("EAST"),
        }),
    ]},
};


