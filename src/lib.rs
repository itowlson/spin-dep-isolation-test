use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_service;

spin_sdk::dependencies!();

#[http_service]
async fn handle_dep_isolation_test(_req: Request) -> anyhow::Result<impl IntoResponse> {
    // let h = spin_sdk::http::get("https://random-data-api.fermyon.app/animals/json").await;
    // let h_text = match h {
    //     Ok(resp) => {
    //         use spin_sdk::http::body::IncomingBodyExt;
    //         match resp.into_body().bytes().await {
    //             Ok(b) => format!("http resp = {}", String::from_utf8_lossy(b.as_ref())),
    //             Err(e) => format!("http resp ERR {e}"),
    //         }
    //     },
    //     Err(e) => format!("http ERR {e}"),
    // };
    // eprintln!("******** MAIN COMP h_text *****\n{h_text}\n*********");
    

    impo::impo::i_love_kv::set_the_kv("fie".to_string(), "I'M A KV".as_bytes().to_vec()).await;

    let dep_output = impo::impo::i_love_kv::get_the_kv("fie".to_string()).await.unwrap();
    let dep_output = String::from_utf8_lossy(&dep_output).to_string();

    let main_comp_var = spin_sdk::variables::get("spork").await.unwrap();

    let dep_output_forbidden = impo::impo::i_love_kv::get_the_kv("spork".to_string()).await.unwrap();
    let dep_output_forbidden = String::from_utf8_lossy(&dep_output_forbidden).to_string();
    let main_comp_var_forbidden = spin_sdk::variables::get("fie").await.err();

    let resp = format!("MAIN: {main_comp_var}\nDEP: {dep_output}\nMAIN TRYING TO GET DEP: {main_comp_var_forbidden:?}\nDEP TRYING TO GET MAIN: {dep_output_forbidden}\n");
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(resp))
}
