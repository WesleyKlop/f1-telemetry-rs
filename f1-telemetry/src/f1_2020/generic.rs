use crate::packet::generic::{
    Flag, Nationality, ResultStatus, Team, TyreCompound, TyreCompoundVisual,
};
use crate::packet::UnpackError;

pub(crate) fn unpack_flag(value: i8) -> Result<Flag, UnpackError> {
    match value {
        0 => Ok(Flag::None),
        1 => Ok(Flag::Green),
        2 => Ok(Flag::Blue),
        3 => Ok(Flag::Yellow),
        4 => Ok(Flag::Red),
        -1 => Ok(Flag::Invalid),
        _ => Err(UnpackError(format!("Invalid Flag value: {}", value))),
    }
}

pub(crate) fn unpack_nationality(value: u8) -> Result<Nationality, UnpackError> {
    match value {
        1 => Ok(Nationality::American),
        2 => Ok(Nationality::Argentinean),
        3 => Ok(Nationality::Australian),
        4 => Ok(Nationality::Austrian),
        5 => Ok(Nationality::Azerbaijani),
        6 => Ok(Nationality::Bahraini),
        7 => Ok(Nationality::Belgian),
        8 => Ok(Nationality::Bolivian),
        9 => Ok(Nationality::Brazilian),
        10 => Ok(Nationality::British),
        11 => Ok(Nationality::Bulgarian),
        12 => Ok(Nationality::Cameroonian),
        13 => Ok(Nationality::Canadian),
        14 => Ok(Nationality::Chilean),
        15 => Ok(Nationality::Chinese),
        16 => Ok(Nationality::Colombian),
        17 => Ok(Nationality::CostaRican),
        18 => Ok(Nationality::Croatian),
        19 => Ok(Nationality::Cypriot),
        20 => Ok(Nationality::Czech),
        21 => Ok(Nationality::Danish),
        22 => Ok(Nationality::Dutch),
        23 => Ok(Nationality::Ecuadorian),
        24 => Ok(Nationality::English),
        25 => Ok(Nationality::Emirian),
        26 => Ok(Nationality::Estonian),
        27 => Ok(Nationality::Finnish),
        28 => Ok(Nationality::French),
        29 => Ok(Nationality::German),
        30 => Ok(Nationality::Ghanaian),
        31 => Ok(Nationality::Greek),
        32 => Ok(Nationality::Guatemalan),
        33 => Ok(Nationality::Honduran),
        34 => Ok(Nationality::HongKonger),
        35 => Ok(Nationality::Hungarian),
        36 => Ok(Nationality::Icelander),
        37 => Ok(Nationality::Indian),
        38 => Ok(Nationality::Indonesian),
        39 => Ok(Nationality::Irish),
        40 => Ok(Nationality::Israeli),
        41 => Ok(Nationality::Italian),
        42 => Ok(Nationality::Jamaican),
        43 => Ok(Nationality::Japanese),
        44 => Ok(Nationality::Jordanian),
        45 => Ok(Nationality::Kuwaiti),
        46 => Ok(Nationality::Latvian),
        47 => Ok(Nationality::Lebanese),
        48 => Ok(Nationality::Lithuanian),
        49 => Ok(Nationality::Luxembourger),
        50 => Ok(Nationality::Malaysian),
        51 => Ok(Nationality::Maltese),
        52 => Ok(Nationality::Mexican),
        53 => Ok(Nationality::Monegasque),
        54 => Ok(Nationality::NewZealander),
        55 => Ok(Nationality::Nicaraguan),
        56 => Ok(Nationality::NorthKorean),
        57 => Ok(Nationality::NorthernIrish),
        58 => Ok(Nationality::Norwegian),
        59 => Ok(Nationality::Omani),
        60 => Ok(Nationality::Pakistani),
        61 => Ok(Nationality::Panamanian),
        62 => Ok(Nationality::Paraguayan),
        63 => Ok(Nationality::Peruvian),
        64 => Ok(Nationality::Polish),
        65 => Ok(Nationality::Portuguese),
        66 => Ok(Nationality::Qatari),
        67 => Ok(Nationality::Romanian),
        68 => Ok(Nationality::Russian),
        69 => Ok(Nationality::Salvadoran),
        70 => Ok(Nationality::Saudi),
        71 => Ok(Nationality::Scottish),
        72 => Ok(Nationality::Serbian),
        73 => Ok(Nationality::Singaporean),
        74 => Ok(Nationality::Slovakian),
        75 => Ok(Nationality::Slovenian),
        76 => Ok(Nationality::SouthKorean),
        77 => Ok(Nationality::SouthAfrican),
        78 => Ok(Nationality::Spanish),
        79 => Ok(Nationality::Swedish),
        80 => Ok(Nationality::Swiss),
        81 => Ok(Nationality::Thai),
        82 => Ok(Nationality::Turkish),
        83 => Ok(Nationality::Uruguayan),
        84 => Ok(Nationality::Ukrainian),
        85 => Ok(Nationality::Venezuelan),
        86 => Ok(Nationality::Welsh),
        87 => Ok(Nationality::Barbadian),
        88 => Ok(Nationality::Vietnamese),
        0 | 255 => Ok(Nationality::Invalid),
        _ => Err(UnpackError(format!("Invalid Nationality value: {}", value))),
    }
}

