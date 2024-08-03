use std::sync::Arc;

use axum::{extract::State, http::StatusCode, routing::get};
use mc_query::status::data::StatusResponse;

fn build_response(mc_status: StatusResponse) -> String {
    let active_players = mc_status.players.online;

    let ret = format!("# HELP mc_active_players Active players in the Minecraft server
# TYPE mc_active_players gauge
mc_active_players {active_players}");

    ret
}

struct AppState {
    mc_addr: String,
    mc_port: u16,
}

async fn metrics_handler(
    State(state): State<Arc<AppState>>
) -> Result<String, StatusCode> {
    let mc_status = mc_query::status(&state.mc_addr, state.mc_port)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(build_response(mc_status))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(file) = std::env::var("ROUXINOLD_ENV_FILE") {
        dotenvy::from_filename(file)?;
    } else {
        let _ = dotenvy::dotenv();
    }

    let listen_addr = std::env::var("ROUXINOLD_LISTEN_ADDR").expect("ROUXINOLD_LISTEN_ADDR not present");
    let listen_port = std::env::var("ROUXINOLD_LISTEN_PORT").expect("ROUXINOLD_LISTEN_PORT not present");

    let mc_addr = std::env::var("ROUXINOLD_SERVER_ADDR").expect("ROUXINOLD_SERVER_ADDR not present");
    let mc_port = std::env::var("ROUXINOLD_SERVER_PORT").expect("ROUXINOLD_SERVER_ADDR not present").parse::<u16>()?;

    let state = Arc::new(AppState {
        mc_addr,
        mc_port,
    });

    let app = axum::Router::new()
        .route("/metrics", get(metrics_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("{listen_addr}:{listen_port}").as_str())
        .await?;

    axum::serve(listener, app).await?;

    Ok(())
}
