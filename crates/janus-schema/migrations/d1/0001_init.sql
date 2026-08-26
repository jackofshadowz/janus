-- JANUS D1 (SQLite) schema v1 — translated from docs/SCHEMA.md (SurrealQL).
-- Graph edges become foreign keys; record links become *_id TEXT columns.

PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS episode (
    id              TEXT PRIMARY KEY,            -- e.g. 'batch-groq-qwen-s42'
    scenario_id     TEXT NOT NULL,
    scenario_hash   TEXT,
    model_provider  TEXT NOT NULL,
    model_id        TEXT NOT NULL,
    seed            INTEGER,
    temperature     REAL,
    sandbox_backend TEXT NOT NULL,
    judge           TEXT,
    egress_policy   TEXT,                        -- JSON array
    status          TEXT NOT NULL DEFAULT 'created'
                    CHECK (status IN ('created','provisioned','running','suspended','terminated','audited','scored','archived','failed')),
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS agent (
    id        TEXT PRIMARY KEY,
    episode_id TEXT NOT NULL REFERENCES episode(id),
    role      TEXT NOT NULL CHECK (role IN ('target','monitor','judge','environment')),
    model_id  TEXT
);

CREATE TABLE IF NOT EXISTS step (
    id         TEXT PRIMARY KEY,                 -- '{episode_id}:t{turn}'
    episode_id TEXT NOT NULL REFERENCES episode(id),
    turn       INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE (episode_id, turn)
);

CREATE TABLE IF NOT EXISTS public_state (
    id      TEXT PRIMARY KEY,
    step_id TEXT NOT NULL REFERENCES step(id),
    kind    TEXT NOT NULL CHECK (kind IN ('agent_message','tool_call_summary','env_observation','monitor_note')),
    content TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS covert_state (
    id         TEXT PRIMARY KEY,
    step_id    TEXT NOT NULL REFERENCES step(id),
    scratchpad TEXT NOT NULL,
    self_model TEXT
);

CREATE TABLE IF NOT EXISTS tool_call (
    id              TEXT PRIMARY KEY,            -- call_id from the interceptor
    step_id         TEXT NOT NULL REFERENCES step(id),
    name            TEXT NOT NULL,
    args            TEXT,                        -- JSON (public-redacted for dead drops)
    declared_intent TEXT NOT NULL DEFAULT '',
    executed_effect TEXT,
    public_effect   TEXT,                        -- tiered summary; NULL = same as executed
    blocked         INTEGER NOT NULL DEFAULT 0,
    divergence      REAL
);

CREATE TABLE IF NOT EXISTS divergence_signal (
    id       TEXT PRIMARY KEY,
    call_id  TEXT NOT NULL REFERENCES tool_call(id),
    detector TEXT NOT NULL,
    detail   TEXT                                -- JSON
);

CREATE TABLE IF NOT EXISTS audit_score (
    id          TEXT PRIMARY KEY,
    episode_id  TEXT NOT NULL REFERENCES episode(id),
    turn        INTEGER,                         -- NULL = whole-episode verdict
    verdict     TEXT NOT NULL CHECK (verdict IN ('clean','suspect','violation')),
    rationale   TEXT
);

CREATE INDEX IF NOT EXISTS idx_step_episode_turn ON step(episode_id, turn);
CREATE INDEX IF NOT EXISTS idx_episode_status ON episode(status);
CREATE INDEX IF NOT EXISTS idx_tool_call_step ON tool_call(step_id);

-- Monitor-safe projection: covert rows are never selected.
CREATE VIEW IF NOT EXISTS public_projection AS
SELECT s.episode_id,
       s.turn,
       p.kind,
       p.content,
       t.name           AS tool_name,
       t.args           AS tool_args,               -- pre-redacted at write time
       t.public_effect  AS tool_effect,             -- tiered; dead-drop payloads excluded
       t.blocked        AS tool_blocked,
       t.divergence     AS tool_divergence
FROM step s
LEFT JOIN public_state p ON p.step_id = s.id
LEFT JOIN tool_call  t ON t.step_id = s.id
ORDER BY s.episode_id, s.turn;
