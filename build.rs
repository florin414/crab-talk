use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = "src/g_rpc/generated";
    fs::create_dir_all(out_dir)?;

    tonic_prost_build::configure()
        .out_dir(out_dir)
        .compile_well_known_types(true)
        .build_server(true)
        .build_client(true)
        .compile_protos(
            &["src/g_rpc/proto/user.proto"],
            &["src/g_rpc/proto"])
        ?;

    Ok(())
}