pub(crate) fn unpack_result_status(value: u8) -> Result<ResultStatus, UnpackError> {
    match value {
        0 => Ok(ResultStatus::Invalid),
        1 => Ok(ResultStatus::Inactive),
        2 => Ok(ResultStatus::Active),
        3 => Ok(ResultStatus::Finished),
        4 => Ok(ResultStatus::Disqualified),
        5 => Ok(ResultStatus::NotClassified),
        6 => Ok(ResultStatus::Retired),
        7 => Ok(ResultStatus::Retired),
        _ => Err(UnpackError(format!(
            "Invalid ResultStatus value: {}",
            value
        ))),
    }
}

pub(crate) fn unpack_team(value: u8) -> Result<Team, UnpackError> {
    match value {
        0 => Ok(Team::Mercedes),
        1 => Ok(Team::Ferrari),
        2 => Ok(Team::RedBullRacing),
        3 => Ok(Team::Williams),
        4 => Ok(Team::RacingPoint),
        5 => Ok(Team::Renault),
        6 => Ok(Team::AlphaTauri),
        7 => Ok(Team::Haas),
        8 => Ok(Team::McLaren),
        9 => Ok(Team::AlfaRomeo),
        10 => Ok(Team::McLaren1988),
        11 => Ok(Team::McLaren1991),
        12 => Ok(Team::Williams1992),
        13 => Ok(Team::Ferrari1995),
        14 => Ok(Team::Williams1996),
        15 => Ok(Team::McLaren1998),
        16 => Ok(Team::Ferrari2002),
        17 => Ok(Team::Ferrari2004),
        18 => Ok(Team::Renault2006),
        19 => Ok(Team::Ferrari2007),
        20 => Ok(Team::McLaren2008),
        21 => Ok(Team::RedBull2010),
        22 => Ok(Team::Ferrari1976),
        23 => Ok(Team::ARTGrandPrix),
        24 => Ok(Team::CamposVexatecRacing),
        25 => Ok(Team::Carlin),
        26 => Ok(Team::CharouzRacingSystem),
        27 => Ok(Team::DAMS),
        28 => Ok(Team::RussianTime),
        29 => Ok(Team::MPMotorsport),
        30 => Ok(Team::Pertamina),
        31 => Ok(Team::McLaren1990),
        32 => Ok(Team::Trident),
        33 => Ok(Team::BWTArden),
        34 => Ok(Team::McLaren1976),
        35 => Ok(Team::Lotus1972),
        36 => Ok(Team::Ferrari1979),
        37 => Ok(Team::McLaren1982),
        38 => Ok(Team::Williams2003),
        39 => Ok(Team::Brawn2009),
        40 => Ok(Team::Lotus1978),
        41 => Ok(Team::F1GenericCar),
        42 => Ok(Team::ArtGP2019),
        43 => Ok(Team::Campos2019),
        44 => Ok(Team::Carlin2019),
        45 => Ok(Team::SauberJuniorCharouz2019),
        46 => Ok(Team::Dams2019),
        47 => Ok(Team::UniVirtuosi2019),
        48 => Ok(Team::MPMotorsport2019),
        49 => Ok(Team::Prema2019),
        50 => Ok(Team::Trident2019),
        51 => Ok(Team::Arden2019),
        53 => Ok(Team::Benetton1994),
        54 => Ok(Team::Benetton1995),
        55 => Ok(Team::Ferrari2000),
        56 => Ok(Team::Jordan1991),
        255 => Ok(Team::MyTeam),
        _ => Err(UnpackError(format!("Invalid Team value: {}", value))),
    }
}

pub(crate) fn unpack_tyre_compound(value: u8) -> Result<TyreCompound, UnpackError> {
    match value {
        16 => Ok(TyreCompound::C5),
        17 => Ok(TyreCompound::C4),
        18 => Ok(TyreCompound::C3),
        19 => Ok(TyreCompound::C2),
        20 => Ok(TyreCompound::C1),
        7 => Ok(TyreCompound::Inter),
        8 => Ok(TyreCompound::Wet),
        9 => Ok(TyreCompound::ClassicDry),
        10 => Ok(TyreCompound::ClassicWet),
        11 => Ok(TyreCompound::F2SuperSoft),
        12 => Ok(TyreCompound::F2Soft),
        13 => Ok(TyreCompound::F2Medium),
        14 => Ok(TyreCompound::F2Hard),
        15 => Ok(TyreCompound::F2Wet),
        0 | 255 => Ok(TyreCompound::Invalid),
        _ => Err(UnpackError(format!(
            "Invalid TyreCompound value: {}",
            value
        ))),
    }
}

pub(crate) fn unpack_tyre_compound_visual(value: u8) -> Result<TyreCompoundVisual, UnpackError> {
    match value {
        16 => Ok(TyreCompoundVisual::Soft),
        17 => Ok(TyreCompoundVisual::Medium),
        18 => Ok(TyreCompoundVisual::Hard),
        7 => Ok(TyreCompoundVisual::Inter),
        8 => Ok(TyreCompoundVisual::Wet),
        9 => Ok(TyreCompoundVisual::ClassicDry),
        10 => Ok(TyreCompoundVisual::ClassicWet),
        11 => Ok(TyreCompoundVisual::F2SuperSoft),
        12 => Ok(TyreCompoundVisual::F2Soft),
        13 => Ok(TyreCompoundVisual::F2Medium),
        14 => Ok(TyreCompoundVisual::F2Hard),
        15 => Ok(TyreCompoundVisual::F2Wet),
        0 => Ok(TyreCompoundVisual::Invalid),
        _ => Err(UnpackError(format!(
            "Invalid TyreCompoundVisual value: {}",
            value
        ))),
    }
}
