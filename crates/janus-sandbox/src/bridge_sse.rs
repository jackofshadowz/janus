use base64::Engine;

/// One parsed event from the bridge's SSE exec stream.
#[derive(Debug, Clone, PartialEq)]
pub enum SseEvent {
    Stdout(Vec<u8>),
    Stderr(Vec<u8>),
    Exit(i32),
    Error(String),
}

/// Minimal parser for the bridge exec stream: `event: <name>` /
/// `data: <payload>` pairs separated by blank lines.
pub fn parse_exec_stream(body: &str) -> Vec<SseEvent> {
    let mut events: Vec<SseEvent> = Vec::new();
    let mut event_name: Option<String> = None;
    let mut data = String::new();

    for line in body.lines() {
        if let Some(rest) = line.strip_prefix("event:") {
            if event_name.is_some() || !data.is_empty() {
                flush(&event_name.take(), &mut data, &mut events);
            }
            event_name = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("data:") {
            data.push_str(rest.trim_start_matches(' '));
            data.push('\n');
        } else if line.is_empty() {
            flush(&event_name.take(), &mut data, &mut events);
        }
    }
    flush(&event_name.take(), &mut data, &mut events);
    events
}

fn flush(name: &Option<String>, data: &mut String, out: &mut Vec<SseEvent>) {
    if let Some(name) = name {
        match name.as_str() {
            "stdout" => {
                if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(data.trim()) {
                    out.push(SseEvent::Stdout(bytes));
                }
            }
            "stderr" => {
                if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(data.trim()) {
                    out.push(SseEvent::Stderr(bytes));
                }
            }
            "exit" => {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(data.trim()) {
                    out.push(SseEvent::Exit(
                        v.get("exit_code").and_then(|c| c.as_i64()).unwrap_or(-1) as i32,
                    ));
                }
            }
            "error" => {
                let msg = serde_json::from_str::<serde_json::Value>(data.trim())
                    .ok()
                    .and_then(|v| v.get("error").and_then(|e| e.as_str()).map(str::to_string))
                    .unwrap_or_else(|| data.trim().to_string());
                out.push(SseEvent::Error(msg));
            }
            _ => {}
        }
    }
    data.clear();
}

pub fn decode_stdout(events: &[SseEvent]) -> String {
    let mut out: Vec<u8> = Vec::new();
    for ev in events {
        if let SseEvent::Stdout(b) = ev {
            out.extend_from_slice(b);
        }
    }
    const MAX: usize = 64 * 1024;
    let truncated: &[u8] = if out.len() > MAX { &out[..MAX] } else { &out };
    String::from_utf8_lossy(truncated).to_string()
}

pub fn exit_code(events: &[SseEvent]) -> Option<i32> {
    events.iter().find_map(|ev| match ev {
        SseEvent::Exit(c) => Some(*c),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_stream() {
        // b64("hello from janus\n") = aGVsbG8gZnJvbSBqYW51cwo=
        let body = "event: stdout\ndata: aGVsbG8gZnJvbSBqYW51cwo=\n\
                    \n\
                    event: exit\ndata: {\"exit_code\": 0}\n\n";
        let evs = parse_exec_stream(body);
        assert_eq!(decode_stdout(&evs), "hello from janus\n");
        assert_eq!(exit_code(&evs), Some(0));
    }

    #[test]
    fn parses_error_terminal() {
        let body = "event: error\ndata: {\"error\": \"boom\", \"code\": \"X\"}\n\n";
        let evs = parse_exec_stream(body);
        assert_eq!(evs.last(), Some(&SseEvent::Error("boom".into())));
    }

    #[test]
    fn handles_missing_trailing_newline() {
        let body = "event: stdout\ndata: aGVsbG8=\nevent: exit\ndata: {\"exit_code\":1}";
        let evs = parse_exec_stream(body);
        assert_eq!(decode_stdout(&evs), "hello");
        assert_eq!(exit_code(&evs), Some(1));
    }
}
