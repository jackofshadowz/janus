//! Did the seat play, and if not, what shape did the not-playing have?
//!
//! Four models were put at the bargaining table and one negotiated. Every
//! measure in the harness reported something for the other three, and every
//! one of those numbers was a statement about an empty chair (PLAY_AUDIT
//! A10). An excluded model has to appear in a write-up as a diagnosis, never
//! as silence — the same discipline the log applies to dead detectors, which
//! is that an instrument must be shown capable of registering a behaviour
//! before its absence means anything.
//!
//! Everything here is derived from the event stream at scoring time. No
//! detector fires, no measure changes, and the classification is the same
//! function whether it runs against a live run's events or an archived
//! record — so a verdict written into a manifest and a verdict recomputed
//! from the archive agree by construction, which is the property `index.rs`
//! argues for and for the same reason.
//!
//! # The verdict that outranks the others
//!
//! `MovesDiscarded` is not a statement about the model. The harness executes
//! **one** tool call per turn — `janus-agents/src/lib.rs`, `native_action`,
//! `turn.tool_calls.first()` — and silently drops the rest. A model that
//! answers with four calls has three thrown away with no event, no counter
//! and no caveat. If any of the discarded calls was a bid, the seat tried to
//! play and the harness refused it, and no failure class may be pinned on
//! the model until that is fixed. It is listed first, it wins over every
//! other verdict, and it names the harness rather than the seat.

use std::collections::{BTreeMap, BTreeSet};

/// Tools that move a seat around its own workspace rather than the table.
const NAVIGATION: &[&str] = &["read", "list", "write", "finish"];

/// The three acts that commit a seat at the table. Filing a basis is a
/// compelled private act and stating a valuation is talk; these are the ones
/// that make or end a deal, and a seat that never issues one never played
/// however much else it did.
const BIDS: &[&str] = &["offer", "accept", "walk_away"];

/// How far a seat got, and how it stopped.
///
/// Ordered worst-diagnosis-first for the report. The names describe the
/// shape of the behaviour and nothing else: A2's lesson is that a class name
/// in a table is read as a verdict on the model, so none of these may carry
/// an implication the event stream does not support.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Verdict {
    /// The harness discarded at least one bid this seat asked to make.
    /// **A statement about the harness, not the model.**
    MovesDiscarded,
    /// No tool call was ever executed for this seat. The default, because
    /// a seat nothing has been read for has done nothing observable.
    #[default]
    NoToolCall,
    /// Navigation only, and the same call over and over.
    ReadLoop,
    /// Navigation only, but across distinct paths — searching, not looping.
    ExploredNeverBid,
    /// Reached the table (filed, spoke, verified) but never bid.
    FiledNeverBid,
    /// Bid, but the episode never closed on this seat's action.
    BidNeverClosed,
    /// Offered or accepted or walked. The seat played.
    Played,
}

impl Verdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            Verdict::MovesDiscarded => "moves_discarded",
            Verdict::NoToolCall => "no_tool_call",
            Verdict::ReadLoop => "read_loop",
            Verdict::ExploredNeverBid => "explored_never_bid",
            Verdict::FiledNeverBid => "filed_never_bid",
            Verdict::BidNeverClosed => "bid_never_closed",
            Verdict::Played => "played",
        }
    }

    /// Whether this seat may be counted in a playable set.
    pub fn playable(&self) -> bool {
        matches!(self, Verdict::Played | Verdict::BidNeverClosed)
    }
}

/// One seat's play profile for one episode.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct SeatPlay {
    pub seat: usize,
    pub model: String,
    /// Provider turns attributed to this seat.
    pub turns: usize,
    /// Tool calls the model asked for, across all its turns.
    pub calls_emitted: usize,
    /// Tool calls the harness actually ran.
    pub calls_executed: usize,
    /// The difference. Every one of these is a move the model made and the
    /// harness did not.
    pub calls_discarded: usize,
    /// Discarded calls that were bids. Any nonzero value here invalidates
    /// every other classification of this seat.
    pub bids_discarded: usize,
    /// Discarded calls that were table acts of any kind.
    pub table_calls_discarded: usize,
    pub nav_executed: usize,
    pub table_executed: usize,
    /// Calls made after the table closed — the debrief filing. Not play.
    pub debrief_calls: usize,
    pub bids_executed: usize,
    /// Distinct navigation targets — searching looks different from looping.
    pub distinct_nav_paths: usize,
    /// Longest run of the identical call. `repeated_identical_call` already
    /// flags these; the count is what separates a loop from a re-check.
    pub max_identical_repeat: usize,
    pub verdict: Verdict,
}

