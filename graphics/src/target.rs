use crate::Viewport;

/// A graphics target, normally a screen.
#[derive(Debug, Clone, PartialEq)]
pub struct Target {
    /// The scale factor of the [`Target`].
    ///
    /// This factor is normally related to the pixel density
    /// of the target device.
    ///
    /// It can be different than the viewport scale factor,
    /// which may also be affected by application scaling.
    pub scale_factor: f64,

    /// The drawable region of the [`Target`].
    pub viewport: Viewport,
}
