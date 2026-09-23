wit_bindgen::generate!({
    world: "w",
    generate_all,
});

struct KVLover;

impl exports::impo::impo::i_love_kv::Guest for KVLover {
    async fn get_the_kv(k: _rt::String,) -> Option<_rt::Vec::<u8>> {
        // let vtest = match spin_sdk::variables::get(&k).await {
        //     Ok(v) => v,
        //     Err(e) => { return Some(e.to_string().into()); }
        // };

        // let Ok(conn) = spin_sdk::sqlite::Connection::open_default().await else {
        //     return Some("sqlite conn woe!\n".into());
        // };

        // let Ok(qr) = conn.execute("select * from pets", []).await else {
        //     return Some("sqlite exec woe!\n".into());
        // };

        // let Ok(resset) = qr.collect().await else  {
        //     return Some("sqlite collect woe!\n".into());
        // };

        // let res_str = format!("{resset:?}");

        // let store = spin_sdk::key_value::Store::open("fie").await.unwrap();
        // let kv = store.get(&k).await.unwrap().map(|v| String::from_utf8_lossy(&v).to_string()).unwrap_or("<unk>".to_string());

        // let envo = std::env::var("BISCUITS");
        // let envo_text = match envo {
        //     Ok(s) => format!("envo = '{s}'"),
        //     Err(e) => format!("envo ERROR!!! {e}"),
        // };

        // let fr = std::fs::read_to_string("/arse.txt");
        // let fr_text = match fr {
        //     Ok(s) => format!("file content = '{s}'"),
        //     Err(e) => format!("file read ERROR!!! {e}"),
        // };

        // let pg_conn = spin_sdk::pg::Connection::open("host=localhost user=postgres password=my_password dbname=mydb").await;
        // let pg_text = match pg_conn {
        //     Err(e) => format!("PG open ERR!!! {e}"),
        //     Ok(conn) => match conn.query("SELECT * FROM pets", &[]).await {
        //         Err(e) => format!("PG query ERR!!! {e}"),
        //         Ok(qr) => match qr.collect().await {
        //             Err(e) => format!("PG collect ERR!!! {e}"),
        //             Ok(rows) => format!("PG row count {}", rows.len()),
        //         }
        //     }
        // };

        let h = spin_sdk::http::get("https://random-data-api.fermyon.app/animals/json").await;
        let h_text = match h {
            Ok(resp) => {
                use spin_sdk::http::body::IncomingBodyExt;
                match resp.into_body().bytes().await {
                    Ok(b) => format!("http resp = {}", String::from_utf8_lossy(b.as_ref())),
                    Err(e) => format!("http resp ERR {e}"),
                }
            },
            Err(e) => format!("http ERR {e}"),
        };

        // Some((format!("vtest = '{vtest}' and resset = '{res_str}' and kv = '{kv}' and env = '{envo_text}' and file = '{fr_text}' and pg = '{pg_text}' and http = '{h_text}'")).into())
        Some((format!("http = '{h_text}'")).into())
    }

    async fn set_the_kv(k: _rt::String, v: _rt::Vec::<u8>,) -> () {
        let store = spin_sdk::key_value::Store::open("fie").await.expect("where is the fie\n\n********* DID YOU REMEMBER TO PASS THE RUNTIME CONFIG *********\n");
        store.set(&k, &v).await.unwrap()
    }
}

export!(KVLover);
