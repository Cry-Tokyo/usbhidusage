#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[repr(u16)]
pub enum KeyboardUsage {
    /// Usally means no key was pressed
    #[default]
    Reserved00_00,
    /// Occurs when more keys are pressed then the keybaord can handle at the same time
    KeyboardErrorRollOver,
    /// Indicates that the keyboard has failed the POST test
    KeyboardPOSTFail,
    /// Undefined or unrecognized error condition
    KeyboardErrorUndefined,
    /// a or A key
    KeyboardaA,
    /// b or B key
    KeyboardbB,
    /// c or C key
    KeyboardcC,
    /// d or D key
    KeyboarddD,
    /// e or E key
    KeyboardeE,
    /// f or F key
    KeyboardfF,
    /// g or G key
    KeyboardgG,
    /// h or H key
    KeyboardhH,
    /// i or I key
    KeyboardiI,
    /// j or J key
    KeyboardjJ,
    /// k or K key
    KeyboardkK,
    /// l or L key
    KeyboardlL,
    /// m or M key
    KeyboardmM,
    /// n or N key
    KeyboardnN,
    /// o or O key
    KeyboardoO,
    /// p or P key
    KeyboardpP,
    /// q or Q key
    KeyboardqQ,
    /// r or R key
    KeyboardrR,
    /// s or S key
    KeyboardsS,
    /// t or T key
    KeyboardtT,
    /// u or U key
    KeyboarduU,
    /// v or V key
    KeyboardvV,
    /// w or W key
    KeyboardwW,
    /// x or X key
    KeyboardxX,
    /// y or Y key
    KeyboardyY,
    /// z or Z key
    KeyboardzZ,
    /// 1 or ! key
    Keyboard1ExclamationPoint,
    /// 2 or @ key
    Keyboard2At,
    /// 3 or # key
    Keyboard3Pound,
    /// 4 or $ key
    Keyboard4DollarSign,
    /// 5 or % key
    Keyboard5PercentSign,
    /// 6 or ^ key
    Keyboard6Carret,
    /// 7 or & key
    Keyboard7Ampersand,
    /// 8 or * key
    Keyboard8Asterisk,
    /// 9 or ( key
    Keyboard9LeftParentheses,
    /// 0 or ) key
    Keyboard0RightParentheses,
    /// used in forms or provides '\n' new line when writting text
    KeyboardReturnEnter,
    /// used to cancel, close, exit, dismiss or escape
    KeyboardEsc,
    /// used to delete character to the left
    KeyboardDeleteBackspace,
    /// used for idents or changing focus
    KeyboardTab,
    /// inserts a ' ' space
    KeyboardSpacebar,
    /// - or _ key
    KeyboardHyphenUnderscore,
    /// = or + key
    KeyboardEqualPlus,
    /// [ or { key
    KeyboardLeftBracket,
    /// ] or } key
    KeyboardRigthBracket,
    /// \ or | key
    KeyboardBackslashPipe,
    /// # or ~ key
    KeyboardNonUSPoundTilda,
    /// ; or : key
    KeyboardSemiNonColon,
    /// ' or " key
    KeyboardSingleDoubleQuotes,
    /// ` or ~ key
    KeyboardGraveAccentTilde,
    /// , or < key
    KeyboardCommnaLessThen,
    /// . or > key
    KeyboardPeriodGreaterThen,
    /// / or ? key
    KeyboardForwardSlashQuestionMark,
    /// used
    KeyboardCapsLock,
    /// toggles the input of uppercase letters
    KeyboardF1,
    /// function key
    KeyboardF2,
    /// function key
    KeyboardF3,
    /// function key
    KeyboardF4,
    /// function key
    KeyboardF5,
    /// function key
    KeyboardF6,
    /// function key
    KeyboardF7,
    /// function key
    KeyboardF8,
    /// function key
    KeyboardF9,
    /// function key
    KeyboardF10,
    /// function key
    KeyboardF11,
    /// function key
    KeyboardF12,
    /// used to capture a screenshot
    KeyboardPrintScreen,
    /// changes the behavior of the arrow and navigation keys
    KeyboardScrollLock,
    /// stops the execution of commands
    KeyboardPause,
    /// toggles insert or overwrite
    KeyboardInsert,
    /// moves cursor to beginning of the line
    KeyboardHome,
    /// scrolls up
    KeyboardPageUp,
    /// removes the right character
    KeyboardDeleteFoward,
    /// moves cursor to the end of the line
    KeyboardEnd,
    /// scrolls down
    KeyboardPageDown,
    /// moves cursor or focus to the right
    KeyboardRightArrow,
    /// moves cursor or focus to the left
    KeyboardLeftArrow,
    /// moves cursor or focus down
    KeyboardDownArrow,
    /// moves cursor or focus up
    KeyboardUpArrow,
    /// toggles the numpad
    KeypadNumLockClear,
    /// / key
    KeypadFowardSlash,
    /// * key
    KeypadAsterisk,
    /// - key
    KeypadMinus,
    /// + key
    KeypadPlus,
    /// used in forms
    KeypadEnter,
    /// 1 or
    Keypad1End,
    ///
    Keypad2DownArrow,
    ///
    Keypad3PageDn,
    ///
    Keypad4LeftArrow,
    ///
    Keypad5,
    ///
    Keypad6RightArrow,
    ///
    Keypad7Home,
    ///
    Keypad8UpArrow,
    ///
    Keypad9PageUp,
    ///
    Keypad0Insert,
    ///
    KeypadPeriodDelete,
    ///
    KeyboardNonUSBackwardSlashPipe,
    ///
    KeyboardApplication,
    ///
    KeyboardPower,
    ///
    KeypadEqual,
    /// function key
    KeyboardF13,
    /// function key
    KeyboardF14,
    /// function key
    KeyboardF15,
    /// function key
    KeyboardF16,
    /// function key
    KeyboardF17,
    /// function key
    KeyboardF18,
    /// function key
    KeyboardF19,
    /// function key
    KeyboardF20,
    /// function key
    KeyboardF21,
    /// function key
    KeyboardF22,
    /// function key
    KeyboardF23,
    /// function key
    KeyboardF24,
    /// function key
    KeyboardExecute,
    ///
    KeyboardHelp,
    ///
    KeyboardMenu,
    ///
    KeyboardSelect,
    ///
    KeyboardStop,
    ///
    KeyboardAgain,
    ///
    KeyboardUndo,
    ///
    KeyboardCut,
    ///
    KeyboardCopy,
    ///
    KeyboardPaste,
    ///
    KeyboardFind,
    ///
    KeyboardMute,
    ///
    KeyboardVolumeUp,
    ///
    KeyboardVolumeDown,
    ///
    KeyboardLockingCapsLock,
    ///
    KeyboardLockingNumLock,
    ///
    KeyboardLockingScrollLock,
    ///
    KeypadComma,
    ///
    KeypadEqualSign,
    ///
    KeyboardInternational1,
    ///
    KeyboardInternational2,
    ///
    KeyboardInternational3,
    ///
    KeyboardInternational4,
    ///
    KeyboardInternational5,
    ///
    KeyboardInternational6,
    ///
    KeyboardInternational7,
    ///
    KeyboardInternational8,
    ///
    KeyboardInternational9,
    ///
    KeyboardLANG1,
    ///
    KeyboardLANG2,
    ///
    KeyboardLANG3,
    ///
    KeyboardLANG4,
    ///
    KeyboardLANG5,
    ///
    KeyboardLANG6,
    ///
    KeyboardLANG7,
    ///
    KeyboardLANG8,
    ///
    KeyboardLANG9,
    ///
    KeyboardAlternateErase,
    ///
    KeyboardSysReqAttention,
    ///
    KeyboardCancel,
    ///
    KeyboardClear,
    ///
    KeyboardPrior,
    ///
    KeyboardReturn,
    ///
    KeyboardSeparator,
    ///
    KeyboardOut,
    ///
    KeyboardOper,
    ///
    KeyboardClearAgain,
    ///
    KeyboardCrSelProps,
    ///
    KeyboardExSel,
    ///
    ReservedA5_Af(u16),
    ///
    Keypad00 = 176,
    ///
    Keypad000,
    ///
    ThousandsSeparator,
    ///
    DecimalSeparator,
    ///
    CurrencyUnit,
    ///
    CurrencySubunit,
    ///
    KeypadLeftParenthesis,
    ///
    KeypadRightParenthesis,
    ///
    KeypadLeftCurlyBracket,
    ///
    KeypadRightCurlyBracket,
    ///
    KeypadTab,
    ///
    KeypadBackspace,
    ///
    KeypadA,
    ///
    KeypadB,
    ///
    KeypadC,
    ///
    KeypadD,
    ///
    KeypadE,
    ///
    KeypadF,
    ///
    KeypadXOR,
    ///
    KeypadCarret,
    ///
    KeypadPrecentSign,
    ///
    KeypadLessThen,
    ///
    KeypadGreaterThen,
    ///
    KeypadAppersand,
    ///
    Keypad2Appersand,
    ///
    KeypadPipe,
    ///
    Keypad2Pipe,
    ///
    KeypadColon,
    ///
    KeypadPound,
    ///
    KeypadSpace,
    ///
    KeypadAt,
    ///
    KeypadExclamtionPoint,
    ///
    KeypadMemoryStore,
    ///
    KeypadMemoryRecall,
    ///
    KeypadMemoryClear,
    ///
    KeypadMemoryAdd,
    ///
    KeypadMemorySubtract,
    ///
    KeypadMemoryMultiply,
    ///
    KeypadMemoryDivide,
    ///
    KeypadPlusMinus,
    ///
    KeypadClear,
    ///
    KeypadClearEntry,
    ///
    KeypadBinary,
    ///
    KeypadOctal,
    ///
    KeypadDecimal,
    ///
    KeypadHexadecimal,
    /// future use or specific purpose not defined
    ReservedDE_DF(u16),
    /// left ctrl key
    KeyboardLeftControl = 224,
    /// left shift key
    KeyboardLeftShift,
    /// left alt key
    KeyboardLeftAlt,
    /// left super, command, or windows key
    KeyboardLeftGUI,
    /// right ctrl key
    KeyboardRightControl,
    /// right shift key
    KeyboardRightShift,
    /// right alt key
    KeyboardRightAlt,
    /// right super, command, or windows key
    KeyboardRightGUI,
    /// future use or specific purpose not defined
    ReservedE8_FFFF(u16),
}
impl<T> From<T> for KeyboardUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value: u16 = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Reserved00_00,
            1 => Self::KeyboardErrorRollOver,
            2 => Self::KeyboardPOSTFail,
            3 => Self::KeyboardErrorUndefined,
            4 => Self::KeyboardaA,
            5 => Self::KeyboardbB,
            6 => Self::KeyboardcC,
            7 => Self::KeyboarddD,
            8 => Self::KeyboardeE,
            9 => Self::KeyboardfF,
            10 => Self::KeyboardgG,
            11 => Self::KeyboardhH,
            12 => Self::KeyboardiI,
            13 => Self::KeyboardjJ,
            14 => Self::KeyboardkK,
            15 => Self::KeyboardlL,
            16 => Self::KeyboardmM,
            17 => Self::KeyboardnN,
            18 => Self::KeyboardoO,
            19 => Self::KeyboardpP,
            20 => Self::KeyboardqQ,
            21 => Self::KeyboardrR,
            22 => Self::KeyboardsS,
            23 => Self::KeyboardtT,
            24 => Self::KeyboarduU,
            25 => Self::KeyboardvV,
            26 => Self::KeyboardwW,
            27 => Self::KeyboardxX,
            28 => Self::KeyboardyY,
            29 => Self::KeyboardzZ,
            30 => Self::Keyboard1ExclamationPoint,
            31 => Self::Keyboard2At,
            32 => Self::Keyboard3Pound,
            33 => Self::Keyboard4DollarSign,
            34 => Self::Keyboard5PercentSign,
            35 => Self::Keyboard6Carret,
            36 => Self::Keyboard7Ampersand,
            37 => Self::Keyboard8Asterisk,
            38 => Self::Keyboard9LeftParentheses,
            39 => Self::Keyboard0RightParentheses,
            40 => Self::KeyboardReturnEnter,
            41 => Self::KeyboardEsc,
            42 => Self::KeyboardDeleteBackspace,
            43 => Self::KeyboardTab,
            44 => Self::KeyboardSpacebar,
            45 => Self::KeyboardHyphenUnderscore,
            46 => Self::KeyboardEqualPlus,
            47 => Self::KeyboardLeftBracket,
            48 => Self::KeyboardRigthBracket,
            49 => Self::KeyboardBackslashPipe,
            50 => Self::KeyboardNonUSPoundTilda,
            51 => Self::KeyboardSemiNonColon,
            52 => Self::KeyboardSingleDoubleQuotes,
            53 => Self::KeyboardGraveAccentTilde,
            54 => Self::KeyboardCommnaLessThen,
            55 => Self::KeyboardPeriodGreaterThen,
            56 => Self::KeyboardForwardSlashQuestionMark,
            57 => Self::KeyboardCapsLock,
            58 => Self::KeyboardF1,
            59 => Self::KeyboardF2,
            60 => Self::KeyboardF3,
            61 => Self::KeyboardF4,
            62 => Self::KeyboardF5,
            63 => Self::KeyboardF6,
            64 => Self::KeyboardF7,
            65 => Self::KeyboardF8,
            66 => Self::KeyboardF9,
            67 => Self::KeyboardF10,
            68 => Self::KeyboardF11,
            69 => Self::KeyboardF12,
            70 => Self::KeyboardPrintScreen,
            71 => Self::KeyboardScrollLock,
            72 => Self::KeyboardPause,
            73 => Self::KeyboardInsert,
            74 => Self::KeyboardHome,
            75 => Self::KeyboardPageUp,
            76 => Self::KeyboardDeleteFoward,
            77 => Self::KeyboardEnd,
            78 => Self::KeyboardPageDown,
            79 => Self::KeyboardRightArrow,
            80 => Self::KeyboardLeftArrow,
            81 => Self::KeyboardDownArrow,
            82 => Self::KeyboardUpArrow,
            83 => Self::KeypadNumLockClear,
            84 => Self::KeypadFowardSlash,
            85 => Self::KeypadAsterisk,
            86 => Self::KeypadMinus,
            87 => Self::KeypadPlus,
            88 => Self::KeypadEnter,
            89 => Self::Keypad1End,
            90 => Self::Keypad2DownArrow,
            91 => Self::Keypad3PageDn,
            92 => Self::Keypad4LeftArrow,
            93 => Self::Keypad5,
            94 => Self::Keypad6RightArrow,
            95 => Self::Keypad7Home,
            96 => Self::Keypad8UpArrow,
            97 => Self::Keypad9PageUp,
            98 => Self::Keypad0Insert,
            99 => Self::KeypadPeriodDelete,
            100 => Self::KeyboardNonUSBackwardSlashPipe,
            101 => Self::KeyboardApplication,
            102 => Self::KeyboardPower,
            103 => Self::KeypadEqual,
            104 => Self::KeyboardF13,
            105 => Self::KeyboardF14,
            106 => Self::KeyboardF15,
            107 => Self::KeyboardF16,
            108 => Self::KeyboardF17,
            109 => Self::KeyboardF18,
            110 => Self::KeyboardF19,
            111 => Self::KeyboardF20,
            112 => Self::KeyboardF21,
            113 => Self::KeyboardF22,
            114 => Self::KeyboardF23,
            115 => Self::KeyboardF24,
            116 => Self::KeyboardExecute,
            117 => Self::KeyboardHelp,
            118 => Self::KeyboardMenu,
            119 => Self::KeyboardSelect,
            120 => Self::KeyboardStop,
            121 => Self::KeyboardAgain,
            122 => Self::KeyboardUndo,
            123 => Self::KeyboardCut,
            124 => Self::KeyboardCopy,
            125 => Self::KeyboardPaste,
            126 => Self::KeyboardFind,
            127 => Self::KeyboardMute,
            128 => Self::KeyboardVolumeUp,
            129 => Self::KeyboardVolumeDown,
            130 => Self::KeyboardLockingNumLock,
            131 => Self::KeyboardLockingNumLock,
            132 => Self::KeyboardLockingScrollLock,
            133 => Self::KeypadComma,
            134 => Self::KeypadEqualSign,
            135 => Self::KeyboardInternational1,
            136 => Self::KeyboardInternational2,
            137 => Self::KeyboardInternational3,
            138 => Self::KeyboardInternational4,
            139 => Self::KeyboardInternational5,
            140 => Self::KeyboardInternational6,
            141 => Self::KeyboardInternational7,
            142 => Self::KeyboardInternational8,
            143 => Self::KeyboardInternational9,
            144 => Self::KeyboardLANG1,
            145 => Self::KeyboardLANG2,
            146 => Self::KeyboardLANG3,
            147 => Self::KeyboardLANG4,
            148 => Self::KeyboardLANG5,
            149 => Self::KeyboardLANG6,
            150 => Self::KeyboardLANG7,
            151 => Self::KeyboardLANG8,
            152 => Self::KeyboardLANG9,
            153 => Self::KeyboardAlternateErase,
            154 => Self::KeyboardSysReqAttention,
            155 => Self::KeyboardCancel,
            156 => Self::KeyboardClear,
            157 => Self::KeyboardPrior,
            158 => Self::KeyboardReturn,
            159 => Self::KeyboardSeparator,
            160 => Self::KeyboardOut,
            161 => Self::KeyboardOper,
            162 => Self::KeyboardClearAgain,
            163 => Self::KeyboardCrSelProps,
            164 => Self::KeyboardExSel,
            165..=175 => Self::ReservedA5_Af(value),
            176 => Self::Keypad00,
            177 => Self::Keypad000,
            178 => Self::ThousandsSeparator,
            179 => Self::DecimalSeparator,
            180 => Self::CurrencyUnit,
            181 => Self::CurrencySubunit,
            182 => Self::KeypadLeftParenthesis,
            183 => Self::KeypadRightParenthesis,
            184 => Self::KeypadLeftCurlyBracket,
            185 => Self::KeypadRightCurlyBracket,
            186 => Self::KeypadTab,
            187 => Self::KeypadBackspace,
            188 => Self::KeypadA,
            189 => Self::KeypadB,
            190 => Self::KeypadC,
            191 => Self::KeypadD,
            192 => Self::KeypadE,
            193 => Self::KeypadF,
            194 => Self::KeypadXOR,
            195 => Self::KeypadCarret,
            196 => Self::KeypadPrecentSign,
            197 => Self::KeypadLessThen,
            198 => Self::KeypadGreaterThen,
            199 => Self::KeypadAppersand,
            200 => Self::Keypad2Appersand,
            201 => Self::KeypadPipe,
            202 => Self::Keypad2Pipe,
            203 => Self::KeypadColon,
            204 => Self::KeypadPound,
            205 => Self::KeypadSpace,
            206 => Self::KeypadAt,
            207 => Self::KeypadExclamtionPoint,
            208 => Self::KeypadMemoryStore,
            209 => Self::KeypadMemoryRecall,
            210 => Self::KeypadClear,
            211 => Self::KeypadMemoryAdd,
            212 => Self::KeypadMemorySubtract,
            213 => Self::KeypadMemoryMultiply,
            214 => Self::KeypadMemoryDivide,
            215 => Self::KeypadPlusMinus,
            216 => Self::KeypadClear,
            217 => Self::KeypadClearEntry,
            218 => Self::KeypadBinary,
            219 => Self::KeypadOctal,
            220 => Self::KeypadDecimal,
            221 => Self::KeypadHexadecimal,
            222..=223 => Self::ReservedDE_DF(value),
            224 => Self::KeyboardLeftControl,
            225 => Self::KeyboardLeftShift,
            226 => Self::KeyboardLeftAlt,
            227 => Self::KeyboardLeftGUI,
            228 => Self::KeyboardRightControl,
            229 => Self::KeyboardRightShift,
            230 => Self::KeyboardRightAlt,
            231 => Self::KeyboardRightGUI,
            232..=65535 => Self::ReservedE8_FFFF(value),
        }
    }
}
impl std::fmt::Display for KeyboardUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Key Code: {}", self.to_string())
    }
}
impl KeyboardUsage {
    pub fn to_symbol(&self) -> Option<(&str, &str)> {
        match self {
            Self::KeyboardaA => Some(("a", "A")),
            Self::KeyboardbB => Some(("b", "B")),
            Self::KeyboardcC => Some(("c", "C")),
            Self::KeyboarddD => Some(("d", "D")),
            Self::KeyboardeE => Some(("e", "E")),
            Self::KeyboardfF => Some(("f", "F")),
            Self::KeyboardgG => Some(("g", "G")),
            Self::KeyboardhH => Some(("h", "H")),
            Self::KeyboardiI => Some(("i", "I")),
            Self::KeyboardjJ => Some(("j", "J")),
            Self::KeyboardkK => Some(("k", "K")),
            Self::KeyboardlL => Some(("l", "L")),
            Self::KeyboardmM => Some(("m", "M")),
            Self::KeyboardnN => Some(("n", "N")),
            Self::KeyboardoO => Some(("o", "O")),
            Self::KeyboardpP => Some(("p", "P")),
            Self::KeyboardqQ => Some(("q", "Q")),
            Self::KeyboardrR => Some(("r", "R")),
            Self::KeyboardsS => Some(("s", "S")),
            Self::KeyboardtT => Some(("t", "T")),
            Self::KeyboarduU => Some(("u", "U")),
            Self::KeyboardvV => Some(("v", "V")),
            Self::KeyboardwW => Some(("w", "W")),
            Self::KeyboardxX => Some(("x", "X")),
            Self::KeyboardyY => Some(("y", "Y")),
            Self::KeyboardzZ => Some(("z", "Z")),
            Self::Keyboard1ExclamationPoint => Some(("1", "!")),
            Self::Keyboard2At => Some(("2", "@")),
            Self::Keyboard3Pound => Some(("3", "#")),
            Self::Keyboard4DollarSign => Some(("4", "$")),
            Self::Keyboard5PercentSign => Some(("5", "%")),
            Self::Keyboard6Carret => Some(("6", "^")),
            Self::Keyboard7Ampersand => Some(("7", "&")),
            Self::Keyboard8Asterisk => Some(("8", "*")),
            Self::Keyboard9LeftParentheses => Some(("9", "(")),
            Self::Keyboard0RightParentheses => Some(("0", ")")),
            Self::KeyboardReturnEnter => Some(("\n", "")),
            Self::KeyboardTab => Some(("\t", "")),
            Self::KeyboardSpacebar => Some((" ", "")),
            Self::KeyboardHyphenUnderscore => Some(("-", "_")),
            Self::KeyboardEqualPlus => Some(("=", "+")),
            Self::KeyboardLeftBracket => Some(("[", "{")),
            Self::KeyboardRigthBracket => Some(("]", "}")),
            Self::KeyboardBackslashPipe => Some(("\\", "|")),
            Self::KeyboardNonUSPoundTilda => Some(("#", "~")),
            Self::KeyboardSemiNonColon => Some((";", ":")),
            Self::KeyboardSingleDoubleQuotes => Some(("\'", "\"")),
            Self::KeyboardGraveAccentTilde => Some(("`", "~")),
            Self::KeyboardCommnaLessThen => Some((",", "<")),
            Self::KeyboardPeriodGreaterThen => Some((".", ">")),
            Self::KeyboardForwardSlashQuestionMark => Some(("/", "?")),
            Self::KeypadFowardSlash => Some(("/", "")),
            Self::KeypadAsterisk => Some(("*", "")),
            Self::KeypadMinus => Some(("-", "")),
            Self::KeypadPlus => Some(("+", "")),
            Self::KeypadEnter => Some(("\n", "")),
            Self::Keypad1End => Some(("1", "")),
            Self::Keypad2DownArrow => Some(("2", "")),
            Self::Keypad3PageDn => Some(("3", "")),
            Self::Keypad4LeftArrow => Some(("4", "")),
            Self::Keypad5 => Some(("5", "")),
            Self::Keypad6RightArrow => Some(("6", "")),
            Self::Keypad7Home => Some(("7", "")),
            Self::Keypad8UpArrow => Some(("8", "")),
            Self::Keypad9PageUp => Some(("9", "")),
            Self::Keypad0Insert => Some(("0", "")),
            Self::KeypadPeriodDelete => Some((".", "")),
            Self::KeyboardNonUSBackwardSlashPipe => Some(("\\", "|")),
            Self::KeypadEqual => Some(("=", "")),
            Self::KeypadComma => Some((",", "")),
            Self::KeypadEqualSign => Some(("=", "")),
            Self::KeypadLeftParenthesis => Some(("(", "")),
            Self::KeypadRightParenthesis => Some((")", "")),
            Self::KeypadLeftCurlyBracket => Some(("{", "")),
            Self::KeypadRightCurlyBracket => Some(("}", "")),
            Self::KeypadTab => Some(("\t", "")),
            Self::KeypadCarret => Some(("^", "")),
            Self::KeypadPrecentSign => Some(("%", "")),
            Self::KeypadLessThen => Some(("<", "")),
            Self::KeypadGreaterThen => Some((">", "")),
            Self::KeypadAppersand => Some(("&", "")),
            Self::Keypad2Appersand => Some(("&", "&")),
            Self::KeypadPipe => Some(("|", "")),
            Self::Keypad2Pipe => Some(("|", "|")),
            Self::KeypadColon => Some((":", "")),
            Self::KeypadPound => Some(("#", "")),
            Self::KeypadSpace => Some((" ", "")),
            Self::KeypadAt => Some(("@", "")),
            Self::KeypadExclamtionPoint => Some(("!", "")),
            Self::KeypadPlusMinus => Some(("+", "-")),
            _ => None,
        }
    }
}
impl KeyboardUsage {
    pub fn to_string(&self) -> String {
        match self {
            Self::Reserved00_00 => "Reserved | No Event".to_string(),
            Self::KeyboardErrorRollOver => "".to_string(),
            Self::KeyboardPOSTFail => "".to_string(),
            Self::KeyboardErrorUndefined => "".to_string(),
            Self::KeyboardaA => "a or A".to_string(),
            Self::KeyboardbB => "b or B".to_string(),
            Self::KeyboardcC => "c or C".to_string(),
            Self::KeyboarddD => "d or D".to_string(),
            Self::KeyboardeE => "e or E".to_string(),
            Self::KeyboardfF => "f or F".to_string(),
            Self::KeyboardgG => "g or G".to_string(),
            Self::KeyboardhH => "h or H".to_string(),
            Self::KeyboardiI => "i or I".to_string(),
            Self::KeyboardjJ => "j or J".to_string(),
            Self::KeyboardkK => "k or K".to_string(),
            Self::KeyboardlL => "l or L".to_string(),
            Self::KeyboardmM => "m or M".to_string(),
            Self::KeyboardnN => "n or N".to_string(),
            Self::KeyboardoO => "o or O".to_string(),
            Self::KeyboardpP => "p or P".to_string(),
            Self::KeyboardqQ => "q or Q".to_string(),
            Self::KeyboardrR => "r or R".to_string(),
            Self::KeyboardsS => "s or S".to_string(),
            Self::KeyboardtT => "t or T".to_string(),
            Self::KeyboarduU => "u or U".to_string(),
            Self::KeyboardvV => "v or V".to_string(),
            Self::KeyboardwW => "w or W".to_string(),
            Self::KeyboardxX => "x or X".to_string(),
            Self::KeyboardyY => "y or Y".to_string(),
            Self::KeyboardzZ => "z or Z".to_string(),
            Self::Keyboard1ExclamationPoint => "1 or !".to_string(),
            Self::Keyboard2At => "2 or @".to_string(),
            Self::Keyboard3Pound => "3 or #".to_string(),
            Self::Keyboard4DollarSign => "4 or $".to_string(),
            Self::Keyboard5PercentSign => "5 or %".to_string(),
            Self::Keyboard6Carret => "6 or ^".to_string(),
            Self::Keyboard7Ampersand => "7 or &".to_string(),
            Self::Keyboard8Asterisk => "8 or *".to_string(),
            Self::Keyboard9LeftParentheses => "9 or (".to_string(),
            Self::Keyboard0RightParentheses => "0 or )".to_string(),
            Self::KeyboardReturnEnter => "".to_string(),
            Self::KeyboardEsc => "".to_string(),
            Self::KeyboardDeleteBackspace => "".to_string(),
            Self::KeyboardTab => "Tab Space".to_string(),
            Self::KeyboardSpacebar => "Space".to_string(),
            Self::KeyboardHyphenUnderscore => "- or _".to_string(),
            Self::KeyboardEqualPlus => "= or +".to_string(),
            Self::KeyboardLeftBracket => "[ or {".to_string(),
            Self::KeyboardRigthBracket => "] or }".to_string(),
            Self::KeyboardBackslashPipe => "\\ or |".to_string(),
            Self::KeyboardNonUSPoundTilda => "# or ~".to_string(),
            Self::KeyboardSemiNonColon => "; or :".to_string(),
            Self::KeyboardSingleDoubleQuotes => "\' or \"".to_string(),
            Self::KeyboardGraveAccentTilde => "` or ~".to_string(),
            Self::KeyboardCommnaLessThen => ", or <".to_string(),
            Self::KeyboardPeriodGreaterThen => ". or >".to_string(),
            Self::KeyboardForwardSlashQuestionMark => "/ or ?".to_string(),
            Self::KeyboardCapsLock => "CapsLock".to_string(),
            Self::KeyboardF1 => "Function 1".to_string(),
            Self::KeyboardF2 => "Function 2".to_string(),
            Self::KeyboardF3 => "Function 3".to_string(),
            Self::KeyboardF4 => "Function 4".to_string(),
            Self::KeyboardF5 => "Function 5".to_string(),
            Self::KeyboardF6 => "Function 6".to_string(),
            Self::KeyboardF7 => "Function 7".to_string(),
            Self::KeyboardF8 => "Function 8".to_string(),
            Self::KeyboardF9 => "Function 9".to_string(),
            Self::KeyboardF10 => "Function 10".to_string(),
            Self::KeyboardF11 => "Function 11".to_string(),
            Self::KeyboardF12 => "Function 12".to_string(),
            Self::KeyboardPrintScreen => "PrintScreen".to_string(),
            Self::KeyboardScrollLock => "ScrollLock".to_string(),
            Self::KeyboardPause => "Pause".to_string(),
            Self::KeyboardInsert => "Insert".to_string(),
            Self::KeyboardHome => "Home".to_string(),
            Self::KeyboardPageUp => "PageUp".to_string(),
            Self::KeyboardDeleteFoward => "Delete".to_string(),
            Self::KeyboardEnd => "End".to_string(),
            Self::KeyboardPageDown => "PageDown".to_string(),
            Self::KeyboardRightArrow => "Right Arrow".to_string(),
            Self::KeyboardLeftArrow => "Left Arrow".to_string(),
            Self::KeyboardDownArrow => "Down Arrow".to_string(),
            Self::KeyboardUpArrow => "Up Arrow".to_string(),
            Self::KeypadNumLockClear => "NumLock or Clear".to_string(),
            Self::KeypadFowardSlash => "/".to_string(),
            Self::KeypadAsterisk => "*".to_string(),
            Self::KeypadMinus => "-".to_string(),
            Self::KeypadPlus => "+".to_string(),
            Self::KeypadEnter => "Enter".to_string(),
            Self::Keypad1End => "1 or End".to_string(),
            Self::Keypad2DownArrow => "2 or Down Arrow".to_string(),
            Self::Keypad3PageDn => "3 or PageDown".to_string(),
            Self::Keypad4LeftArrow => "4 or Left Arrow".to_string(),
            Self::Keypad5 => "5".to_string(),
            Self::Keypad6RightArrow => "6 or Right Arrow".to_string(),
            Self::Keypad7Home => "7 or Home".to_string(),
            Self::Keypad8UpArrow => "8 or Up Arrow".to_string(),
            Self::Keypad9PageUp => "9 or PageUp".to_string(),
            Self::Keypad0Insert => "0 or Insert".to_string(),
            Self::KeypadPeriodDelete => ". or Delete".to_string(),
            Self::KeyboardNonUSBackwardSlashPipe => "\\ or |".to_string(),
            Self::KeyboardApplication => "Application".to_string(),
            Self::KeyboardPower => "Power".to_string(),
            Self::KeypadEqual => "=".to_string(),
            Self::KeyboardF13 => "Function 13".to_string(),
            Self::KeyboardF14 => "Function 14".to_string(),
            Self::KeyboardF15 => "Function 15".to_string(),
            Self::KeyboardF16 => "Function 16".to_string(),
            Self::KeyboardF17 => "Function 17".to_string(),
            Self::KeyboardF18 => "Function 18".to_string(),
            Self::KeyboardF19 => "Function 19".to_string(),
            Self::KeyboardF20 => "Function 20".to_string(),
            Self::KeyboardF21 => "Function 21".to_string(),
            Self::KeyboardF22 => "Function 22".to_string(),
            Self::KeyboardF23 => "Function 23".to_string(),
            Self::KeyboardF24 => "Function 24".to_string(),
            Self::KeyboardExecute => "Execute".to_string(),
            Self::KeyboardHelp => "Help".to_string(),
            Self::KeyboardMenu => "Menu".to_string(),
            Self::KeyboardSelect => "Select".to_string(),
            Self::KeyboardStop => "Stop".to_string(),
            Self::KeyboardAgain => "Again".to_string(),
            Self::KeyboardUndo => "Undo".to_string(),
            Self::KeyboardCut => "Cut".to_string(),
            Self::KeyboardCopy => "Copy".to_string(),
            Self::KeyboardPaste => "Paste".to_string(),
            Self::KeyboardFind => "Find".to_string(),
            Self::KeyboardMute => "Mute".to_string(),
            Self::KeyboardVolumeUp => "VolumeUp".to_string(),
            Self::KeyboardVolumeDown => "VolumeDown".to_string(),
            Self::KeyboardLockingCapsLock => "LockingCapsLock".to_string(),
            Self::KeyboardLockingNumLock => "LockingNumLock".to_string(),
            Self::KeyboardLockingScrollLock => "LockingScrollLock".to_string(),
            Self::KeypadComma => ",".to_string(),
            Self::KeypadEqualSign => "=".to_string(),
            //fix these
            Self::KeyboardInternational1 => "1".to_string(),
            Self::KeyboardInternational2 => "2".to_string(),
            Self::KeyboardInternational3 => "3".to_string(),
            Self::KeyboardInternational4 => "4".to_string(),
            Self::KeyboardInternational5 => "5".to_string(),
            Self::KeyboardInternational6 => "6".to_string(),
            Self::KeyboardInternational7 => "7".to_string(),
            Self::KeyboardInternational8 => "8".to_string(),
            Self::KeyboardInternational9 => "9".to_string(),
            Self::KeyboardLANG1 => "".to_string(),
            Self::KeyboardLANG2 => "".to_string(),
            Self::KeyboardLANG3 => "".to_string(),
            Self::KeyboardLANG4 => "".to_string(),
            Self::KeyboardLANG5 => "".to_string(),
            Self::KeyboardLANG6 => "".to_string(),
            Self::KeyboardLANG7 => "".to_string(),
            Self::KeyboardLANG8 => "".to_string(),
            Self::KeyboardLANG9 => "".to_string(),
            Self::KeyboardAlternateErase => "".to_string(),
            Self::KeyboardSysReqAttention => "".to_string(),
            Self::KeyboardCancel => "".to_string(),
            Self::KeyboardClear => "".to_string(),
            Self::KeyboardPrior => "".to_string(),
            Self::KeyboardReturn => "".to_string(),
            Self::KeyboardSeparator => "".to_string(),
            Self::KeyboardOut => "".to_string(),
            Self::KeyboardOper => "".to_string(),
            Self::KeyboardClearAgain => "".to_string(),
            Self::KeyboardCrSelProps => "".to_string(),
            Self::KeyboardExSel => "".to_string(),
            Self::ReservedA5_Af(u) => "".to_string(),
            Self::Keypad00 => "".to_string(),
            Self::Keypad000 => "".to_string(),
            Self::ThousandsSeparator => "".to_string(),
            Self::DecimalSeparator => "".to_string(),
            Self::CurrencyUnit => "".to_string(),
            Self::CurrencySubunit => "".to_string(),
            Self::KeypadLeftParenthesis => "".to_string(),
            Self::KeypadRightParenthesis => "".to_string(),
            Self::KeypadLeftCurlyBracket => "".to_string(),
            Self::KeypadRightCurlyBracket => "".to_string(),
            Self::KeypadTab => "".to_string(),
            Self::KeypadBackspace => "".to_string(),
            Self::KeypadA => "".to_string(),
            Self::KeypadB => "".to_string(),
            Self::KeypadC => "".to_string(),
            Self::KeypadD => "".to_string(),
            Self::KeypadE => "".to_string(),
            Self::KeypadF => "".to_string(),
            Self::KeypadXOR => "".to_string(),
            Self::KeypadCarret => "".to_string(),
            Self::KeypadPrecentSign => "".to_string(),
            Self::KeypadLessThen => "".to_string(),
            Self::KeypadGreaterThen => "".to_string(),
            Self::KeypadAppersand => "".to_string(),
            Self::Keypad2Appersand => "".to_string(),
            Self::KeypadPipe => "".to_string(),
            Self::Keypad2Pipe => "".to_string(),
            Self::KeypadColon => "".to_string(),
            Self::KeypadPound => "".to_string(),
            Self::KeypadSpace => "".to_string(),
            Self::KeypadAt => "".to_string(),
            Self::KeypadExclamtionPoint => "".to_string(),
            Self::KeypadMemoryStore => "".to_string(),
            Self::KeypadMemoryRecall => "".to_string(),
            Self::KeypadMemoryClear => "".to_string(),
            Self::KeypadMemoryAdd => "".to_string(),
            Self::KeypadMemorySubtract => "".to_string(),
            Self::KeypadMemoryMultiply => "".to_string(),
            Self::KeypadMemoryDivide => "".to_string(),
            Self::KeypadPlusMinus => "".to_string(),
            Self::KeypadClear => "".to_string(),
            Self::KeypadClearEntry => "".to_string(),
            Self::KeypadBinary => "".to_string(),
            Self::KeypadOctal => "".to_string(),
            Self::KeypadDecimal => "".to_string(),
            Self::KeypadHexadecimal => "".to_string(),
            Self::ReservedDE_DF(u) => "".to_string(),
            Self::KeyboardLeftControl => "".to_string(),
            Self::KeyboardLeftShift => "".to_string(),
            Self::KeyboardLeftAlt => "".to_string(),
            Self::KeyboardLeftGUI => "".to_string(),
            Self::KeyboardRightControl => "".to_string(),
            Self::KeyboardRightShift => "".to_string(),
            Self::KeyboardRightAlt => "".to_string(),
            Self::KeyboardRightGUI => "".to_string(),
            Self::ReservedE8_FFFF(u) => "".to_string(),
        }
    }
}
