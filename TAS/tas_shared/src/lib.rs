//! Everything tas_ui and tas_test share about TAS_Helper.dll: the shared-memory
//! protocol (`state`, `platform`), the identity stamps recordings carry
//! (`rider`, `renderer`, `level`), the menu channel (`menu`), and the policy
//! both sides must agree on — the CONT bucket judge (`cont`) and the
//! restart/arm/reroll transport controller (`transport`).

mod menu;
#[cfg(windows)]
mod platform;
mod renderer;
mod rider;
mod state;

pub mod cont;
pub mod level;
pub mod transport;

pub use menu::*;
#[cfg(windows)]
pub use platform::TasSharedMemoryClient;
pub use renderer::*;
pub use rider::*;
pub use state::*;
