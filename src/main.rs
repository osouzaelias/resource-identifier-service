use warp::{Filter, http::StatusCode, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Deserialize)]
struct Input {
    param1: String,
    param2: String,
    param3: String,
    param4: String,
    param5: String,
}

#[derive(Serialize)]
struct Response {
    formatted_string: String,
}

fn validate_input(input: Input) -> Result<Input, Rejection> {
    if input.param1.trim().is_empty()
        || input.param2.trim().is_empty()
        || input.param3.trim().is_empty()
        || input.param4.trim().is_empty()
        || input.param5.trim().is_empty() {
        Err(warp::reject::custom(InvalidInput))
    } else {
        Ok(input)
    }
}

#[derive(Debug)]
struct InvalidInput;
impl warp::reject::Reject for InvalidInput {}

async fn handle_rejection(error: Rejection) -> Result<impl Reply, Infallible> {
    if error.find::<InvalidInput>().is_some() {
        Ok(warp::reply::with_status("Invalid input", StatusCode::BAD_REQUEST))
    } else {
        Ok(warp::reply::with_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR))
    }
}

#[tokio::main]
async fn main() {
    let format_route = warp::post()
        .and(warp::path("format"))
        .and(warp::body::json())
        .and_then(|input: Input| async move {
            validate_input(input).map(|input| {
                let formatted_string = format!(
                    "Parâmetro 1: {}, Parâmetro 2: {}, Parâmetro 3: {}, Parâmetro 4: {}, Parâmetro 5: {}",
                    input.param1, input.param2, input.param3, input.param4, input.param5
                );
                warp::reply::json(&Response { formatted_string })
            })
        });

    let health_route = warp::get()
        .and(warp::path("health"))
        .map(|| warp::reply::with_status("OK", StatusCode::OK));

    let routes = format_route.or(health_route).recover(handle_rejection);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
