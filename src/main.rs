#[cfg(test)]
mod tests;

use warp::{Filter, http::StatusCode, Rejection, Reply};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use log::info;


#[derive(Deserialize)]
struct Input {
    legal_entity: String,
    tenant: String,
    segment: String,
    payment_instrument: String,
    customer_id: String,
}

#[derive(Serialize)]
struct Response {
    ris: String,
}

fn create_irn(input: &Input) -> String {
    format!(
        "ris:{}:{}:{}:{}:{}",
        input.legal_entity, input.tenant, input.segment, input.payment_instrument, input.customer_id
    )
}

fn validate_input(input: &Input) -> Result<(), Rejection> {
    if input.legal_entity.trim().is_empty() ||
        input.tenant.trim().is_empty() ||
        input.segment.trim().is_empty() ||
        input.payment_instrument.trim().is_empty() ||
        input.customer_id.trim().is_empty() {
        Err(warp::reject::custom(InvalidInput))
    } else {
        Ok(())
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
    env_logger::init();
    let format_route = warp::post()
        .and(warp::path("format"))
        .and(warp::body::json())
        .and_then(|input: Input| async move {
            info!("Request for /format received");

            validate_input(&input)?;
            let ris = create_irn(&input);
            info!("Responding to the request with '{}'", ris);
            Ok::<_, Rejection>(warp::reply::json(&Response { ris }))
        });


    let health_route = warp::get()
        .and(warp::path("health"))
        .map(|| warp::reply::with_status("OK", StatusCode::OK));

    let routes = format_route.or(health_route).recover(handle_rejection);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}