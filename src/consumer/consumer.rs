#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Default)]
#[non_exhaustive]
#[repr(u16)]
pub enum ConsumerUsage {
    #[default]
    Undefined,
    ConsumerControl,
    NumericKeyPad,
    ProgrammableButtons,
    Microphone,
    Headphone,
    GraphicEqualizer,
    Reserved07_1F(u16),
    Plus10 = 0x20,
    Plus100,
    AM_PM,
    Reserved23_2F(u16),
    Power = 0x30,
    Reset,
    Sleep,
    SleepAfter,
    SleepMode,
    Illumination,
    FunctionButtons,
    Reserved37_3F(u16),
    Menu = 0x40,
    MenuPick,
    MenuUp,
    MenuDown,
    MenuLeft,
    MenuRight,
    MenuEscape,
    MenuValueIncrease,
    MenuValueDecrease,
    Reserved49_5F(u16),
    DataOnScreen = 0x60,
    ClosedCaption,
    ClosedCaptionSelect,
    VCR_TV,
    BroadcastMode,
    Snapshot,
    Still,
    Picture_in_PictureToggle,
    Picture_in_PictureSwap,
    RedMenuButton,
    GreenMenuButton,
    BlueMenuButton,
    YellowMenuButton,
    Aspect,
    _3DModeSelect,
    DisplayBrightnessIncrement,
    DisplayBrightnessDecrement,
    DisplayBrightness,
    DisplayBacklightToggle,
    DisplaySetBrightnesstoMinimum,
    DisplaySetBrightnesstoMaximum,
    DisplaySetAutoBrightness,
    CameraAccessEnabled,
    CameraAccessDisabled,
    CameraAccessToggle,
    KeyboardBrightnessIncrement,
    KeyboardBrightnessDecrement,
    KeyboardBacklightSetLevel,
    KeyboardBacklightOOC,
    KeyboardBacklightSetMinimum,
    KeyboardBacklightSetMaximum,
    KeyboardBacklightAuto,
    Selection,
    AssignSelection,
    ModeStep,
    RecallLast,
    EnterChannel,
    OrderMovie,
    Channel,
    MediaSelection,
    MediaSelectComputer,
    MediaSelectTV,
    MediaSelectWWW,
    MediaSelectDVD,
    MediaSelectTelephone,
    MediaSelectProgramGuide,
    MediaSelectVideoPhone,
    MediaSelectGames,
    MediaSelectMessages,
    MediaSelectCD,
    MediaSelectVCR,
    MediaSelectTuner,
    Quit,
    Help,
    MediaSelectTape,
    MediaSelectCable,
    MediaSelectSatellite,
    MediaSelectSecurity,
    MediaSelectHome,
    MediaSelectCall,
    ChannelIncrement,
    ChannelDecrement,
    MediaSelectSAP,
    Reserved9F,
    VCRPlus,
    Once,
    Daily,
    Weekly,
    Monthly,
    ReservedA5_AF(u16),
    Play = 0xB0,
    Pause,
    Record,
    FastForward,
    Rewind,
    ScanNextTrack,
    ScanPreviousTrack,
    Stop,
    Eject,
    RandomPlay,
    SelectDisc,
    EnterDisc,
    Repeat,
    Tracking,
    TrackNormal,
    SlowTracking,
    FrameForward,
    FrameBack,
    Mark,
    ClearMark,
    RepeatFromMark,
    ReturnToMark,
    SearchMarkForward,
    SearchMarkBackwards,
    CounterReset,
    ShowCounter,
    TrackingIncrement,
    TrackingDecrement,
    Stop_Eject,
    Play_Pause,
    Play_Skip,
    VoiceCommand,
    InvokeCaptureInterface,
    StartorStopGameRecording,
    HistoricalGameCapture,
    CaptureGameScreenshot,
    ShoworHideRecordingIndicator,
    StartorStopMicrophoneCapture,
    StartorStopCameraCapture,
    StartorStopGameBroadcast,
    StartorStopVoiceDictationSession,
    Invoke_DismissEmojiPicker,
    ReservedDA_DF(u16),
    Volume = 0xE0,
    Balance,
    Mute,
    Bass,
    Treble,
    BassBoost,
    SurroundMode,
    Loudness,
    MPX,
    VolumeIncrement,
    VolumeDecrement,
    ReservedEB_EF(u16),
    SpeedSelect = 0xF0,
    PlaybackSpeed,
    StandardPlay,
    LongPlay,
    ExtendedPlay,
    Slow,
    ReservedF6_FF(u16),
    FanEnable = 0x100,
    FanSpeed,
    LightEnable,
    LightIlluminationLevel,
    ClimateControlEnable,
    RoomTemperature,
    SecurityEnable,
    FireAlarm,
    PoliceAlarm,
    Proximity,
    Motion,
    DuressAlarm,
    HoldupAlarm,
    MedicalAlarm,
    Reserved10E_14F(u16),
    BalanceRight = 0x150,
    BalanceLeft,
    BassIncrement,
    BassDecrement,
    TrebleIncrement,
    TrebleDecrement,
    Reserved156_15F(u16),
    SpeakerSystem = 0x160,
    ChannelLeft,
    ChannelRight,
    ChannelCenter,
    ChannelFront,
    ChannelCenterFront,
    ChannelSide,
    ChannelSurround,
    ChannelLowFrequencyEnhancement,
    ChannelTop,
    ChannelUnknown,
    Reserved16B_16F(u16),
    Sub_channel = 0x170,
    Sub_channelIncrement,
    Sub_channelDecrement,
    AlternateAudioIncrement,
    AlternateAudioDecrement,
    Reserved175_17F(u16),
    ApplicationLaunchButtons = 0x180,
    ALLaunchButtonConfigurationTool,
    ALProgrammableButtonConfiguration,
    ALConsumerControlConfiguration,
    ALWordProcessor,
    ALTextEditor,
    ALSpreadsheet,
    ALGraphicsEditor,
    ALPresentationApp,
    ALDatabaseApp,
    ALEmailReader,
    ALNewsreader,
    ALVoicemail,
    ALContacts_AddressBook,
    ALCalendar_Schedule,
    ALTask_ProjectManager,
    ALLog_Journal_Timecard,
    ALCheckbook_Finance,
    ALCalculator,
    ALA_VCapture_Playback,
    ALLocalMachineBrowser,
    ALLAN_WANBrowser,
    ALInternetBrowser,
    ALRemoteNetworking_ISPConnect,
    ALNetworkConference,
    ALNetworkChat,
    ALTelephony_Dialer,
    ALLogon,
    ALLogoff,
    ALLogon_Logoff,
    ALTerminalLock_Screensaver,
    ALControlPanel,
    ALCommandLineProcessor_Run,
    ALProcess_TaskManager,
    ALSelectTask_Application,
    ALNextTask_Application,
    ALPreviousTask_Application,
    ALPreemptiveHaltTask_Application,
    ALIntegratedHelpCenter,
    ALDocuments,
    ALThesaurus,
    ALDictionary,
    ALDesktop,
    ALSpellCheck,
    ALGrammarCheck,
    ALWirelessStatus,
    ALKeyboardLayout,
    ALVirusProtection,
    ALEncryption,
    ALScreenSaver,
    ALAlarms,
    ALClock,
    ALFileBrowser,
    ALPowerStatus,
    ALImageBrowser,
    ALAudioBrowser,
    ALMovieBrowser,
    ALDigitalRightsManager,
    ALDigitalWallet,
    Reserved1BB,
    ALInstantMessaging,
    ALOEMFeatures_Tips_TutorialBrowser,
    ALOEMHelp,
    ALOnlineCommunity,
    ALEntertainmentContentBrowser,
    ALOnlineShoppingBrowser,
    ALSmartCardInformation_Help,
    ALMarketMonitor_FinanceBrowser,
    ALCustomizedCorporateNewsBrowser,
    ALOnlineActivityBrowser,
    ALResearch_SearchBrowser,
    ALAudioPlayer,
    ALMessageStatus,
    ALContactSync,
    ALNavigation,
    ALContext_awareDesktopAssistant,
    Reserved1CC_1FF(u16),
    GenericGUIApplicationControls = 0x200,
    ACNew,
    ACOpen,
    ACClose,
    ACExit,
    ACMaximize,
    ACMinimize,
    ACSave,
    ACPrint,
    ACProperties,
    Reserved20A_219(u16),
    ACUndo = 0x21A,
    ACCopy,
    ACCut,
    ACPaste,
    ACSelectAll,
    ACFind,
    ACFindandReplace,
    ACSearch,
    ACGoTo,
    ACHome,
    ACBack,
    ACForward,
    ACStop,
    ACRefresh,
    ACPreviousLink,
    ACNextLink,
    ACBookmarks,
    ACHistory,
    ACSubscriptions,
    ACZoomIn,
    ACZoomOut,
    ACZoom,
    ACFullScreenView,
    ACNormalView,
    ACViewToggle,
    ACScrollUp,
    ACScrollDown,
    ACScroll,
    ACPanLeft,
    ACPanRight,
    ACPan,
    ACNewWindow,
    ACTileHorizontally,
    ACTileVertically,
    ACFormat,
    ACEdit,
    ACBold,
    ACItalics,
    ACUnderline,
    ACStrikethrough,
    ACSubscript,
    ACSuperscript,
    ACAllCaps,
    ACRotate,
    ACResize,
    ACFlipHorizontal,
    ACFlipVertical,
    ACMirrorHorizontal,
    ACMirrorVertical,
    ACFontSelect,
    ACFontColor,
    ACFontSize,
    ACJustifyLeft,
    ACJustifyCenterH,
    ACJustifyRight,
    ACJustifyBlockH,
    ACJustifyTop,
    ACJustifyCenterV,
    ACJustifyBottom,
    ACJustifyBlockV,
    ACIndentDecrease,
    ACIndentIncrease,
    ACNumberedList,
    ACRestartNumbering,
    ACBulletedList,
    ACPromote,
    ACDemote,
    ACYes,
    ACNo,
    ACCancel,
    ACCatalog,
    ACBuy_Checkout,
    ACAddtoCart,
    ACExpand,
    ACExpandAll,
    ACCollapse,
    ACCollapseAll,
    ACPrintPreview,
    ACPasteSpecial,
    ACInsertMode,
    ACDelete,
    ACLock,
    ACUnlock,
    ACProtect,
    ACUnprotect,
    ACAttachComment,
    ACDeleteComment,
    ACViewComment,
    ACSelectWord,
    ACSelectSentence,
    ACSelectParagraph,
    ACSelectColumn,
    ACSelectRow,
    ACSelectTable,
    ACSelectObject,
    ACRedo_Repeat,
    ACSort,
    ACSortAscending,
    ACSortDescending,
    ACFilter,
    ACSetClock,
    ACViewClock,
    ACSelectTimeZone,
    ACEditTimeZones,
    ACSetAlarm,
    ACClearAlarm,
    ACSnoozeAlarm,
    ACResetAlarm,
    ACSynchronize,
    ACSend_Receive,
    ACSendTo,
    ACReply,
    ACReplyAll,
    ACForwardMsg,
    ACSend,
    ACAttachFile,
    ACUpload,
    ACDownload_SaveTargetAs,
    ACSetBorders,
    ACInsertRow,
    ACInsertColumn,
    ACInsertFile,
    ACInsertPicture,
    ACInsertObject,
    ACInsertSymbol,
    ACSaveandClose,
    ACRename,
    ACMerge,
    ACSplit,
    ACDisributeHorizontally,
    ACDistributeVertically,
    ACNextKeyboardLayoutSelect,
    ACNavigationGuidance,
    ACDesktopShowAllWindows,
    ACSoftKeyLeft,
    ACSoftKeyRight,
    ACDesktopShowAllApplications,
    Reserved2A3_2AF(u16),
    ACIdleKeepAlive = 0x2B0,
    Reserved2B1_2BF(u16),
    ExtendedKeyboardAttributesCollection = 0x2C0,
    KeyboardFormFactor,
    KeyboardKeyType,
    KeyboardPhysicalLayout,
    Vendor_SpecificKeyboardPhysicalLayout,
    KeyboardIETFLanguageTagIndex,
    ImplementedKeyboardInputAssistControls,
    KeyboardInputAssistPrevious,
    KeyboardInputAssistNext,
    KeyboardInputAssistPreviousGroup,
    KeyboardInputAssistNextGroup,
    KeyboardInputAssistAccept,
    KeyboardInputAssistCancel,
    Reserved2CD_2CF(u16),
    PrivacyScreenToggle = 0x2D0,
    PrivacyScreenLevelDecrement,
    PrivacyScreenLevelIncrement,
    PrivacyScreenLevelMinimum,
    PrivacyScreenLevelMaximum,
    Reserved2D5_4FF(u16),
    ContactEdited = 0x500,
    ContactAdded,
    ContactRecordActive,
    ContactIndex,
    ContactNickname,
    ContactFirstName,
    ContactLastName,
    ContactFullName,
    ContactPhoneNumberPersonal,
    ContactPhoneNumberBusiness,
    ContactPhoneNumberMobile,
    ContactPhoneNumberPager,
    ContactPhoneNumberFax,
    ContactPhoneNumberOther,
    ContactEmailPersonal,
    ContactEmailBusiness,
    ContactEmailOther,
    ContactEmailMain,
    ContactSpeedDialNumber,
    ContactStatusFlag,
    ContactMisc,
    Reserved515_FFFF(u16),
}
impl<T> From<T> for ConsumerUsage
where
    T: TryInto<u16>,
{
    fn from(value: T) -> Self {
        let value = value.try_into().unwrap_or(0);
        match value {
            0 => Self::Undefined,
            1 => Self::ConsumerControl,
            2 => Self::NumericKeyPad,
            3 => Self::ProgrammableButtons,
            4 => Self::Microphone,
            5 => Self::Headphone,
            6 => Self::GraphicEqualizer,
            7..32 => Self::Reserved07_1F(value),
            32 => Self::Plus10,
            33 => Self::Plus100,
            34 => Self::AM_PM,
            35..48 => Self::Reserved23_2F(value),
            48 => Self::Power,
            49 => Self::Reset,
            50 => Self::Sleep,
            51 => Self::SleepAfter,
            52 => Self::SleepMode,
            53 => Self::Illumination,
            54 => Self::FunctionButtons,
            55..64 => Self::Reserved37_3F(value),
            64 => Self::Menu,
            65 => Self::MenuPick,
            66 => Self::MenuUp,
            67 => Self::MenuDown,
            68 => Self::MenuLeft,
            69 => Self::MenuRight,
            70 => Self::MenuEscape,
            71 => Self::MenuValueIncrease,
            72 => Self::MenuValueDecrease,
            73..96 => Self::Reserved49_5F(value),
            96 => Self::DataOnScreen,
            97 => Self::ClosedCaption,
            98 => Self::ClosedCaptionSelect,
            99 => Self::VCR_TV,
            100 => Self::BroadcastMode,
            101 => Self::Snapshot,
            102 => Self::Still,
            103 => Self::Picture_in_PictureToggle,
            104 => Self::Picture_in_PictureSwap,
            105 => Self::RedMenuButton,
            106 => Self::GreenMenuButton,
            107 => Self::BlueMenuButton,
            108 => Self::YellowMenuButton,
            109 => Self::Aspect,
            110 => Self::_3DModeSelect,
            111 => Self::DisplayBrightnessIncrement,
            112 => Self::DisplayBrightnessDecrement,
            113 => Self::DisplayBrightness,
            114 => Self::DisplayBacklightToggle,
            115 => Self::DisplaySetBrightnesstoMinimum,
            116 => Self::DisplaySetBrightnesstoMaximum,
            117 => Self::DisplaySetAutoBrightness,
            118 => Self::CameraAccessEnabled,
            119 => Self::CameraAccessDisabled,
            120 => Self::CameraAccessToggle,
            121 => Self::KeyboardBrightnessIncrement,
            122 => Self::KeyboardBrightnessDecrement,
            123 => Self::KeyboardBacklightSetLevel,
            124 => Self::KeyboardBacklightOOC,
            125 => Self::KeyboardBacklightSetMinimum,
            126 => Self::KeyboardBacklightSetMaximum,
            127 => Self::KeyboardBacklightAuto,
            128 => Self::Selection,
            129 => Self::AssignSelection,
            130 => Self::ModeStep,
            131 => Self::RecallLast,
            132 => Self::EnterChannel,
            133 => Self::OrderMovie,
            134 => Self::Channel,
            135 => Self::MediaSelection,
            136 => Self::MediaSelectComputer,
            137 => Self::MediaSelectTV,
            138 => Self::MediaSelectWWW,
            139 => Self::MediaSelectDVD,
            140 => Self::MediaSelectTelephone,
            141 => Self::MediaSelectProgramGuide,
            142 => Self::MediaSelectVideoPhone,
            143 => Self::MediaSelectGames,
            144 => Self::MediaSelectMessages,
            145 => Self::MediaSelectCD,
            146 => Self::MediaSelectVCR,
            147 => Self::MediaSelectTuner,
            148 => Self::Quit,
            149 => Self::Help,
            150 => Self::MediaSelectTape,
            151 => Self::MediaSelectCable,
            152 => Self::MediaSelectSatellite,
            153 => Self::MediaSelectSecurity,
            154 => Self::MediaSelectHome,
            155 => Self::MediaSelectCall,
            156 => Self::ChannelIncrement,
            157 => Self::ChannelDecrement,
            158 => Self::MediaSelectSAP,
            159 => Self::Reserved9F,
            160 => Self::VCRPlus,
            161 => Self::Once,
            162 => Self::Daily,
            163 => Self::Weekly,
            164 => Self::Monthly,
            165..176 => Self::ReservedA5_AF(value),
            176 => Self::Play,
            177 => Self::Pause,
            178 => Self::Record,
            179 => Self::FastForward,
            180 => Self::Rewind,
            181 => Self::ScanNextTrack,
            182 => Self::ScanPreviousTrack,
            183 => Self::Stop,
            184 => Self::Eject,
            185 => Self::RandomPlay,
            186 => Self::SelectDisc,
            187 => Self::EnterDisc,
            188 => Self::Repeat,
            189 => Self::Tracking,
            190 => Self::TrackNormal,
            191 => Self::SlowTracking,
            192 => Self::FrameForward,
            193 => Self::FrameBack,
            194 => Self::Mark,
            195 => Self::ClearMark,
            196 => Self::RepeatFromMark,
            197 => Self::ReturnToMark,
            198 => Self::SearchMarkForward,
            199 => Self::SearchMarkBackwards,
            200 => Self::CounterReset,
            201 => Self::ShowCounter,
            202 => Self::TrackingIncrement,
            203 => Self::TrackingDecrement,
            204 => Self::Stop_Eject,
            205 => Self::Play_Pause,
            206 => Self::Play_Skip,
            207 => Self::VoiceCommand,
            208 => Self::InvokeCaptureInterface,
            209 => Self::StartorStopGameRecording,
            210 => Self::HistoricalGameCapture,
            211 => Self::CaptureGameScreenshot,
            212 => Self::ShoworHideRecordingIndicator,
            213 => Self::StartorStopMicrophoneCapture,
            214 => Self::StartorStopCameraCapture,
            215 => Self::StartorStopGameBroadcast,
            216 => Self::StartorStopVoiceDictationSession,
            217 => Self::Invoke_DismissEmojiPicker,
            218..224 => Self::ReservedDA_DF(value),
            224 => Self::Volume,
            225 => Self::Balance,
            226 => Self::Mute,
            227 => Self::Bass,
            228 => Self::Treble,
            229 => Self::BassBoost,
            230 => Self::SurroundMode,
            231 => Self::Loudness,
            232 => Self::MPX,
            233 => Self::VolumeIncrement,
            234 => Self::VolumeDecrement,
            235..240 => Self::ReservedEB_EF(value),
            240 => Self::SpeedSelect,
            241 => Self::PlaybackSpeed,
            242 => Self::StandardPlay,
            243 => Self::LongPlay,
            244 => Self::ExtendedPlay,
            245 => Self::Slow,
            246..256 => Self::ReservedF6_FF(value),
            256 => Self::FanEnable,
            257 => Self::FanSpeed,
            258 => Self::LightEnable,
            259 => Self::LightIlluminationLevel,
            260 => Self::ClimateControlEnable,
            261 => Self::RoomTemperature,
            262 => Self::SecurityEnable,
            263 => Self::FireAlarm,
            264 => Self::PoliceAlarm,
            265 => Self::Proximity,
            266 => Self::Motion,
            267 => Self::DuressAlarm,
            268 => Self::HoldupAlarm,
            269 => Self::MedicalAlarm,
            270..336 => Self::Reserved10E_14F(value),
            336 => Self::BalanceRight,
            337 => Self::BalanceLeft,
            338 => Self::BassIncrement,
            339 => Self::BassDecrement,
            340 => Self::TrebleIncrement,
            341 => Self::TrebleDecrement,
            342..352 => Self::Reserved156_15F(value),
            352 => Self::SpeakerSystem,
            353 => Self::ChannelLeft,
            354 => Self::ChannelRight,
            355 => Self::ChannelCenter,
            356 => Self::ChannelFront,
            357 => Self::ChannelCenterFront,
            358 => Self::ChannelSide,
            359 => Self::ChannelSurround,
            360 => Self::ChannelLowFrequencyEnhancement,
            361 => Self::ChannelTop,
            362 => Self::ChannelUnknown,
            363..368 => Self::Reserved16B_16F(value),
            368 => Self::Sub_channel,
            369 => Self::Sub_channelIncrement,
            370 => Self::Sub_channelDecrement,
            371 => Self::AlternateAudioIncrement,
            372 => Self::AlternateAudioDecrement,
            373..384 => Self::Reserved175_17F(value),
            384 => Self::ApplicationLaunchButtons,
            385 => Self::ALLaunchButtonConfigurationTool,
            386 => Self::ALProgrammableButtonConfiguration,
            387 => Self::ALConsumerControlConfiguration,
            388 => Self::ALWordProcessor,
            389 => Self::ALTextEditor,
            390 => Self::ALSpreadsheet,
            391 => Self::ALGraphicsEditor,
            392 => Self::ALPresentationApp,
            393 => Self::ALDatabaseApp,
            394 => Self::ALEmailReader,
            395 => Self::ALNewsreader,
            396 => Self::ALVoicemail,
            397 => Self::ALContacts_AddressBook,
            398 => Self::ALCalendar_Schedule,
            399 => Self::ALTask_ProjectManager,
            400 => Self::ALLog_Journal_Timecard,
            401 => Self::ALCheckbook_Finance,
            402 => Self::ALCalculator,
            403 => Self::ALA_VCapture_Playback,
            404 => Self::ALLocalMachineBrowser,
            405 => Self::ALLAN_WANBrowser,
            406 => Self::ALInternetBrowser,
            407 => Self::ALRemoteNetworking_ISPConnect,
            408 => Self::ALNetworkConference,
            409 => Self::ALNetworkChat,
            410 => Self::ALTelephony_Dialer,
            411 => Self::ALLogon,
            412 => Self::ALLogoff,
            413 => Self::ALLogon_Logoff,
            414 => Self::ALTerminalLock_Screensaver,
            415 => Self::ALControlPanel,
            416 => Self::ALCommandLineProcessor_Run,
            417 => Self::ALProcess_TaskManager,
            418 => Self::ALSelectTask_Application,
            419 => Self::ALNextTask_Application,
            420 => Self::ALPreviousTask_Application,
            421 => Self::ALPreemptiveHaltTask_Application,
            422 => Self::ALIntegratedHelpCenter,
            423 => Self::ALDocuments,
            424 => Self::ALThesaurus,
            425 => Self::ALDictionary,
            426 => Self::ALDesktop,
            427 => Self::ALSpellCheck,
            428 => Self::ALGrammarCheck,
            429 => Self::ALWirelessStatus,
            430 => Self::ALKeyboardLayout,
            431 => Self::ALVirusProtection,
            432 => Self::ALEncryption,
            433 => Self::ALScreenSaver,
            434 => Self::ALAlarms,
            435 => Self::ALClock,
            436 => Self::ALFileBrowser,
            437 => Self::ALPowerStatus,
            438 => Self::ALImageBrowser,
            439 => Self::ALAudioBrowser,
            440 => Self::ALMovieBrowser,
            441 => Self::ALDigitalRightsManager,
            442 => Self::ALDigitalWallet,
            443 => Self::Reserved1BB,
            444 => Self::ALInstantMessaging,
            445 => Self::ALOEMFeatures_Tips_TutorialBrowser,
            446 => Self::ALOEMHelp,
            447 => Self::ALOnlineCommunity,
            448 => Self::ALEntertainmentContentBrowser,
            449 => Self::ALOnlineShoppingBrowser,
            450 => Self::ALSmartCardInformation_Help,
            451 => Self::ALMarketMonitor_FinanceBrowser,
            452 => Self::ALCustomizedCorporateNewsBrowser,
            453 => Self::ALOnlineActivityBrowser,
            454 => Self::ALResearch_SearchBrowser,
            455 => Self::ALAudioPlayer,
            456 => Self::ALMessageStatus,
            457 => Self::ALContactSync,
            458 => Self::ALNavigation,
            459 => Self::ALContext_awareDesktopAssistant,
            460..512 => Self::Reserved1CC_1FF(value),
            512 => Self::GenericGUIApplicationControls,
            513 => Self::ACNew,
            514 => Self::ACOpen,
            515 => Self::ACClose,
            516 => Self::ACExit,
            517 => Self::ACMaximize,
            518 => Self::ACMinimize,
            519 => Self::ACSave,
            520 => Self::ACPrint,
            521 => Self::ACProperties,
            522..538 => Self::Reserved20A_219(value),
            538 => Self::ACUndo,
            539 => Self::ACCopy,
            540 => Self::ACCut,
            541 => Self::ACPaste,
            542 => Self::ACSelectAll,
            543 => Self::ACFind,
            544 => Self::ACFindandReplace,
            545 => Self::ACSearch,
            546 => Self::ACGoTo,
            547 => Self::ACHome,
            548 => Self::ACBack,
            549 => Self::ACForward,
            550 => Self::ACStop,
            551 => Self::ACRefresh,
            552 => Self::ACPreviousLink,
            553 => Self::ACNextLink,
            554 => Self::ACBookmarks,
            555 => Self::ACHistory,
            556 => Self::ACSubscriptions,
            557 => Self::ACZoomIn,
            558 => Self::ACZoomOut,
            559 => Self::ACZoom,
            560 => Self::ACFullScreenView,
            561 => Self::ACNormalView,
            562 => Self::ACViewToggle,
            563 => Self::ACScrollUp,
            564 => Self::ACScrollDown,
            565 => Self::ACScroll,
            566 => Self::ACPanLeft,
            567 => Self::ACPanRight,
            568 => Self::ACPan,
            569 => Self::ACNewWindow,
            570 => Self::ACTileHorizontally,
            571 => Self::ACTileVertically,
            572 => Self::ACFormat,
            573 => Self::ACEdit,
            574 => Self::ACBold,
            575 => Self::ACItalics,
            576 => Self::ACUnderline,
            577 => Self::ACStrikethrough,
            578 => Self::ACSubscript,
            579 => Self::ACSuperscript,
            580 => Self::ACAllCaps,
            581 => Self::ACRotate,
            582 => Self::ACResize,
            583 => Self::ACFlipHorizontal,
            584 => Self::ACFlipVertical,
            585 => Self::ACMirrorHorizontal,
            586 => Self::ACMirrorVertical,
            587 => Self::ACFontSelect,
            588 => Self::ACFontColor,
            589 => Self::ACFontSize,
            590 => Self::ACJustifyLeft,
            591 => Self::ACJustifyCenterH,
            592 => Self::ACJustifyRight,
            593 => Self::ACJustifyBlockH,
            594 => Self::ACJustifyTop,
            595 => Self::ACJustifyCenterV,
            596 => Self::ACJustifyBottom,
            597 => Self::ACJustifyBlockV,
            598 => Self::ACIndentDecrease,
            599 => Self::ACIndentIncrease,
            600 => Self::ACNumberedList,
            601 => Self::ACRestartNumbering,
            602 => Self::ACBulletedList,
            603 => Self::ACPromote,
            604 => Self::ACDemote,
            605 => Self::ACYes,
            606 => Self::ACNo,
            607 => Self::ACCancel,
            608 => Self::ACCatalog,
            609 => Self::ACBuy_Checkout,
            610 => Self::ACAddtoCart,
            611 => Self::ACExpand,
            612 => Self::ACExpandAll,
            613 => Self::ACCollapse,
            614 => Self::ACCollapseAll,
            615 => Self::ACPrintPreview,
            616 => Self::ACPasteSpecial,
            617 => Self::ACInsertMode,
            618 => Self::ACDelete,
            619 => Self::ACLock,
            620 => Self::ACUnlock,
            621 => Self::ACProtect,
            622 => Self::ACUnprotect,
            623 => Self::ACAttachComment,
            624 => Self::ACDeleteComment,
            625 => Self::ACViewComment,
            626 => Self::ACSelectWord,
            627 => Self::ACSelectSentence,
            628 => Self::ACSelectParagraph,
            629 => Self::ACSelectColumn,
            630 => Self::ACSelectRow,
            631 => Self::ACSelectTable,
            632 => Self::ACSelectObject,
            633 => Self::ACRedo_Repeat,
            634 => Self::ACSort,
            635 => Self::ACSortAscending,
            636 => Self::ACSortDescending,
            637 => Self::ACFilter,
            638 => Self::ACSetClock,
            639 => Self::ACViewClock,
            640 => Self::ACSelectTimeZone,
            641 => Self::ACEditTimeZones,
            642 => Self::ACSetAlarm,
            643 => Self::ACClearAlarm,
            644 => Self::ACSnoozeAlarm,
            645 => Self::ACResetAlarm,
            646 => Self::ACSynchronize,
            647 => Self::ACSend_Receive,
            648 => Self::ACSendTo,
            649 => Self::ACReply,
            650 => Self::ACReplyAll,
            651 => Self::ACForwardMsg,
            652 => Self::ACSend,
            653 => Self::ACAttachFile,
            654 => Self::ACUpload,
            655 => Self::ACDownload_SaveTargetAs,
            656 => Self::ACSetBorders,
            657 => Self::ACInsertRow,
            658 => Self::ACInsertColumn,
            659 => Self::ACInsertFile,
            660 => Self::ACInsertPicture,
            661 => Self::ACInsertObject,
            662 => Self::ACInsertSymbol,
            663 => Self::ACSaveandClose,
            664 => Self::ACRename,
            665 => Self::ACMerge,
            666 => Self::ACSplit,
            667 => Self::ACDisributeHorizontally,
            668 => Self::ACDistributeVertically,
            669 => Self::ACNextKeyboardLayoutSelect,
            670 => Self::ACNavigationGuidance,
            671 => Self::ACDesktopShowAllWindows,
            672 => Self::ACSoftKeyLeft,
            673 => Self::ACSoftKeyRight,
            674 => Self::ACDesktopShowAllApplications,
            675..688 => Self::Reserved2A3_2AF(value),
            688 => Self::ACIdleKeepAlive,
            689..704 => Self::Reserved2B1_2BF(value),
            704 => Self::ExtendedKeyboardAttributesCollection,
            705 => Self::KeyboardFormFactor,
            706 => Self::KeyboardKeyType,
            707 => Self::KeyboardPhysicalLayout,
            708 => Self::Vendor_SpecificKeyboardPhysicalLayout,
            709 => Self::KeyboardIETFLanguageTagIndex,
            710 => Self::ImplementedKeyboardInputAssistControls,
            711 => Self::KeyboardInputAssistPrevious,
            712 => Self::KeyboardInputAssistNext,
            713 => Self::KeyboardInputAssistPreviousGroup,
            714 => Self::KeyboardInputAssistNextGroup,
            715 => Self::KeyboardInputAssistAccept,
            716 => Self::KeyboardInputAssistCancel,
            717..720 => Self::Reserved2CD_2CF(value),
            720 => Self::PrivacyScreenToggle,
            721 => Self::PrivacyScreenLevelDecrement,
            722 => Self::PrivacyScreenLevelIncrement,
            723 => Self::PrivacyScreenLevelMinimum,
            724 => Self::PrivacyScreenLevelMaximum,
            725..1280 => Self::Reserved2D5_4FF(value),
            1280 => Self::ContactEdited,
            1281 => Self::ContactAdded,
            1282 => Self::ContactRecordActive,
            1283 => Self::ContactIndex,
            1284 => Self::ContactNickname,
            1285 => Self::ContactFirstName,
            1286 => Self::ContactLastName,
            1287 => Self::ContactFullName,
            1288 => Self::ContactPhoneNumberPersonal,
            1289 => Self::ContactPhoneNumberBusiness,
            1290 => Self::ContactPhoneNumberMobile,
            1291 => Self::ContactPhoneNumberPager,
            1292 => Self::ContactPhoneNumberFax,
            1293 => Self::ContactPhoneNumberOther,
            1294 => Self::ContactEmailPersonal,
            1295 => Self::ContactEmailBusiness,
            1296 => Self::ContactEmailOther,
            1297 => Self::ContactEmailMain,
            1298 => Self::ContactSpeedDialNumber,
            1299 => Self::ContactStatusFlag,
            1300 => Self::ContactMisc,
            1301..=65535 => Self::Reserved515_FFFF(value),
        }
    }
}
