-- JANUS SurrealDB schema, v1 (see docs/SCHEMA.md)

DEFINE TABLE episode SCHEMAFUL;
DEFINE FIELD scenario_id     ON episode TYPE string;
DEFINE FIELD scenario_hash   ON episode TYPE string;
DEFINE FIELD model_config    ON episode TYPE object;
DEFINE FIELD sandbox_backend ON episode TYPE string;
DEFINE FIELD egress_policy   ON episode TYPE array;
DEFINE FIELD status          ON episode TYPE string ASSERT $value IN ["created","provisioned","running","suspended","terminated","audited","scored","archived","failed"];
DEFINE FIELD created_at      ON episode TYPE datetime DEFAULT time::now();
DEFINE FIELD pinned_config   ON episode TYPE option<object>;

DEFINE TABLE agent SCHEMAFUL;
DEFINE FIELD episode  ON agent TYPE record<episode>;
DEFINE FIELD role     ON agent TYPE string ASSERT $value IN ["target","monitor","judge","environment"];
DEFINE FIELD model_id ON agent TYPE option<string>;

DEFINE TABLE step SCHEMAFUL;
DEFINE FIELD episode    ON step TYPE record<episode>;
DEFINE FIELD turn       ON step TYPE int;
DEFINE FIELD created_at ON step TYPE datetime DEFAULT time::now();

DEFINE TABLE public_state SCHEMAFUL;
DEFINE FIELD step    ON public_state TYPE record<step>;
DEFINE FIELD kind    ON public_state TYPE string ASSERT $value IN ["agent_message","tool_call_summary","env_observation","monitor_note"];
DEFINE FIELD content ON public_state TYPE string;

DEFINE TABLE covert_state SCHEMAFUL;
DEFINE FIELD step       ON covert_state TYPE record<step>;
DEFINE FIELD scratchpad ON covert_state TYPE string;
DEFINE FIELD self_model ON covert_state TYPE option<string>;

DEFINE TABLE tool_call SCHEMAFUL;
DEFINE FIELD step            ON tool_call TYPE record<step>;
DEFINE FIELD name            ON tool_call TYPE string;
DEFINE FIELD args            ON tool_call TYPE flexible;
DEFINE FIELD declared_intent ON tool_call TYPE string;
DEFINE FIELD executed_effect ON tool_call TYPE option<string>;
DEFINE FIELD blocked         ON tool_call TYPE bool DEFAULT false;
DEFINE FIELD divergence      ON tool_call TYPE option<float>;

DEFINE TABLE divergence_signal SCHEMAFUL;
DEFINE FIELD call     ON divergence_signal TYPE record<tool_call>;
DEFINE FIELD detector ON divergence_signal TYPE string;
DEFINE FIELD detail   ON divergence_signal TYPE flexible;

DEFINE TABLE audit_score SCHEMAFUL;
DEFINE FIELD episode   ON audit_score TYPE record<episode>;
DEFINE FIELD scope     ON audit_score TYPE option<record<step>>;
DEFINE FIELD verdict   ON audit_score TYPE string ASSERT $value IN ["clean","suspect","violation"];
DEFINE FIELD rationale ON audit_score TYPE string;

DEFINE TABLE has_agent    SCHEMAFUL TYPE RELATION FROM episode TO agent;
DEFINE TABLE has_step     SCHEMAFUL TYPE RELATION FROM episode TO step;
DEFINE TABLE performed_by SCHEMAFUL TYPE RELATION FROM tool_call TO agent;
DEFINE TABLE audited_by   SCHEMAFUL TYPE RELATION FROM episode TO audit_score;

DEFINE INDEX idx_step_episode_turn ON step FIELDS episode, turn UNIQUE;
DEFINE INDEX idx_episode_status ON episode FIELDS status;