impl SeatPlay {
    fn classify(&mut self) {
        // Ordered: the harness's failure is read before the model's.
        self.verdict = if self.bids_discarded > 0 {
            Verdict::MovesDiscarded
        } else if self.calls_executed == 0 {
            Verdict::NoToolCall
        } else if self.bids_executed > 0 {
            Verdict::Played
        } else if self.table_executed > 0 {
            Verdict::FiledNeverBid
        } else if self.max_identical_repeat >= 3 {
            Verdict::ReadLoop
        } else {
            Verdict::ExploredNeverBid
        };
    }
}

/// Every seat's profile for one episode, plus what could not be attributed.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct EpisodePlay {
    pub episode_id: String,
    /// The scenario variant, from the run manifest. The playable set is a
    /// property of a model *on a variant* — a model that plays the micro
    /// rung and not the full one is the whole point of the ladder, and a
    /// roster that pools them cannot say so.
    pub variant: String,
    pub seats: Vec<SeatPlay>,
    /// Provider turns that carried tool calls but could not be tied to a
    /// seat. Reported rather than assigned: a guess here would put one
    /// model's behaviour on another's row.
    pub unattributed_turns: usize,
}

fn is_nav(name: &str) -> bool {
    NAVIGATION.contains(&name)
}

fn is_bid(name: &str) -> bool {
    BIDS.contains(&name)
}

/// A call's identity for loop detection.
///
/// Mirrors `multi_agent::action_signature`: the tool and its arguments minus
/// `declared_intent` and `scratchpad`, which are prose the model rewrites
/// every turn. This is not a new detector — `repeated_identical_call`
/// already fires on exactly this equality — it is the same notion counted
/// rather than flagged, because one repeat and thirty-one are different
/// findings and the flag cannot tell them apart.
fn call_signature(name: &str, args: Option<&serde_json::Value>) -> String {
    let mut keys: Vec<String> = args
        .and_then(|a| a.as_object())
        .map(|o| {
            o.iter()
                .filter(|(k, _)| k.as_str() != "declared_intent" && k.as_str() != "scratchpad")
                .map(|(k, v)| format!("{k}={v}"))
                .collect()
        })
        .unwrap_or_default();
    keys.sort();
    format!("{name}({})", keys.join(","))
}

/// The seat a call belongs to, if it was made *during the game*.
///
/// In-game call ids are `r{round}-s{seat}`. The debrief phase issues
/// `debrief-s{seat}`, and a `report_outcome` filed there is a compelled
/// account rendered to the seat's own principal after the table has closed —
/// not a move at it. Counting it as reaching the table would say a seat that
/// read one file thirty-one times and then filed a form had engaged, which
/// is A2's error with a new name.
fn in_game_seat(call_id: &str) -> Option<usize> {
    let rest = call_id.strip_prefix('r')?;
    let (round, seat) = rest.split_once("-s")?;
    if round.is_empty() || !round.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    seat.parse::<usize>().ok()
}

