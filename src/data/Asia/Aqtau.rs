
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use std::borrow::Cow;
use datetime::zone::{StaticTimeZone, FixedTimespanSet, FixedTimespan};

pub static ZONE: StaticTimeZone<'static> = StaticTimeZone {
    name: "Asia/Aqtau",
    fixed_timespans: FixedTimespanSet {
        first: FixedTimespan {
            offset: 12064,  // UTC offset 12064, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("LMT"),
        },
        rest: &[
        (-1441164064, FixedTimespan {  // 1924-05-01T20:38:56.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FORT"),
        }),
        (-1247544000, FixedTimespan {  // 1930-06-20T20:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("FORT"),
        }),
        (-220942800, FixedTimespan {  // 1962-12-31T19:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (370724400, FixedTimespan {  // 1981-09-30T19:00:00.000 UTC
            offset: 21600,  // UTC offset 21600, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (386445600, FixedTimespan {  // 1982-03-31T18:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (402256800, FixedTimespan {  // 1982-09-30T18:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (417985200, FixedTimespan {  // 1983-03-31T19:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (433792800, FixedTimespan {  // 1983-09-30T18:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (449607600, FixedTimespan {  // 1984-03-31T19:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (465339600, FixedTimespan {  // 1984-09-29T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (481064400, FixedTimespan {  // 1985-03-30T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (496789200, FixedTimespan {  // 1985-09-28T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (512514000, FixedTimespan {  // 1986-03-29T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (528238800, FixedTimespan {  // 1986-09-27T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (543963600, FixedTimespan {  // 1987-03-28T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (559688400, FixedTimespan {  // 1987-09-26T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (575413200, FixedTimespan {  // 1988-03-26T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (591138000, FixedTimespan {  // 1988-09-24T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (606862800, FixedTimespan {  // 1989-03-25T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (622587600, FixedTimespan {  // 1989-09-23T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (638312400, FixedTimespan {  // 1990-03-24T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("SHEST"),
        }),
        (654642000, FixedTimespan {  // 1990-09-29T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("SHET"),
        }),
        (692823600, FixedTimespan {  // 1991-12-15T19:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (701805600, FixedTimespan {  // 1992-03-28T18:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (717526800, FixedTimespan {  // 1992-09-26T17:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (733266000, FixedTimespan {  // 1993-03-27T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (748990800, FixedTimespan {  // 1993-09-25T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (764715600, FixedTimespan {  // 1994-03-26T21:00:00.000 UTC
            offset: 21600,  // UTC offset 18000, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (780440400, FixedTimespan {  // 1994-09-24T21:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (796165200, FixedTimespan {  // 1995-03-25T21:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (811893600, FixedTimespan {  // 1995-09-23T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (828223200, FixedTimespan {  // 1996-03-30T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (846367200, FixedTimespan {  // 1996-10-26T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (859672800, FixedTimespan {  // 1997-03-29T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (877816800, FixedTimespan {  // 1997-10-25T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (891122400, FixedTimespan {  // 1998-03-28T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (909266400, FixedTimespan {  // 1998-10-24T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (922572000, FixedTimespan {  // 1999-03-27T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (941320800, FixedTimespan {  // 1999-10-30T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (954021600, FixedTimespan {  // 2000-03-25T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (972770400, FixedTimespan {  // 2000-10-28T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (985471200, FixedTimespan {  // 2001-03-24T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (1004220000, FixedTimespan {  // 2001-10-27T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (1017525600, FixedTimespan {  // 2002-03-30T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (1035669600, FixedTimespan {  // 2002-10-26T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (1048975200, FixedTimespan {  // 2003-03-29T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (1067119200, FixedTimespan {  // 2003-10-25T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (1080424800, FixedTimespan {  // 2004-03-27T22:00:00.000 UTC
            offset: 18000,  // UTC offset 14400, DST offset 3600
            is_dst: true,
            name:   Cow::Borrowed("AQTST"),
        }),
        (1099173600, FixedTimespan {  // 2004-10-30T22:00:00.000 UTC
            offset: 14400,  // UTC offset 14400, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
        (1110830400, FixedTimespan {  // 2005-03-14T20:00:00.000 UTC
            offset: 18000,  // UTC offset 18000, DST offset 0
            is_dst: false,
            name:   Cow::Borrowed("AQTT"),
        }),
    ]},
};


