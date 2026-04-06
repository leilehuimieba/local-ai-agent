use std::env;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

use runtime_core::{
    RUNTIME_NAME, RUNTIME_VERSION, RunEvent, RunRequest, RuntimeRunResponse, RuntimeSnapshot,
    simulate_run,
};

fn main() {
    let port = env::var("LOCAL_AGENT_RUNTIME_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8898);

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr).expect("bind runtime host");

    println!("[local-agent-runtime] runtime host listening on http://{addr}");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
        };
        eprintln!("[local-agent-runtime] incoming request");

        let request = match read_request(&stream) {
            Some(request) => request,
            None => continue,
        };
        eprintln!(
            "[local-agent-runtime] {} {} body={}",
            request.method,
            request.path,
            request.body.len()
        );

        let response = match (request.method.as_str(), request.path.as_str()) {
            ("GET", "/health") => json_response(
                200,
                format!(
                    "{{\"ok\":true,\"name\":\"{RUNTIME_NAME}\",\"version\":\"{RUNTIME_VERSION}\"}}"
                ),
            ),
            ("GET", "/v1/runtime/info") => {
                let snapshot = RuntimeSnapshot::idle();
                let current_run_id = snapshot
                    .current_run_id
                    .as_deref()
                    .map(|value| format!("\"{value}\""))
                    .unwrap_or_else(|| "null".to_string());
                json_response(
                    200,
                    format!(
                        "{{\"name\":\"{RUNTIME_NAME}\",\"version\":\"{RUNTIME_VERSION}\",\"state\":\"{}\",\"current_run_id\":{current_run_id}}}",
                        snapshot.state
                    ),
                )
            }
            ("POST", "/v1/runtime/run") => run_response(&request.body),
            _ => not_found(),
        };

        let _ = stream.write_all(response.as_bytes());
        let _ = stream.flush();
    }
}

struct HttpRequest {
    method: String,
    path: String,
    body: Vec<u8>,
}

fn read_request(stream: &std::net::TcpStream) -> Option<HttpRequest> {
    let cloned = stream.try_clone().ok()?;
    let mut reader = BufReader::new(cloned);

    let mut request_line = String::new();
    if reader.read_line(&mut request_line).ok()? == 0 {
        return None;
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next()?.to_string();
    let path = parts.next()?.to_string();

    let mut content_length = 0_usize;
    loop {
        let mut header_line = String::new();
        if reader.read_line(&mut header_line).ok()? == 0 {
            return None;
        }

        if header_line == "\r\n" {
            break;
        }

        if let Some((name, value)) = header_line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse::<usize>().unwrap_or(0);
            }
        }
    }

    let mut body = vec![0_u8; content_length];
    if content_length > 0 && reader.read_exact(&mut body).is_err() {
        return None;
    }

    Some(HttpRequest { method, path, body })
}

fn run_response(body: &[u8]) -> String {
    eprintln!("[local-agent-runtime] parsing run request");
    let request: RunRequest = match serde_json::from_slice(body) {
        Ok(request) => request,
        Err(_) => return json_response(400, "{\"error\":\"invalid_json\"}".to_string()),
    };

    eprintln!("[local-agent-runtime] simulate_run start");
    let response = with_memory_recall_event(simulate_run(&request));
    eprintln!("[local-agent-runtime] simulate_run finish");
    match serde_json::to_string(&response) {
        Ok(payload) => json_response(200, payload),
        Err(_) => json_response(500, "{\"error\":\"serialization_failed\"}".to_string()),
    }
}

fn with_memory_recall_event(mut response: RuntimeRunResponse) -> RuntimeRunResponse {
    let Some(index) = response.events.iter().position(is_memory_recall_source) else {
        return response;
    };
    let source = response.events[index].clone();
    response
        .events
        .insert(index + 1, memory_recall_event(&source));
    response
}

fn is_memory_recall_source(event: &RunEvent) -> bool {
    event.event_type == "plan_ready" && !memory_digest(event).is_empty()
}

fn memory_recall_event(source: &RunEvent) -> RunEvent {
    let digest = memory_digest(source);
    let mut event = source.clone();
    event.event_id = format!("{}-memory-recalled", source.event_id);
    event.event_type = "memory_recalled".to_string();
    event.summary = memory_recall_title(&digest);
    event.detail = digest.clone();
    event.result_summary = digest.clone();
    event.record_type = "recall_digest".to_string();
    event.source_type = "runtime".to_string();
    event.metadata = memory_recall_metadata(&source.metadata, &digest);
    event
}

fn memory_digest(event: &RunEvent) -> String {
    event
        .context_snapshot
        .as_ref()
        .map(|item| item.memory_digest.clone())
        .filter(|item| !item.is_empty())
        .unwrap_or_else(|| {
            event
                .metadata
                .get("memory_digest")
                .cloned()
                .unwrap_or_default()
        })
}

fn memory_recall_metadata(
    source: &std::collections::BTreeMap<String, String>,
    digest: &str,
) -> std::collections::BTreeMap<String, String> {
    let mut metadata = source.clone();
    metadata.insert("layer".to_string(), "long_term_memory".to_string());
    metadata.insert("record_type".to_string(), "recall_digest".to_string());
    metadata.insert("memory_kind".to_string(), "recall_digest".to_string());
    metadata.insert("source_type".to_string(), "runtime".to_string());
    metadata.insert("title".to_string(), memory_recall_title(digest));
    metadata.insert("reason".to_string(), memory_recall_reason(digest));
    metadata.insert("result_summary".to_string(), digest.to_string());
    metadata
}

fn memory_recall_title(digest: &str) -> String {
    if digest == "当前没有命中相关长期记忆。" {
        "未命中长期记忆".to_string()
    } else {
        "已完成记忆召回".to_string()
    }
}

fn memory_recall_reason(digest: &str) -> String {
    if digest == "当前没有命中相关长期记忆。" {
        "当前查询未命中可复用长期记忆，已输出空召回结果。".to_string()
    } else {
        "已按当前输入完成长期记忆召回，并将摘要注入上下文。".to_string()
    }
}

fn json_response(status_code: u16, body: String) -> String {
    let status_text = match status_code {
        200 => "OK",
        400 => "Bad Request",
        500 => "Internal Server Error",
        _ => "OK",
    };
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    )
}

fn not_found() -> String {
    let body = "{\"error\":\"not_found\"}";
    format!(
        "HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    )
}
