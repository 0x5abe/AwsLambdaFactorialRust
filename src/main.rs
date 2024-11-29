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

    let num: u128 = match event.query_string_parameters().first("number") {
        Some(num) => match num.parse::<u128>() {
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

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
                "number": num,
                "result": factorial,
            })
            .to_string(),
        )
        .map_err(Box::new)?;

    Ok(response)
}

fn factorial(num: u128) -> u128 {
    (1..=num).product()
}