//! Monitor Page (0x80)
//!
//! This page and VESA Virtual Controls Page (0x82) define Usages for the management and control of system-attached monitors. This is the primary page for generic monitor control, with VESA Virtual Controls Page (0x82) giving specific support for the VESA MCCS VCP. Note: VESA MCCS VCP controls are defined in their own UsagePage so they can have the same UsageIds as the externally defined VCP op-codes. If included in this UsagePage, collisions may occur after additions to the MCCS. Note: At the time of writing (1998), USB 1.0 bandwidth (1.5Mbits) is insufficent for the actual transmission of video/audio data. It is only suitable for the transmission of management/control data.
mod monitor;
pub use monitor::MonitorUsage;
