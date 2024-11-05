//! Telephony Device Page (0x0B)
//!
//! This usage page defines the keytop and control usages for telephony devices. Note that in many cases usage definitions are intentionally vague, this is because it is assumed that the controls are interpreted by the telephone software application (PBX). For instance, one software implementation may allow the Park usage to hold the line open while waiting for the target number to go on-hook, while another implementation will allow the user to hang up and then ring the user back when the target number is available. Often recommendations are made so that users of USB telephones see consistent interfaces across multiple vendors, minimizing learning curves and frustration when dealing with new or multiple systems
mod telephony_device;
pub use telephony_device::TelephonyDeviceUsage;
