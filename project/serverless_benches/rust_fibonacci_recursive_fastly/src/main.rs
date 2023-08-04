use fastly::http::{StatusCode};
use fastly::{mime, Error, Request, Response};
use serde_json::json;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match req.get_path() {
        "/" => {
            let params = req.get_query_str().unwrap_or_default();
            let result = fibonacci(40);

            let data = json!({
                "param": params,
                "result": result
            });
            let response = serde_json::to_string(&data)?;

            // Send a default synthetic response.
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JSON)
                .with_body(response))
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