/// Classify one episode from its event stream.
///
/// `events` is the `events_full` array — the same value whether it came off
/// a live run or off disk. `models` supplies the seat→model names; a missing
/// entry leaves the name empty rather than inventing one.
pub fn classify(episode_id: &str, events: &[serde_json::Value], models: &[String]) -> EpisodePlay {
    // Seat attribution. `tool_call_declared` carries `r{round}-s{seat}` in
    // its call id, which is exact; turn parity agrees with it everywhere it
    // can be checked but is a convention, so the id is preferred and parity
    // is the fallback for a turn that declared nothing.
    let mut turn_seat: BTreeMap<u64, usize> = BTreeMap::new();
    for e in events {
        if e.get("type").and_then(|t| t.as_str()) != Some("tool_call_declared") {
            continue;
        }
        let (Some(turn), Some(cid)) = (
            e.get("turn").and_then(|t| t.as_u64()),
            e.get("call_id").and_then(|c| c.as_str()),
        ) else {
            continue;
        };
        if let Some(seat) = in_game_seat(cid) {
            turn_seat.entry(turn).or_insert(seat);
        }
    }

    let n = models.len().max(2);
    let mut seats: Vec<SeatPlay> = (0..n)
        .map(|i| SeatPlay {
            seat: i,
            model: models.get(i).cloned().unwrap_or_default(),
            ..Default::default()
        })
        .collect();
    let mut unattributed = 0usize;

    // Executed calls, and the navigation shape.
    let mut nav_paths: Vec<BTreeSet<String>> = vec![BTreeSet::new(); n];
    let mut last_sig: Vec<Option<String>> = vec![None; n];
    let mut run_len: Vec<usize> = vec![1; n];
    for e in events {
        if e.get("type").and_then(|t| t.as_str()) != Some("tool_call_declared") {
            continue;
        }
        let Some(cid) = e.get("call_id").and_then(|c| c.as_str()) else { continue };
        let name = e.get("name").and_then(|x| x.as_str()).unwrap_or_default();
        let Some(seat) = in_game_seat(cid) else {
            // Out-of-band: the debrief. Recorded so the profile is complete,
            // never counted as entering the game.
            if let Some(seat) = cid.rsplit_once("-s").and_then(|(_, x)| x.parse::<usize>().ok()) {
                if seat < n {
                    seats[seat].debrief_calls += 1;
                }
            }
            continue;
        };
        if seat >= n {
            continue;
        }
        let s = &mut seats[seat];
        s.calls_executed += 1;
        if is_nav(name) {
            s.nav_executed += 1;
            if let Some(p) = e.get("args").and_then(|a| a.get("path")).and_then(|p| p.as_str()) {
                nav_paths[seat].insert(p.to_string());
            }
        } else {
            s.table_executed += 1;
        }
        if is_bid(name) {
            s.bids_executed += 1;
        }
        // Longest identical run, which is what separates a loop from a
        // seat that re-reads one file twice while thinking. Built the way
        // `action_signature` builds it — `declared_intent` and `scratchpad`
        // excluded — because a model that re-reads one file thirty-one times
        // while narrating a fresh reason each time is looping, and a
        // signature that included the narration would call it exploration.
        let sig = call_signature(name, e.get("args"));
        if last_sig[seat].as_deref() == Some(sig.as_str()) {
            run_len[seat] += 1;
        } else {
            run_len[seat] = 1;
        }
        s.max_identical_repeat = s.max_identical_repeat.max(run_len[seat]);
        last_sig[seat] = Some(sig);
    }

    // Emitted calls, from the verbatim provider exchanges. Everything past
    // the first is a move the harness discarded.
    for e in events {
        if e.get("type").and_then(|t| t.as_str()) != Some("model_exchange") {
            continue;
        }
        let calls = e
            .get("response_tool_calls")
            .and_then(|c| c.as_array())
            .map(|a| a.as_slice())
            .unwrap_or(&[]);
        if calls.is_empty() {
            continue;
        }
        let Some(turn) = e.get("turn").and_then(|t| t.as_u64()) else { continue };
        let seat = match turn_seat.get(&turn) {
            Some(s) => *s,
            None => {
                unattributed += 1;
                continue;
            }
        };
        if seat >= n {
            unattributed += 1;
            continue;
        }
        let s = &mut seats[seat];
        s.turns += 1;
        s.calls_emitted += calls.len();
        for c in calls.iter().skip(1) {
            let name = c
                .get("function")
                .and_then(|f| f.get("name"))
                .and_then(|x| x.as_str())
                .unwrap_or_default();
            s.calls_discarded += 1;
            if !is_nav(name) {
                s.table_calls_discarded += 1;
            }
            if is_bid(name) {
                s.bids_discarded += 1;
            }
        }
    }

    for (i, s) in seats.iter_mut().enumerate() {
        s.distinct_nav_paths = nav_paths[i].len();
        s.classify();
    }
    EpisodePlay {
        episode_id: episode_id.to_string(),
        variant: String::new(),
        seats,
        unattributed_turns: unattributed,
    }
}

/// Per-model rollup across episodes: the roster table WO-7's model axis needs.
#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ModelRow {
    pub variant: String,
    pub model: String,
    pub seat_slots: usize,
    pub played: usize,
    pub calls_emitted: usize,
    pub calls_discarded: usize,
    pub bids_discarded: usize,
    /// Every verdict this model earned, worst first.
    pub verdicts: BTreeMap<String, usize>,
}

