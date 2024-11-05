//! Physical Input Device Page (0x0F)
//!
//! A Physical Input Device (PID) generates force-feedback sensations using parameterized, pre-downloaded, waveform-based Effects. A device can support Effects from well-understood waveforms (e.g. ET Ramp, ET Sine, ET Spring), and/or custom waveforms (ET Custom-Force). See 18.4.1 Effect Types for all well-known waveforms.
mod physical_input_device;
pub use physical_input_device::PhysicalInputDeviceUsage;
