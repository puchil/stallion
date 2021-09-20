//! System console.

// Console Interface
pub mod interface {
    use core::fmt;

    // Console write function
    pub trait Write {
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
    }

    /// Console statistics.
    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fleged console.
    pub trait All = Write + Statistics;
}