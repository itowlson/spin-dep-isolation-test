wit_bindgen::generate!({
    world: "w",
    generate_all,
});

struct KVLover;

impl exports::impo::impo::i_love_kv::Guest for KVLover {
    async fn get_the_kv(k: _rt::String,) -> Option<_rt::Vec::<u8>> {
        match spin_sdk::variables::get(&k).await {
            Ok(v) => Some(v.into()),
            Err(e) => Some(e.to_string().into()),
        }
        // let store = spin_sdk::key_value::Store::open("fie").await.unwrap();
        // store.get(&k).await.unwrap()
    }

    async fn set_the_kv(_k: _rt::String,_v: _rt::Vec::<u8>,) -> () {
        // let store = spin_sdk::key_value::Store::open("fie").await.unwrap();
        // store.set(&k, &v).await.unwrap()
    }
}

export!(KVLover);
