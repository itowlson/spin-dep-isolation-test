use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_service;

spin_sdk::dependencies!();

#[http_service]
async fn handle_dep_isolation_test(_req: Request) -> anyhow::Result<impl IntoResponse> {
    impo::impo::i_love_kv::set_the_kv("fie".to_string(), "I'M A KV".as_bytes().to_vec()).await;
    let glarg = impo::impo::i_love_kv::get_the_kv("fie".to_string()).await.unwrap();
    let glarg = String::from_utf8_lossy(&glarg).to_string();
    let moar = spin_sdk::variables::get("spork").await.unwrap();
    let glarg_nope = impo::impo::i_love_kv::get_the_kv("spork".to_string()).await.unwrap();
    let glarg_nope = String::from_utf8_lossy(&glarg_nope).to_string();
    let moar_nope = spin_sdk::variables::get("fie").await.err();

    let resp = format!("MAIN: {moar}\nDEP: {glarg}\nMAIN TRYING TO GET DEP: {moar_nope:?}\nDEP TRYING TO GET MAIN: {glarg_nope}\n");
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(resp))
}
