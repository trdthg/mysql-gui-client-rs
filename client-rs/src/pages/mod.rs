//! Demo-code for showing how egui is used.
//!
//! The demo-code is also used in benchmarks and tests.

// ----------------------------------------------------------------------------

use eframe::egui;

pub mod headline;
pub mod list;
pub mod scroll;
pub mod setting;
pub mod talk;
pub mod test;

// ----------------------------------------------------------------------------

/// Something to view in the demo windows
pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

/// Something to view
pub trait Demo {
    /// `&'static` so we can also use it as a key to store open/close state.
    fn name(&self) -> &'static str;

    /// Show windows, etc
    fn show(&mut self, ctx: &egui::Context, open: &mut bool);
}
