use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

pub async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let bad_request = |message: &str| {
        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "error": "Bad Request",
                    "message": message,
                })
                .to_string(),
            )
            .map_err(Box::new)
    };

    let num: u32 = match event.query_string_parameters().first("number") {
        Some(num) => match num.parse::<u32>() {
            Ok(n) => n,
            Err(_) => {
                return Ok(bad_request(
                    "Invalid 'number' parameter. Must be a positive integer.",
                )?)
            }
        },
        None => return Ok(bad_request("Missing 'number' parameter in query string.")?),
    };

    let factorial = factorial(num);

    let body_json_parts = json!({
        "number": num,
        "result": -1,
    })
    .to_string();

    let split = body_json_parts.split("-1").collect::<Vec<&str>>();

    let json = format!("{}{}{}", split[0], factorial.to_string(), split[1]);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json)
        .map_err(Box::new)?;

    Ok(response)
}

use rug::Integer;

fn factorial(value: u32) -> Integer {
    (2..=value).fold(Integer::ONE.clone(), |res, n| res * n)
}
