use lambda_http::{
    http::{Response, StatusCode},
    run, service_fn, Error, IntoResponse, Request, RequestExt,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

pub async fn function_handler(event: Request) -> Result<impl IntoResponse, Error> {
    let bad_request = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "'number' parameter should be passed by query and be an unsigned integer",
            })
            .to_string(),
        )
        .map_err(Box::new)?;

    let num = match event.query_string_parameters().first("number") {
        Some(num) => match num.parse::<u128>() {
            Ok(n) => n,
            Err(e) => return Ok(bad_request),
        },
        None => return Ok(bad_request),
    };
    print!("NUMBER: {}", num);

    let factorial = factorial(num);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "result": factorial,
            })
            .to_string(),
        )
        .map_err(Box::new)?;

    Ok(response)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NumberPayload {
    pub num: u32,
}

fn factorial(num: u128) -> u128 {
    (1..=num).product()
}
