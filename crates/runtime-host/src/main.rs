use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use runtime_core::{
    RUNTIME_NAME, RUNTIME_VERSION, RunRequest, RuntimeSnapshot, capability_catalog,
    connector_catalog, simulate_run_with_runtime_events,
};

fn main() {
    let addr = runtime_addr();
    let listener = TcpListener::bind(&addr).expect("bind runtime host");
    println!("[local-agent-runtime] runtime host listening on http://{addr}");
    serve_requests(listener);
}

struct HttpRequest {
    method: String,
    path: String,
    body: Vec<u8>,
}

fn runtime_addr() -> String {
    let port = env::var("LOCAL_AGENT_RUNTIME_PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8898);
    format!("127.0.0.1:{port}")
}

fn serve_requests(listener: TcpListener) {
    for stream in listener.incoming().flatten() {
        handle_stream_on_worker(stream);
    }
}

fn handle_stream_on_worker(stream: TcpStream) {
    eprintln!("[local-agent-runtime] spawning request worker");
    let result = thread::Builder::new()
        .name("runtime-host-request".to_string())
        .stack_size(128 * 1024 * 1024)
        .spawn(move || handle_stream(stream));
    match result {
        Ok(worker) => {
            eprintln!("[local-agent-runtime] joining request worker");
            if worker.join().is_err() {
                eprintln!("[local-agent-runtime] request worker panicked");
            }
            eprintln!("[local-agent-runtime] request worker joined");
        }
        Err(error) => eprintln!("[local-agent-runtime] spawn request worker failed: {error}"),
    }
}

fn handle_stream(mut stream: TcpStream) {
    eprintln!("[local-agent-runtime] incoming request");
    let Some(request) = read_request(&stream) else {
        return;
    };
    eprintln!(
        "[local-agent-runtime] {} {} body={}",
        request.method,
        request.path,
        request.body.len()
    );
    let response = route_response(&request);
    if let Err(error) = stream.write_all(response.as_bytes()) {
        eprintln!("[local-agent-runtime] write response failed: {error}");
        return;
    }
    if let Err(error) = stream.flush() {
        eprintln!("[local-agent-runtime] flush response failed: {error}");
    }
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

    let body = read_body(&mut reader, content_length)?;
    Some(HttpRequest { method, path, body })
}

fn read_body(reader: &mut BufReader<TcpStream>, content_length: usize) -> Option<Vec<u8>> {
    let mut body = Vec::with_capacity(content_length);
    while body.len() < content_length {
        let available = reader.fill_buf().ok()?;
        if available.is_empty() {
            return None;
        }
        let take = available.len().min(content_length - body.len());
        body.extend_from_slice(&available[..take]);
        reader.consume(take);
    }
    Some(body)
}

fn route_response(request: &HttpRequest) -> String {
    let (path, query) = path_and_query(&request.path);
    match (request.method.as_str(), path) {
        ("GET", "/health") => health_response(),
        ("GET", "/v1/runtime/info") => info_response(),
        ("GET", "/v1/runtime/capabilities") => capabilities_response(query),
        ("GET", "/v1/runtime/connectors") => connectors_response(),
        ("POST", "/v1/runtime/run") => run_response(&request.body),
        _ => not_found(),
    }
}

fn path_and_query(path: &str) -> (&str, &str) {
    path.split_once('?').unwrap_or((path, ""))
}

fn health_response() -> String {
    json_response(
        200,
        format!("{{\"ok\":true,\"name\":\"{RUNTIME_NAME}\",\"version\":\"{RUNTIME_VERSION}\"}}"),
    )
}

fn info_response() -> String {
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

fn capabilities_response(query: &str) -> String {
    let mode = query_param(query, "mode").unwrap_or("standard");
    match serde_json::to_string(&capability_catalog(mode)) {
        Ok(payload) => json_response(200, payload),
        Err(_) => json_response(500, "{\"error\":\"serialization_failed\"}".to_string()),
    }
}

fn connectors_response() -> String {
    match serde_json::to_string(&connector_catalog()) {
        Ok(payload) => json_response(200, payload),
        Err(_) => json_response(500, "{\"error\":\"serialization_failed\"}".to_string()),
    }
}

fn query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    query.split('&').find_map(|pair| {
        let (name, value) = pair.split_once('=')?;
        if name == key { Some(value) } else { None }
    })
}

fn run_response(body: &[u8]) -> String {
    eprintln!("[local-agent-runtime] parsing run request");
    let request: RunRequest = match serde_json::from_slice(body) {
        Ok(request) => request,
        Err(_) => return json_response(400, "{\"error\":\"invalid_json\"}".to_string()),
    };

    eprintln!("[local-agent-runtime] simulate_run start");
    let response = simulate_run_with_runtime_events(&request);
    eprintln!("[local-agent-runtime] simulate_run finish");
    match serde_json::to_string(&response) {
        Ok(payload) => json_response(200, payload),
        Err(_) => json_response(500, "{\"error\":\"serialization_failed\"}".to_string()),
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
