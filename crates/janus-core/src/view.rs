use serde::{Deserialize, Serialize};

/// Marker-wrapped transcript slices. The type system enforces D1:
/// monitors and judges accept `PublicView` only; `CovertView` is
/// unreachable from monitor code paths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicView<T> {
    inner: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovertView<T> {
    pub(crate) inner: T,
}

impl<T> PublicView<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    pub fn get(&self) -> &T {
        &self.inner
    }
}

impl<T> CovertView<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    pub fn get(&self) -> &T {
        &self.inner
    }
}

/// Redaction applied when projecting full telemetry into the public stream.
pub trait Redact {
    fn redacted(&self) -> Self;
}
