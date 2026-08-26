use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpisodeStatus {
    Created,
    Provisioned,
    Running,
    AwaitingAudit,
    Suspended,
    Terminated,
    Audited,
    Scored,
    Archived,
    Failed,
}

impl EpisodeStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Archived | Self::Failed | Self::Scored
        )
    }
}
