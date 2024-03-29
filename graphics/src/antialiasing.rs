//! Choose antialiasing strategies.

/// An antialiasing strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Antialiasing {
    /// No antialiasing.
    ///
    /// This is the default.
    #[default]
    Disabled,
    /// Automatic detection of the best antialiasing strategy.
    ///
    /// Backends will choose an antialiasing strategy based on
    /// the pixel density of the target display; potentially
    /// disabling antialiasing altogether if deemed unnecessary.
    Automatic,
    /// Multisample antialiasing.
    MSAA(MSAA),
}

impl From<bool> for Antialiasing {
    fn from(enable: bool) -> Self {
        if enable {
            Self::Automatic
        } else {
            Self::Disabled
        }
    }
}

/// A multisample antialiasing strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MSAA {
    /// 2 samples
    X2,
    /// 4 samples
    X4,
    /// 8 samples
    X8,
    /// 16 samples
    X16,
}

impl MSAA {
    /// Returns the amount of samples of the [`MSAA`] strategy.
    pub fn sample_count(self) -> u32 {
        match self {
            Self::X2 => 2,
            Self::X4 => 4,
            Self::X8 => 8,
            Self::X16 => 16,
        }
    }
}
