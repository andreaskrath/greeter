use axum::{
    Router,
    body::Body,
    extract::{MatchedPath, Query},
    http::{Request, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use fact::FACTS;
use rand::Rng;
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{Level, info, info_span};
use tracing_subscriber::fmt::time::OffsetTime;
use uuid::Uuid;

mod fact;

const INDEX_HTML: &[u8] = include_bytes!("../static/index.html");

const RICK_GIF: &[u8] = include_bytes!("../static/rickroll.gif");

const FAVICON: &[u8] = include_bytes!("../static/favicon.ico");

#[derive(Deserialize, Debug)]
struct BadLuckProtection {
    blp: String,
}

fn main() {
    let time = OffsetTime::local_rfc_3339().expect("could not initialize time offset");
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_timer(time)
        .with_level(true)
        .with_target(false)
        .try_init()
        .expect("failed to initialize tracing subscriber");

    async_main();
}

#[tokio::main]
async fn async_main() {
    let http_trace = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
        let matched_path = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str);

        let request_id = Uuid::new_v4().to_string();

        info_span!(
            "http_request",
            method = ?request.method(),
            path = matched_path,
            id = request_id,
        )
    });

    let greeter = Router::new()
        .route("/", get(index))
        .route("/fact", get(fact))
        .route("/favicon.ico", get(favicon))
        .layer(http_trace);

    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("failed to bind to port 8080");

    info!("Starting greeter service on port 8080");
    axum::serve(listener, greeter)
        .await
        .expect("failed to start greeter service");
}

async fn index() -> Html<&'static [u8]> {
    info!("Index request");
    Html(INDEX_HTML)
}

async fn fact(query: Query<BadLuckProtection>) -> Response {
    info!("Getting fact with blp='{}'", query.blp);
    let Ok(blp) = query.blp.parse::<usize>() else {
        info!("Invalid BLP value, rick roll commensing as punishment");
        return rick();
    };

    let mut rng = rand::rng();
    let random_index = rng.random_range(0..FACTS.len() + 5);
    let actual_index = random_index + blp;
    if actual_index < FACTS.len() {
        info!("Getting fact {actual_index}");
        let fact = FACTS[actual_index];
        (StatusCode::OK, fact).into_response()
    } else {
        info!("Got rick rolled naturally");
        rick()
    }
}

async fn favicon() -> Response {
    FAVICON.into_response()
}

fn rick() -> Response {
    let body = Body::from(RICK_GIF);

    Response::builder()
        .header("Content-Type", "image/gif")
        .body(body)
        .expect("failed to create rick GIF response")
}
