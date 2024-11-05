//! Sensors Page (0x20)
//!
//! * The lowest-numbered IDs from 0x00 to 0xFF are Usages applied to Collections and represent sensor objects (may equate to sensor Categories or Types).
//! * The IDs from 0x0100 to 0x07FF are Usages applied to Properties and Data Fields. These are grouped by the sensor Category where the Usages are commonly employed, but this arrangement is arbitrary. Usages may be reported by any sensor (or more than one sensor) if it makes sense to do so. Properties and Data Fields can also apply to Collections within a Collection described by a Categories or Types Usage.
//! * The IDs from 0x0300 to 0x03FF and 0x0529 (timestamp) are commonly used with all Sensors.
//! * The IDs from 0x0800 to 0x0FFF are Selector Usages used with Properties or Data Fields that are Named Array enumerations. Selectors can also apply to Collections within a Collection described by a Categories or Types Usage.
//! * The IDs from 0x1000 to 0xEFFF are Properties or Data Fields from the 0x0100 to 0x0FFF range with Modifiers OR-ed in to the top 4 bits. Note: 0x0100 to 0x0FFF are the base usages without Modifiers
//! * The IDs from 0xF000 upward are reserved for proprietary use by vendors.
mod sensors;
pub use sensors::SensorUsage;
