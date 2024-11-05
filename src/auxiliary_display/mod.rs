//! Auxiliary Display Page (0x14)
//!
//! The Auxiliary Display page is intended for use by simple alphanumeric/auxiliary displays that are used on consumer devices. Making the alphanumeric and bitmap specific types of segments on an Auxiliary Display and creating a Custom Segments Report (CL) allow display manufacturers to produce displays with different segments. (e.g. custom segment as a battery indicator on a mobile phone

mod auxiliary_display;
pub use auxiliary_display::AuxiliaryDisplayUsage;
