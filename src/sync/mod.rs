// Copied from https://learningos.github.io/rCore-Tutorial-Book-v3/chapter2/3batch-system.html
// A utility struct to use global variable safely

//! Synchronization and interior mutability primitives

mod up;

pub use up::UPSafeCell;