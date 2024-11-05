//! Haptics Page (0x0E)
//!
//! The Physical Input Device framework defines a protocol for rich control of force-feedback devices, typically joysticks or rumble packs. However, for many devices, only simple feedback is needed for common feedback such as clicks, buzzes and such that are non-directional, and should be consistent from device to device and vendor to vendor, albeit tuned to the specific hardware. The below specifies a simplified haptics feedback control, with most features being optional, from simple, autonomous click feedback to somewhat more complex models with host-controlled variable intensity and timing. The protocol uses discoverable, pre-defined user-level haptic events. These haptics controls can be stand-alone, such as a phone vibration unit, or associated with a particular control or set of controls, such as a rigid-surface keyboard. As such, they may be contained within an independent Haptic Controller Application.
mod haptics;
pub use haptics::HapticsUsage;
