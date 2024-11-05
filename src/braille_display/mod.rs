//! Braille Display Page (0x41)
//!
//! Braille display allow visually impaired computer users to read out text using raised pins. The pins are electro-mechanically activated. These devices also have support for controls that help navigate the computer screen. Typically, braille displays interface with software known as a screen reader in order to perform this navigation.
mod braille_display;
pub use braille_display::BrailleDisplayUsage;
