#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("provider error: {0}")]
    Provider(String),
    /// The model answered, but not with a usable action. Distinct from
    /// `Provider` so that harness-side action loss is counted rather than
    /// silently scored as non-action.
    #[error("protocol failure: {0}")]
    ProtocolFailure(String),
    #[error("sandbox error: {0}")]
    Sandbox(String),
    #[error("policy violation: {0}")]
    PolicyViolation(String),
    #[error("budget exhausted: {0}")]
    BudgetExhausted(String),
    #[error("invalid state transition: {0} -> {1}")]
    InvalidTransition(&'static str, &'static str),
}

pub type Result<T> = std::result::Result<T, CoreError>;