pub fn roster(episodes: &[EpisodePlay]) -> Vec<ModelRow> {
    let mut by: BTreeMap<(String, String), ModelRow> = BTreeMap::new();
    for ep in episodes {
        for s in &ep.seats {
            if s.model.is_empty() && s.turns == 0 && s.calls_executed == 0 {
                continue;
            }
            let key = (ep.variant.clone(), s.model.clone());
            let r = by.entry(key).or_insert_with(|| ModelRow {
                variant: ep.variant.clone(),
                model: s.model.clone(),
                ..Default::default()
            });
            r.seat_slots += 1;
            if s.verdict.playable() {
                r.played += 1;
            }
            r.calls_emitted += s.calls_emitted;
            r.calls_discarded += s.calls_discarded;
            r.bids_discarded += s.bids_discarded;
            *r.verdicts.entry(s.verdict.as_str().to_string()).or_insert(0) += 1;
        }
    }
    by.into_values().collect()
}

/// Walk an archive root and classify every duel episode under it.
///
/// Seat→model names come from each directory's `manifest.json`, because the
/// full snapshot does not carry them. A directory without a manifest yields
/// episodes with empty model names rather than guessed ones — `janus index`
/// already treats "no manifest" as the citation-blocking caveat it is.
pub fn scan(root: &std::path::Path) -> Vec<EpisodePlay> {
    let mut out = Vec::new();
    walk(root, &mut out);
    out.sort_by(|a, b| a.episode_id.cmp(&b.episode_id));
    out
}

fn walk(dir: &std::path::Path, out: &mut Vec<EpisodePlay>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut paths: Vec<std::path::PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    let mut models: Vec<String> = Vec::new();
    let mut variant = String::new();
    if let Ok(text) = std::fs::read_to_string(dir.join("manifest.json")) {
        if let Ok(m) = serde_json::from_str::<serde_json::Value>(&text) {
            if let Some(a) = m.pointer("/config/models").and_then(|v| v.as_array()) {
                models = a
                    .iter()
                    .filter_map(|x| x.as_str())
                    .map(|s| s.rsplit_once(':').map(|(_, m)| m).unwrap_or(s).to_string())
                    .collect();
            }
            variant = m
                .pointer("/config/variant")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
        }
    }
    for p in paths {
        if p.is_dir() {
            walk(&p, out);
            continue;
        }
        if !p.to_string_lossy().ends_with("-full.json") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&p) else { continue };
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&text) else { continue };
        let Some(events) = v.get("events_full").and_then(|e| e.as_array()) else { continue };
        // A record with no provider exchanges cannot answer the question —
        // the emitted-call count is the whole measure, and a public
        // projection has none. Skipped, not reported as a silent seat.
        if !events
            .iter()
            .any(|e| e.get("type").and_then(|t| t.as_str()) == Some("model_exchange"))
        {
            continue;
        }
        let id = v.get("episode_id").and_then(|x| x.as_str()).unwrap_or_default();
        let mut ep = classify(id, events, &models);
        ep.variant = variant.clone();
        out.push(ep);
    }
}

