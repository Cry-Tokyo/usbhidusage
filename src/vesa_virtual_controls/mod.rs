//! VESA Virtual Controls Page (0x82)
//!
//! Defines controls for VESA supported monitor characteristics and control of monitor brightness, contrast, size, position, etc. Each UsageId below is a Virtual Control Panel (VCP) op-code of the same id/value. All VCP op-codes are assigned by VESA (see VESA Monitor Command Set Standard (MCCS)).
mod vesa_virtual_controls;
pub use vesa_virtual_controls::VesaVirtualControlsUsage;
