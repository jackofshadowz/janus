//! Schema migrations and canonical queries. DB wiring (surrealdb crate)
//! lands with the persistence milestone; this module is the single
//! source of truth for DDL and the public projection.

pub const MIGRATION_0001: &str = include_str!("../migrations/0001_init.surql");

/// Monitor-safe projection: covert rows are never selected (D1).
pub const PUBLIC_PROJECTION: &str = r#"
SELECT id, turn,
       (SELECT kind, content FROM public_state WHERE step = $parent.id) AS public,
       ->invoked->tool_call.{name, args, executed_effect, divergence} AS tools
FROM step WHERE episode = $episode ORDER BY turn;
"#;

/// Auth-gated full projection; exports produced from it are tainted.
pub const FULL_PROJECTION: &str = r#"
SELECT id, turn,
       (SELECT kind, content FROM public_state WHERE step = $parent.id) AS public,
       (SELECT scratchpad FROM covert_state WHERE step = $parent.id) AS covert,
       ->invoked->tool_call.* AS tools
FROM step WHERE episode = $episode ORDER BY turn;
"#;
