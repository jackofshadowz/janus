pub mod bridge_sse;
pub mod cf;
pub mod mock;

use std::future::Future;

use janus_core::{EgressPolicy, Result, SandboxEnvironment, ToolInvocation, ToolResult};

pub use cf::{map_path, CfSandboxClient};
pub use mock::MockSandbox;

/// Dyn-free sandbox dispatcher (SandboxEnvironment uses RPITIT and cannot
/// be used as a trait object).
#[derive(Clone)]
pub enum AnySandbox {
    Mock(MockSandbox),
    Cf(CfSandboxClient),
}

impl SandboxEnvironment for AnySandbox {
    fn provision(
        &self,
        episode_id: &str,
        egress: EgressPolicy,
    ) -> impl Future<Output = Result<()>> + Send {
        let this = self.clone();
        let episode_id = episode_id.to_string();
        async move {
            match &this {
                AnySandbox::Mock(s) => s.provision(&episode_id, egress).await,
                AnySandbox::Cf(s) => s.provision(&episode_id, egress).await,
            }
        }
    }

    fn exec(&self, invocation: &ToolInvocation) -> impl Future<Output = Result<ToolResult>> + Send {
        let this = self.clone();
        let invocation = invocation.clone();
        async move {
            match &this {
                AnySandbox::Mock(s) => s.exec(&invocation).await,
                AnySandbox::Cf(s) => s.exec(&invocation).await,
            }
        }
    }

    fn teardown(&self, episode_id: &str) -> impl Future<Output = Result<()>> + Send {
        let this = self.clone();
        let episode_id = episode_id.to_string();
        async move {
            match &this {
                AnySandbox::Mock(s) => s.teardown(&episode_id).await,
                AnySandbox::Cf(s) => s.teardown(&episode_id).await,
            }
        }
    }
}
