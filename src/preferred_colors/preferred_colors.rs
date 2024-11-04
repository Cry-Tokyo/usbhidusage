#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}
impl RGB {
    fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r: r, g: g, b: b }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum PreferredColors8bit {
    AliceBlue {
        rgb_values: RGB,
    },
    AntiqueWhite {
        rgb_values: RGB,
    },
    Aqua {
        rgb_values: RGB,
    },
    Aquamarine {
        rgb_values: RGB,
    },
    Azure {
        rgb_values: RGB,
    },
    Beige {
        rgb_values: RGB,
    },
    Bisque {
        rgb_values: RGB,
    },
    Black {
        rgb_values: RGB,
    },
    BlanchedAlmond {
        rgb_values: RGB,
    },
    Blue {
        rgb_values: RGB,
    },
    BlueViolet {
        rgb_values: RGB,
    },
    Brown {
        rgb_values: RGB,
    },
    BurlyWood {
        rgb_values: RGB,
    },
    CadetBlue {
        rgb_values: RGB,
    },
    Chartreuse {
        rgb_values: RGB,
    },
    Chocolate {
        rgb_values: RGB,
    },
    Coral {
        rgb_values: RGB,
    },
    CornflowerBlue {
        rgb_values: RGB,
    },
    Cornsilk {
        rgb_values: RGB,
    },
    Crimson {
        rgb_values: RGB,
    },
    Cyan {
        rgb_values: RGB,
    },
    DarkBlue {
        rgb_values: RGB,
    },
    DarkCyan {
        rgb_values: RGB,
    },
    DarkGoldenRod {
        rgb_values: RGB,
    },
    DarkGray {
        rgb_values: RGB,
    },
    DarkGreen {
        rgb_values: RGB,
    },
    DarkKhaki {
        rgb_values: RGB,
    },
    DarkMagenta {
        rgb_values: RGB,
    },
    DarkOliveGreen {
        rgb_values: RGB,
    },
    DarkOrange {
        rgb_values: RGB,
    },
    DarkOrchid {
        rgb_values: RGB,
    },
    DarkRed {
        rgb_values: RGB,
    },
    DarkSalmon {
        rgb_values: RGB,
    },
    DarkSeaGreen {
        rgb_values: RGB,
    },
    DarkSlateBlue {
        rgb_values: RGB,
    },
    DarkSlateGray {
        rgb_values: RGB,
    },
    DarkTurquoise {
        rgb_values: RGB,
    },
    DarkViolet {
        rgb_values: RGB,
    },
    DeepPink {
        rgb_values: RGB,
    },
    DeepSkyBlue {
        rgb_values: RGB,
    },
    DimGray {
        rgb_values: RGB,
    },
    DodgerBlue {
        rgb_values: RGB,
    },
    FireBrick {
        rgb_values: RGB,
    },
    FloralWhite {
        rgb_values: RGB,
    },
    ForestGreen {
        rgb_values: RGB,
    },
    Fuchsia {
        rgb_values: RGB,
    },
    Gainsboro {
        rgb_values: RGB,
    },
    GhostWhite {
        rgb_values: RGB,
    },
    Gold {
        rgb_values: RGB,
    },
    GoldenRod {
        rgb_values: RGB,
    },
    Gray {
        rgb_values: RGB,
    },
    Green {
        rgb_values: RGB,
    },
    GreenYellow {
        rgb_values: RGB,
    },
    HoneyDew {
        rgb_values: RGB,
    },
    HotPink {
        rgb_values: RGB,
    },
    IndianRed {
        rgb_values: RGB,
    },
    Indigo {
        rgb_values: RGB,
    },
    Ivory {
        rgb_values: RGB,
    },
    Khaki {
        rgb_values: RGB,
    },
    Lavender {
        rgb_values: RGB,
    },
    LavenderBlush {
        rgb_values: RGB,
    },
    LawnGreen {
        rgb_values: RGB,
    },
    LemonChiffon {
        rgb_values: RGB,
    },
    LightBlue {
        rgb_values: RGB,
    },
    LightCoral {
        rgb_values: RGB,
    },
    LightCyan {
        rgb_values: RGB,
    },
    LightGoldenRodYellow {
        rgb_values: RGB,
    },
    LightGray {
        rgb_values: RGB,
    },
    LightGreen {
        rgb_values: RGB,
    },
    LightPink {
        rgb_values: RGB,
    },
    LightSalmon {
        rgb_values: RGB,
    },
    LightSeaGreen {
        rgb_values: RGB,
    },
    LightSkyBlue {
        rgb_values: RGB,
    },
    LightSlateGray {
        rgb_values: RGB,
    },
    LightSteelBlue {
        rgb_values: RGB,
    },
    LightYellow {
        rgb_values: RGB,
    },
    Lime {
        rgb_values: RGB,
    },
    LimeGreen {
        rgb_values: RGB,
    },
    Linen {
        rgb_values: RGB,
    },
    Magenta {
        rgb_values: RGB,
    },
    Maroon {
        rgb_values: RGB,
    },
    MediumAquaMarine {
        rgb_values: RGB,
    },
    MediumBlue {
        rgb_values: RGB,
    },
    MediumOrchid {
        rgb_values: RGB,
    },
    MediumPurple {
        rgb_values: RGB,
    },
    MediumSeaGreen {
        rgb_values: RGB,
    },
    MediumSlateBlue {
        rgb_values: RGB,
    },
    MediumSpringGreen {
        rgb_values: RGB,
    },
    MediumTurquoise {
        rgb_values: RGB,
    },
    MediumVioletRed {
        rgb_values: RGB,
    },
    MidnightBlue {
        rgb_values: RGB,
    },
    MintCream {
        rgb_values: RGB,
    },
    MistyRose {
        rgb_values: RGB,
    },
    Moccasin {
        rgb_values: RGB,
    },
    NavajoWhite {
        rgb_values: RGB,
    },
    Navy {
        rgb_values: RGB,
    },
    OldLace {
        rgb_values: RGB,
    },
    Olive {
        rgb_values: RGB,
    },
    OliveDrab {
        rgb_values: RGB,
    },
    Orange {
        rgb_values: RGB,
    },
    OrangeRed {
        rgb_values: RGB,
    },
    Orchid {
        rgb_values: RGB,
    },
    PaleGoldenRod {
        rgb_values: RGB,
    },
    PaleGreen {
        rgb_values: RGB,
    },
    PaleTurquoise {
        rgb_values: RGB,
    },
    PaleVioletRed {
        rgb_values: RGB,
    },
    PapayaWhip {
        rgb_values: RGB,
    },
    PeachPuff {
        rgb_values: RGB,
    },
    Peru {
        rgb_values: RGB,
    },
    Pink {
        rgb_values: RGB,
    },
    Plum {
        rgb_values: RGB,
    },
    PowderBlue {
        rgb_values: RGB,
    },
    Purple {
        rgb_values: RGB,
    },
    RebeccaPurple {
        rgb_values: RGB,
    },
    Red {
        rgb_values: RGB,
    },
    RosyBrown {
        rgb_values: RGB,
    },
    RoyalBlue {
        rgb_values: RGB,
    },
    SaddleBrown {
        rgb_values: RGB,
    },
    Salmon {
        rgb_values: RGB,
    },
    SandyBrown {
        rgb_values: RGB,
    },
    SeaGreen {
        rgb_values: RGB,
    },
    SeaShell {
        rgb_values: RGB,
    },
    Sienna {
        rgb_values: RGB,
    },
    Silver {
        rgb_values: RGB,
    },
    SkyBlue {
        rgb_values: RGB,
    },
    SlateBlue {
        rgb_values: RGB,
    },
    SlateGray {
        rgb_values: RGB,
    },
    Snow {
        rgb_values: RGB,
    },
    SpringGreen {
        rgb_values: RGB,
    },
    SteelBlue {
        rgb_values: RGB,
    },
    Tan {
        rgb_values: RGB,
    },
    Teal {
        rgb_values: RGB,
    },
    Thistle {
        rgb_values: RGB,
    },
    Tomato {
        rgb_values: RGB,
    },
    Turquoise {
        rgb_values: RGB,
    },
    Violet {
        rgb_values: RGB,
    },
    Wheat {
        rgb_values: RGB,
    },
    White {
        rgb_values: RGB,
    },
    WhiteSmoke {
        rgb_values: RGB,
    },
    Yellow {
        rgb_values: RGB,
    },
    YellowGreen {
        rgb_values: RGB,
    },
    Reserved141_254,
    #[default]
    NoPreferredColor = 255,
}
impl<T> From<T> for PreferredColors8bit
where
    T: TryInto<u8>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(255);
        match value {
            0 => Self::AliceBlue {
                rgb_values: RGB::new(0xF0, 0xF8, 0xFF),
            },
            1 => Self::AntiqueWhite {
                rgb_values: RGB::new(0xFA, 0xEB, 0xD7),
            },
            2 => Self::Aqua {
                rgb_values: RGB::new(0x00, 0xFF, 0xFF),
            },
            3 => Self::Aquamarine {
                rgb_values: RGB::new(0x7F, 0xFF, 0xD4),
            },
            4 => Self::Azure {
                rgb_values: RGB::new(0xF0, 0xFF, 0xFF),
            },
            5 => Self::Beige {
                rgb_values: RGB::new(0xF5, 0xF5, 0xDC),
            },
            6 => Self::Bisque {
                rgb_values: RGB::new(0xFF, 0xE4, 0xC4),
            },
            7 => Self::Black {
                rgb_values: RGB::new(0x00, 0x00, 0x00),
            },
            8 => Self::BlanchedAlmond {
                rgb_values: RGB::new(0xFF, 0xEB, 0xCD),
            },
            9 => Self::Blue {
                rgb_values: RGB::new(0x00, 0x00, 0xFF),
            },
            10 => Self::BlueViolet {
                rgb_values: RGB::new(0x8A, 0x2B, 0xE2),
            },
            11 => Self::Brown {
                rgb_values: RGB::new(0xA5, 0x2A, 0x2A),
            },
            12 => Self::BurlyWood {
                rgb_values: RGB::new(0xDE, 0xB8, 0x87),
            },
            13 => Self::CadetBlue {
                rgb_values: RGB::new(0x5F, 0x9E, 0xA0),
            },
            14 => Self::Chartreuse {
                rgb_values: RGB::new(0x7F, 0xFF, 0x00),
            },
            15 => Self::Chocolate {
                rgb_values: RGB::new(0xD2, 0x69, 0x1E),
            },
            16 => Self::Coral {
                rgb_values: RGB::new(0xFF, 0x7F, 0x50),
            },
            17 => Self::CornflowerBlue {
                rgb_values: RGB::new(0x64, 0x95, 0xED),
            },
            18 => Self::Cornsilk {
                rgb_values: RGB::new(0xFF, 0xF8, 0xDC),
            },
            19 => Self::Crimson {
                rgb_values: RGB::new(0xDC, 0x14, 0x3C),
            },
            20 => Self::Cyan {
                rgb_values: RGB::new(0x00, 0xFF, 0xFF),
            },
            21 => Self::DarkBlue {
                rgb_values: RGB::new(0x00, 0x00, 0x8B),
            },
            22 => Self::DarkCyan {
                rgb_values: RGB::new(0x00, 0x8B, 0x8B),
            },
            23 => Self::DarkGoldenRod {
                rgb_values: RGB::new(0xB8, 0x86, 0x0B),
            },
            24 => Self::DarkGray {
                rgb_values: RGB::new(0xA9, 0xA9, 0xA9),
            },
            25 => Self::DarkGreen {
                rgb_values: RGB::new(0x00, 0x64, 0x00),
            },
            26 => Self::DarkKhaki {
                rgb_values: RGB::new(0xBD, 0xB7, 0x6B),
            },
            27 => Self::DarkMagenta {
                rgb_values: RGB::new(0x8B, 0x00, 0x8B),
            },
            28 => Self::DarkOliveGreen {
                rgb_values: RGB::new(0x55, 0x6B, 0x2F),
            },
            29 => Self::DarkOrange {
                rgb_values: RGB::new(0xFF, 0x8C, 0x00),
            },
            30 => Self::DarkOrchid {
                rgb_values: RGB::new(0x99, 0x32, 0xCC),
            },
            31 => Self::DarkRed {
                rgb_values: RGB::new(0x8B, 0x00, 0x00),
            },
            32 => Self::DarkSalmon {
                rgb_values: RGB::new(0xE9, 0x96, 0x7A),
            },
            33 => Self::DarkSeaGreen {
                rgb_values: RGB::new(0x8F, 0xBC, 0x8F),
            },
            34 => Self::DarkSlateBlue {
                rgb_values: RGB::new(0x48, 0x3D, 0x8B),
            },
            35 => Self::DarkSlateGray {
                rgb_values: RGB::new(0x2F, 0x4F, 0x4F),
            },
            36 => Self::DarkTurquoise {
                rgb_values: RGB::new(0x00, 0xCE, 0xD1),
            },
            37 => Self::DarkViolet {
                rgb_values: RGB::new(0x94, 0x00, 0xD3),
            },
            38 => Self::DeepPink {
                rgb_values: RGB::new(0xFF, 0x14, 0x93),
            },
            39 => Self::DeepSkyBlue {
                rgb_values: RGB::new(0x00, 0xBF, 0xFF),
            },
            40 => Self::DimGray {
                rgb_values: RGB::new(0x69, 0x69, 0x69),
            },
            41 => Self::DodgerBlue {
                rgb_values: RGB::new(0x1E, 0x90, 0xFF),
            },
            42 => Self::FireBrick {
                rgb_values: RGB::new(0xB2, 0x22, 0x22),
            },
            43 => Self::FloralWhite {
                rgb_values: RGB::new(0xFF, 0xFA, 0xF0),
            },
            44 => Self::ForestGreen {
                rgb_values: RGB::new(0x22, 0x8B, 0x22),
            },
            45 => Self::Fuchsia {
                rgb_values: RGB::new(0xFF, 0x00, 0xFF),
            },
            46 => Self::Gainsboro {
                rgb_values: RGB::new(0xDC, 0xDC, 0xDC),
            },
            47 => Self::GhostWhite {
                rgb_values: RGB::new(0xF8, 0xF8, 0xFF),
            },
            48 => Self::Gold {
                rgb_values: RGB::new(0xFF, 0xD7, 0x00),
            },
            49 => Self::GoldenRod {
                rgb_values: RGB::new(0xDA, 0xA5, 0x20),
            },
            50 => Self::Gray {
                rgb_values: RGB::new(0x80, 0x80, 0x80),
            },
            51 => Self::Green {
                rgb_values: RGB::new(0x00, 0x80, 0x00),
            },
            52 => Self::GreenYellow {
                rgb_values: RGB::new(0xAD, 0xFF, 0x2F),
            },
            53 => Self::HoneyDew {
                rgb_values: RGB::new(0xF0, 0xFF, 0xF0),
            },
            54 => Self::HotPink {
                rgb_values: RGB::new(0xFF, 0x69, 0xB4),
            },
            55 => Self::IndianRed {
                rgb_values: RGB::new(0xCD, 0x5C, 0x5C),
            },
            56 => Self::Indigo {
                rgb_values: RGB::new(0x4B, 0x00, 0x82),
            },
            57 => Self::Ivory {
                rgb_values: RGB::new(0xFF, 0xFF, 0xF0),
            },
            58 => Self::Khaki {
                rgb_values: RGB::new(0xF0, 0xE6, 0x8C),
            },
            59 => Self::Lavender {
                rgb_values: RGB::new(0xE6, 0xE6, 0xFA),
            },
            60 => Self::LavenderBlush {
                rgb_values: RGB::new(0xFF, 0xF0, 0xF5),
            },
            61 => Self::LawnGreen {
                rgb_values: RGB::new(0x7C, 0xFC, 0x00),
            },
            62 => Self::LemonChiffon {
                rgb_values: RGB::new(0xFF, 0xFA, 0xCD),
            },
            63 => Self::LightBlue {
                rgb_values: RGB::new(0xAD, 0xD8, 0xE6),
            },
            64 => Self::LightCoral {
                rgb_values: RGB::new(0xF0, 0x80, 0x80),
            },
            65 => Self::LightCyan {
                rgb_values: RGB::new(0xE0, 0xFF, 0xFF),
            },
            66 => Self::LightGoldenRodYellow {
                rgb_values: RGB::new(0xFA, 0xFA, 0xD2),
            },
            67 => Self::LightGray {
                rgb_values: RGB::new(0xD3, 0xD3, 0xD3),
            },
            68 => Self::LightGreen {
                rgb_values: RGB::new(0x90, 0xEE, 0x90),
            },
            69 => Self::LightPink {
                rgb_values: RGB::new(0xFF, 0xB6, 0xC1),
            },
            70 => Self::LightSalmon {
                rgb_values: RGB::new(0xFF, 0xA0, 0x7A),
            },
            71 => Self::LightSeaGreen {
                rgb_values: RGB::new(0x20, 0xB2, 0xAA),
            },
            72 => Self::LightSkyBlue {
                rgb_values: RGB::new(0x87, 0xCE, 0xFA),
            },
            73 => Self::LightSlateGray {
                rgb_values: RGB::new(0x77, 0x88, 0x99),
            },
            74 => Self::LightSteelBlue {
                rgb_values: RGB::new(0xB0, 0xC4, 0xDE),
            },
            75 => Self::LightYellow {
                rgb_values: RGB::new(0xFF, 0xFF, 0xE0),
            },
            76 => Self::Lime {
                rgb_values: RGB::new(0x00, 0xFF, 0x00),
            },
            77 => Self::LimeGreen {
                rgb_values: RGB::new(0x32, 0xCD, 0x32),
            },
            78 => Self::Linen {
                rgb_values: RGB::new(0xFA, 0xF0, 0xE6),
            },
            79 => Self::Magenta {
                rgb_values: RGB::new(0xFF, 0x00, 0xFF),
            },
            80 => Self::Maroon {
                rgb_values: RGB::new(0x80, 0x00, 0x00),
            },
            81 => Self::MediumAquaMarine {
                rgb_values: RGB::new(0x66, 0xCD, 0xAA),
            },
            82 => Self::MediumBlue {
                rgb_values: RGB::new(0x00, 0x00, 0xCD),
            },
            83 => Self::MediumOrchid {
                rgb_values: RGB::new(0xBA, 0x55, 0xD3),
            },
            84 => Self::MediumPurple {
                rgb_values: RGB::new(0x93, 0x70, 0xDB),
            },
            85 => Self::MediumSeaGreen {
                rgb_values: RGB::new(0x3C, 0xB3, 0x71),
            },
            86 => Self::MediumSlateBlue {
                rgb_values: RGB::new(0x7B, 0x68, 0xEE),
            },
            87 => Self::MediumSpringGreen {
                rgb_values: RGB::new(0x00, 0xFA, 0x9A),
            },
            88 => Self::MediumTurquoise {
                rgb_values: RGB::new(0x48, 0xD1, 0xCC),
            },
            89 => Self::MediumVioletRed {
                rgb_values: RGB::new(0xC7, 0x15, 0x85),
            },
            90 => Self::MidnightBlue {
                rgb_values: RGB::new(0x19, 0x19, 0x70),
            },
            91 => Self::MintCream {
                rgb_values: RGB::new(0xF5, 0xFF, 0xFA),
            },
            92 => Self::MistyRose {
                rgb_values: RGB::new(0xFF, 0xE4, 0xE1),
            },
            93 => Self::Moccasin {
                rgb_values: RGB::new(0xFF, 0xE4, 0xB5),
            },
            94 => Self::NavajoWhite {
                rgb_values: RGB::new(0xFF, 0xDE, 0xAD),
            },
            95 => Self::Navy {
                rgb_values: RGB::new(0x00, 0x00, 0x80),
            },
            96 => Self::OldLace {
                rgb_values: RGB::new(0xFD, 0xF5, 0xE6),
            },
            97 => Self::Olive {
                rgb_values: RGB::new(0x80, 0x80, 0x00),
            },
            98 => Self::OliveDrab {
                rgb_values: RGB::new(0x6B, 0x8E, 0x23),
            },
            99 => Self::Orange {
                rgb_values: RGB::new(0xFF, 0xA5, 0x00),
            },
            100 => Self::OrangeRed {
                rgb_values: RGB::new(0xFF, 0x45, 0x00),
            },
            101 => Self::Orchid {
                rgb_values: RGB::new(0xDA, 0x70, 0xD6),
            },
            102 => Self::PaleGoldenRod {
                rgb_values: RGB::new(0xEE, 0xE8, 0xAA),
            },
            103 => Self::PaleGreen {
                rgb_values: RGB::new(0x98, 0xFB, 0x98),
            },
            104 => Self::PaleTurquoise {
                rgb_values: RGB::new(0xAF, 0xEE, 0xEE),
            },
            105 => Self::PaleVioletRed {
                rgb_values: RGB::new(0xDB, 0x70, 0x93),
            },
            106 => Self::PapayaWhip {
                rgb_values: RGB::new(0xFF, 0xEF, 0xD5),
            },
            107 => Self::PeachPuff {
                rgb_values: RGB::new(0xFF, 0xDA, 0xB9),
            },
            108 => Self::Peru {
                rgb_values: RGB::new(0xCD, 0x85, 0x3F),
            },
            109 => Self::Pink {
                rgb_values: RGB::new(0xFF, 0xC0, 0xCB),
            },
            110 => Self::Plum {
                rgb_values: RGB::new(0xDD, 0xA0, 0xDD),
            },
            111 => Self::PowderBlue {
                rgb_values: RGB::new(0xB0, 0xE0, 0xE6),
            },
            112 => Self::Purple {
                rgb_values: RGB::new(0x80, 0x00, 0x80),
            },
            113 => Self::RebeccaPurple {
                rgb_values: RGB::new(0x66, 0x33, 0x99),
            },
            114 => Self::Red {
                rgb_values: RGB::new(0xFF, 0x00, 0x00),
            },
            115 => Self::RosyBrown {
                rgb_values: RGB::new(0xBC, 0x8F, 0x8F),
            },
            116 => Self::RoyalBlue {
                rgb_values: RGB::new(0x41, 0x69, 0xE1),
            },
            117 => Self::SaddleBrown {
                rgb_values: RGB::new(0x8B, 0x45, 0x13),
            },
            118 => Self::Salmon {
                rgb_values: RGB::new(0xFA, 0x80, 0x72),
            },
            119 => Self::SandyBrown {
                rgb_values: RGB::new(0xF4, 0xA4, 0x60),
            },
            120 => Self::SeaGreen {
                rgb_values: RGB::new(0x2E, 0x8B, 0x57),
            },
            121 => Self::SeaShell {
                rgb_values: RGB::new(0xFF, 0xF5, 0xEE),
            },
            122 => Self::Sienna {
                rgb_values: RGB::new(0xA0, 0x52, 0x2D),
            },
            123 => Self::Silver {
                rgb_values: RGB::new(0xC0, 0xC0, 0xC0),
            },
            124 => Self::SkyBlue {
                rgb_values: RGB::new(0x87, 0xCE, 0xEB),
            },
            125 => Self::SlateBlue {
                rgb_values: RGB::new(0x6A, 0x5A, 0xCD),
            },
            126 => Self::SlateGray {
                rgb_values: RGB::new(0x70, 0x80, 0x90),
            },
            127 => Self::Snow {
                rgb_values: RGB::new(0xFF, 0xFA, 0xFA),
            },
            128 => Self::SpringGreen {
                rgb_values: RGB::new(0x00, 0xFF, 0x7F),
            },
            129 => Self::SteelBlue {
                rgb_values: RGB::new(0x46, 0x82, 0xB4),
            },
            130 => Self::Tan {
                rgb_values: RGB::new(0xD2, 0xB4, 0x8C),
            },
            131 => Self::Teal {
                rgb_values: RGB::new(0x00, 0x80, 0x80),
            },
            132 => Self::Thistle {
                rgb_values: RGB::new(0xD8, 0xBF, 0xD8),
            },
            133 => Self::Tomato {
                rgb_values: RGB::new(0xFF, 0x63, 0x47),
            },
            134 => Self::Turquoise {
                rgb_values: RGB::new(0x40, 0xE0, 0xD0),
            },
            135 => Self::Violet {
                rgb_values: RGB::new(0xEE, 0x82, 0xEE),
            },
            136 => Self::Wheat {
                rgb_values: RGB::new(0xF5, 0xDE, 0xB3),
            },
            137 => Self::White {
                rgb_values: RGB::new(0xFF, 0xFF, 0xFF),
            },
            138 => Self::WhiteSmoke {
                rgb_values: RGB::new(0xF5, 0xF5, 0xF5),
            },
            139 => Self::Yellow {
                rgb_values: RGB::new(0xFF, 0xFF, 0x00),
            },
            140 => Self::YellowGreen {
                rgb_values: RGB::new(0x9A, 0xCD, 0x32),
            },
            141..255 => Self::Reserved141_254,
            255 => Self::NoPreferredColor,
        }
    }
}
