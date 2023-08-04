use blake3::hash;
use fastly::http::StatusCode;
use fastly::{Error, Request, Response};
use serde_json::json;
use std::time::Instant;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let input = b"hello, world";
    let iterations = 100_000;
    let start_time = Instant::now();
    for _ in 0..iterations {
        let _ = hash(input);
    }
    let duration = start_time.elapsed();
    let average_time = duration.as_micros() as f64 / iterations as f64;

    // optional parameter to prevent cache hit
    let param = req.get_query_parameter("param").unwrap_or("default");

    let data = json!({
        "param": param,
        "fib_execution_time": format!("Average time per hash: {:.2} micro seconds", average_time),
    });
    let response = serde_json::to_string(&data)?; // Serialize JSON object

    Ok(Response::from_status(StatusCode::OK)
        .with_header("Content-Type", "application/json")
        .with_body(response))
}
