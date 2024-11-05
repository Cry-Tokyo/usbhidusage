//! Eye and Head Trackers Page (0x12)
//!
//! An eye tracker is a device designed to measure gaze point and eye position. When calibrated against a display device, an eye tracker is capable of returning coordinates on the screen the user is looking at. Using these coordinates it is possible to define mechanisms to interact with applications using eyes. A head tracker performs a similar role, except it tracks the orientation of the head against the calibrated screen and returns the corresponding coordinate. An eye tracker may be capable of tracking both head and eyes. This page has facilities to discover, control and read data from eye trackers and head trackers mounted on the monitor
mod eye_and_head_trackers;
pub use eye_and_head_trackers::EyeAndHeadTrackersUsage;
