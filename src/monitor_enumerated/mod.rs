//! Monitor Enumerated Page (0x81)
//!
//! To simplify the implementation of a monitor implementing VESA MCCS (Monitor Control Command Set) VCP (Virtual Control Panel) (or similiar), UsageIds for controls in VESA Virtual Controls Page (0x82) are the same as their VESA op-code counterparts. The returned VCP values for each control have context-specific meaning, varying for each control. Rather than assigning each control a seperate UsagePage (to prevent collisions) for its valid Sel values, each control defines a mapping between a Usage (Enum N ) and a definition. This is seen as a good compromise between the requirements of a HID descriptor, uniqueness of UsageIds, and utilizing the existing values of VESA MCCS VCP.
mod monitor_enumerated;
pub use monitor_enumerated::MonitorEnumeratedUsage;
