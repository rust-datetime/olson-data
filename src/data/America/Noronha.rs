
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "America/Noronha",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: -6620,  // UTC offset -6620, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1767218980, FixedTimespan {  // 1914-01-01T01:50:20.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-1206961200, FixedTimespan {  // 1931-10-03T13:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-1191366000, FixedTimespan {  // 1932-04-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-1175378400, FixedTimespan {  // 1932-10-03T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-1159830000, FixedTimespan {  // 1933-04-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-633823200, FixedTimespan {  // 1949-12-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-622072800, FixedTimespan {  // 1950-04-16T02:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-602287200, FixedTimespan {  // 1950-12-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-591836400, FixedTimespan {  // 1951-04-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-570751200, FixedTimespan {  // 1951-12-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-560214000, FixedTimespan {  // 1952-04-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-539128800, FixedTimespan {  // 1952-12-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-531356400, FixedTimespan {  // 1953-03-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-191368800, FixedTimespan {  // 1963-12-09T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-184201200, FixedTimespan {  // 1964-03-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-155167200, FixedTimespan {  // 1965-01-31T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-150073200, FixedTimespan {  // 1965-03-31T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-128901600, FixedTimespan {  // 1965-12-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-121129200, FixedTimespan {  // 1966-03-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-99957600, FixedTimespan {  // 1966-11-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-89593200, FixedTimespan {  // 1967-03-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (-68421600, FixedTimespan {  // 1967-11-01T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (-57970800, FixedTimespan {  // 1968-03-01T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (499744800, FixedTimespan {  // 1985-11-02T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (511232400, FixedTimespan {  // 1986-03-15T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (530589600, FixedTimespan {  // 1986-10-25T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (540262800, FixedTimespan {  // 1987-02-14T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (562125600, FixedTimespan {  // 1987-10-25T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (571194000, FixedTimespan {  // 1988-02-07T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (592970400, FixedTimespan {  // 1988-10-16T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (602038800, FixedTimespan {  // 1989-01-29T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (624420000, FixedTimespan {  // 1989-10-15T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (634698000, FixedTimespan {  // 1990-02-11T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (938916000, FixedTimespan {  // 1999-10-03T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (951613200, FixedTimespan {  // 2000-02-27T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (970970400, FixedTimespan {  // 2000-10-08T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (971571600, FixedTimespan {  // 2000-10-15T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
        (1003024800, FixedTimespan {  // 2001-10-14T02:00:00.000 UTC
            offset: -3600,  // UTC offset -7200, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("FNST"),
        }),
        (1013907600, FixedTimespan {  // 2002-02-17T01:00:00.000 UTC
            offset: -7200,  // UTC offset -7200, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FNT"),
        }),
    ]},
};


