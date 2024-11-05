//! Button Page (0x09)
//!
//! The Button page is the first place an application should look for user selection controls. System graphical user interfaces typically employ a pointer and a set of hierarchical selectors to select, move and otherwise manipulate their environment. For these purposes the following assignment of significance can be applied to the Button usages
mod button;
pub use button::ButtonUsage;
