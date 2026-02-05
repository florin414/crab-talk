pub mod user_service;

pub mod user {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/g_rpc/generated/user.rs"));
}
