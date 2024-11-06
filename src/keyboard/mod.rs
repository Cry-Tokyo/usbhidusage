//! Keyboard/Keypad Page (0x07)
//!
//! This section is the Usage Page for key codes to be used in implementing a USB keyboard. A Boot Keyboard (84-, 101-or 104-key) should at a minimum support all associated usage codes as indicated in the Boot column below.
mod keyboard;
pub use keyboard::KeyboardUsage;
