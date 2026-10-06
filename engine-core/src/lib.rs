pub mod math;
pub mod time;
pub mod error;

pub use math::{Vec2, Mat3, Rect, Color};
pub use time::{Time, DeltaTime};
pub use error::{EngineError, Result};
