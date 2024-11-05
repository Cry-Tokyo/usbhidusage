//! LED Page (0x08)
//!
//! An LED or indicator is implemented as an On/Off Control (OOC) using the Single button toggle mode, where a value of 1 will turn on the indicator, and a value of 0 will turn it off. The exceptions are described below.
mod led;
pub use led::LedUsage;