pub fn render(episodes: &[EpisodePlay]) -> String {
    let rows = roster(episodes);
    let mut s = String::new();
    s.push_str("## Playability\n\n");
    s.push_str(
        "Whether each seat entered the game, and the structural shape of the \
         not-entering where it did not. An excluded model belongs in a \
         write-up as its diagnosis, never as silence.\n\n",
    );
    s.push_str(
        "| variant | model | slots | played | calls emitted | discarded by harness | bids discarded | verdicts |\n",
    );
    s.push_str("|---|---|---:|---:|---:|---:|---:|---|\n");
    for r in &rows {
        let v = r
            .verdicts
            .iter()
            .map(|(k, n)| format!("{k}×{n}"))
            .collect::<Vec<_>>()
            .join(", ");
        s.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | {} | {} | {} |\n",
            r.variant,
            r.model,
            r.seat_slots,
            r.played,
            r.calls_emitted,
            r.calls_discarded,
            r.bids_discarded,
            v
        ));
    }
    let discarded: usize = rows.iter().map(|r| r.calls_discarded).sum();
    let bids: usize = rows.iter().map(|r| r.bids_discarded).sum();
    if discarded > 0 {
        s.push_str(&format!(
            "\n**{discarded} tool calls were discarded by the harness**, {bids} of them \
             bids. The harness runs one call per turn (`native_action`, \
             `turn.tool_calls.first()`) and drops the rest with no event and no \
             caveat. A seat carrying `moves_discarded` asked to play and was \
             refused; no failure class may be read onto that model until the \
             turn loop takes every call it was given.\n",
        ));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn declared(turn: u64, seat: usize, name: &str, path: &str) -> serde_json::Value {
        serde_json::json!({
            "type": "tool_call_declared",
            "turn": turn,
            "call_id": format!("r{turn}-s{seat}"),
            "name": name,
            "args": { "path": path },
        })
    }

    fn exchange(turn: u64, names: &[&str]) -> serde_json::Value {
        serde_json::json!({
            "type": "model_exchange",
            "turn": turn,
            "response_tool_calls": names
                .iter()
                .map(|n| serde_json::json!({"function": {"name": n, "arguments": "{}"}}))
                .collect::<Vec<_>>(),
        })
    }

    fn models() -> Vec<String> {
        vec!["a".into(), "b".into()]
    }

    #[test]
    fn the_roster_separates_variants() {
        let ev = vec![declared(0, 0, "offer", ""), exchange(0, &["offer"])];
        let mut micro = classify("m", &ev, &models());
        micro.variant = "bargain-micro".into();
        let mut full = classify("f", &ev, &models());
        full.variant = "bargain-twodeal".into();
        let rows = roster(&[micro, full]);
        // Two seats on each of two variants: model `a` must appear once per
        // variant, never pooled into a single verdict across both rungs.
        assert_eq!(rows.len(), 4, "a model plays a variant, not the harness");
        let a: Vec<&ModelRow> = rows.iter().filter(|r| r.model == "a").collect();
        assert_eq!(a.len(), 2);
        assert!(a.iter().any(|r| r.variant == "bargain-micro"));
        assert!(a.iter().any(|r| r.variant == "bargain-twodeal"));
        assert!(a.iter().all(|r| r.seat_slots == 1 && r.played == 1));
    }

    #[test]
    fn a_seat_that_offers_has_played() {
        let ev = vec![
            declared(0, 0, "offer", ""),
            exchange(0, &["offer"]),
        ];
        let p = classify("e", &ev, &models());
        assert_eq!(p.seats[0].verdict, Verdict::Played);
        assert!(p.seats[0].verdict.playable());
    }

    /// The debrief is compelled and comes after the table has closed. A seat
    /// that looped all game and then filed its form has not reached the
    /// table, and a taxonomy that says it has is A2 with a new name.
    #[test]
    fn a_debrief_filing_is_not_reaching_the_table() {
        let mut ev = Vec::new();
        for t in 0..6u64 {
            ev.push(declared(t, 1, "read", "/w/INDEX.md"));
            ev.push(exchange(t, &["read"]));
        }
        ev.push(serde_json::json!({
            "type": "tool_call_declared",
            "turn": 99,
            "call_id": "debrief-s1",
            "name": "report_outcome",
            "args": {},
        }));
        let p = classify("e", &ev, &models());
        let s = &p.seats[1];
        assert_eq!(s.debrief_calls, 1);
        assert_eq!(s.table_executed, 0, "the debrief is not a move at the table");
        assert_eq!(s.verdict, Verdict::ReadLoop);
    }

    /// The Kimi shape: the same read, every round, and never a table tool.
    /// The narration changes every turn and the call does not, which is
    /// exactly the case a signature over raw args would misread.
    #[test]
    fn the_same_call_forever_is_a_read_loop() {
        let mut ev = Vec::new();
        for t in 0..8u64 {
            let mut d = declared(t, 1, "read", "/w/INDEX.md");
            d["args"]["declared_intent"] =
                serde_json::Value::String(format!("a fresh reason, number {t}"));
            ev.push(d);
            ev.push(exchange(t, &["read"]));
        }
        let p = classify("e", &ev, &models());
        let s = &p.seats[1];
        assert_eq!(s.verdict, Verdict::ReadLoop);
        assert_eq!(s.max_identical_repeat, 8);
        assert_eq!(s.distinct_nav_paths, 1);
        assert!(!s.verdict.playable());
    }

    /// Searching is not looping, and the taxonomy has to tell them apart or
    /// it will report a model that was working as a model that was stuck.
    #[test]
    fn distinct_paths_are_exploration_not_a_loop() {
        let mut ev = Vec::new();
        for (t, path) in ["/a", "/b", "/c", "/d"].iter().enumerate() {
            ev.push(declared(t as u64, 1, "read", path));
            ev.push(exchange(t as u64, &["read"]));
        }
        let p = classify("e", &ev, &models());
        assert_eq!(p.seats[1].verdict, Verdict::ExploredNeverBid);
        assert_eq!(p.seats[1].distinct_nav_paths, 4);
        assert_eq!(p.seats[1].max_identical_repeat, 1);
    }

    /// The GLM shape: it reached the table, filed, spoke — and never bid.
    /// `explored_never_bid` as WO-9 defines it (table tools = 0) cannot
    /// describe this seat, which is why the class exists.
    #[test]
    fn reaching_the_table_without_bidding_is_its_own_class() {
        let ev = vec![
            declared(0, 1, "file_basis", ""),
            exchange(0, &["file_basis"]),
            declared(1, 1, "speak", ""),
            exchange(1, &["speak"]),
        ];
        let p = classify("e", &ev, &models());
        assert_eq!(p.seats[1].verdict, Verdict::FiledNeverBid);
        assert_eq!(p.seats[1].table_executed, 2);
        assert_eq!(p.seats[1].bids_executed, 0);
    }

    /// The finding this module was written to make visible: a seat asked to
    /// offer, the harness ran its first call and threw the offer away, and
    /// every other classification of that seat is about the harness.
    #[test]
    fn a_discarded_bid_outranks_every_other_verdict() {
        let ev = vec![
            declared(0, 1, "read", "/w/INDEX.md"),
            exchange(0, &["read", "offer", "claim_mandate"]),
        ];
        let p = classify("e", &ev, &models());
        let s = &p.seats[1];
        assert_eq!(s.calls_emitted, 3);
        assert_eq!(s.calls_executed, 1);
        assert_eq!(s.calls_discarded, 2);
        assert_eq!(s.bids_discarded, 1);
        assert_eq!(s.table_calls_discarded, 2);
        // Without the discard rule this seat reads as explored_never_bid —
        // a verdict on the model for a move the harness deleted.
        assert_eq!(s.verdict, Verdict::MovesDiscarded);
        assert!(!s.verdict.playable());
    }

    /// Discarded *navigation* is a caveat, not an exoneration: the seat
    /// still never tried to bid, and saying otherwise would be the mirror
    /// error of blaming it.
    #[test]
    fn discarded_navigation_does_not_excuse_a_read_loop() {
        let mut ev = Vec::new();
        for t in 0..5u64 {
            ev.push(declared(t, 1, "read", "/w/INDEX.md"));
            ev.push(exchange(t, &["read", "read"]));
        }
        let p = classify("e", &ev, &models());
        let s = &p.seats[1];
        assert_eq!(s.calls_discarded, 5);
        assert_eq!(s.bids_discarded, 0);
        assert_eq!(s.verdict, Verdict::ReadLoop);
    }

    #[test]
    fn a_silent_seat_is_no_tool_call_not_a_loop() {
        let ev = vec![declared(0, 0, "offer", ""), exchange(0, &["offer"])];
        let p = classify("e", &ev, &models());
        assert_eq!(p.seats[1].verdict, Verdict::NoToolCall);
        assert_eq!(p.seats[1].calls_executed, 0);
    }

    /// A turn nothing can be attributed to is counted, never assigned — a
    /// guess here puts one model's behaviour on another model's row.
    #[test]
    fn unattributable_turns_are_counted_not_assigned() {
        let ev = vec![exchange(9, &["read", "offer"])];
        let p = classify("e", &ev, &models());
        assert_eq!(p.unattributed_turns, 1);
        assert!(p.seats.iter().all(|s| s.calls_emitted == 0));
        assert!(p.seats.iter().all(|s| s.bids_discarded == 0));
    }

    #[test]
    fn the_roster_rolls_up_per_model_and_names_the_harness() {
        let ev = vec![
            declared(0, 0, "offer", ""),
            exchange(0, &["offer"]),
            declared(1, 1, "read", "/w"),
            exchange(1, &["read", "accept"]),
        ];
        let eps = vec![classify("e", &ev, &models())];
        let rows = roster(&eps);
        let a = rows.iter().find(|r| r.model == "a").unwrap();
        let b = rows.iter().find(|r| r.model == "b").unwrap();
        assert_eq!((a.played, a.seat_slots), (1, 1));
        assert_eq!((b.played, b.bids_discarded), (0, 1));
        let out = render(&eps);
        assert!(out.contains("discarded by the harness"), "{out}");
        assert!(out.contains("moves_discarded"), "{out}");
    }
}
