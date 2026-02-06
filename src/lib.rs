pub mod user {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/g_rpc/generated/user.rs"));
}

pub mod g_rpc;


