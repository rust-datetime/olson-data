
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;

pub const ZONE: Zone<'static> = Zone {
    name: "America/Regina",
    transitions: &[
        Transition { occurs_at: None, utc_offset: -18084, dst_offset: 0, name: "LMT" },
        Transition { occurs_at: Some(-2030209116), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1632063600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1615132800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1251651600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1238349600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1220202000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1206900000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1188752400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1175450400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1156698000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1144000800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1125248400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1111946400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1032714000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-1016992800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-1001264400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-986148000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-969814800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-954093600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-937760400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-922039200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-906310800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-890589600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-880210800), utc_offset: -25200, dst_offset: 3600, name: "MWT" },
        Transition { occurs_at: Some(-769395600), utc_offset: -25200, dst_offset: 3600, name: "MPT" },
        Transition { occurs_at: Some(-765388800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-748450800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-732729600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-715791600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-702489600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-684342000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-671040000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-652892400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-639590400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-620838000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-608140800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-589388400), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-576086400), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-557938800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-544636800), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-526489200), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-513187200), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-495039600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-481737600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-463590000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-450288000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-431535600), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-418233600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-400086000), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-386784000), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-337186800), utc_offset: -25200, dst_offset: 3600, name: "MDT" },
        Transition { occurs_at: Some(-321465600), utc_offset: -25200, dst_offset: 0, name: "MST" },
        Transition { occurs_at: Some(-305737200), utc_offset: -21600, dst_offset: 0, name: "CST" },
    ],
};


