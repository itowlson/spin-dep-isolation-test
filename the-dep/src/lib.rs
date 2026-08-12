wit_bindgen::generate!({
    world: "w",
    generate_all,
});

struct KVLover;

impl exports::impo::impo::i_love_kv::Guest for KVLover {
    async fn get_the_kv(k: _rt::String,) -> Option<_rt::Vec::<u8>> {
        let vtest = match spin_sdk::variables::get(&k).await {
            Ok(v) => v,
            Err(e) => { return Some(e.to_string().into()); }
        };

        let Ok(conn) = spin_sdk::sqlite::Connection::open_default().await else {
            return Some("sqlite conn woe!\n".into());
        };

        let Ok(qr) = conn.execute("select * from pets", []).await else {
            return Some("sqlite exec woe!\n".into());
        };

        let Ok(resset) = qr.collect().await else  {
            return Some("sqlite collect woe!\n".into());
        };

        let res_str = format!("{resset:?}");

        Some((format!("vtest = '{vtest}' and resset = '{res_str}'")).into())

        // let store = spin_sdk::key_value::Store::open("fie").await.unwrap();
        // store.get(&k).await.unwrap()
    }

    async fn set_the_kv(_k: _rt::String,_v: _rt::Vec::<u8>,) -> () {
        // let store = spin_sdk::key_value::Store::open("fie").await.unwrap();
        // store.set(&k, &v).await.unwrap()
    }
}

export!(KVLover);
