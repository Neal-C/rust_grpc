use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/quote.proto")?;

    // magic
    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(out_dir);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("quote_descriptor.bin"))
        .compile(&["proto/quote.proto"], &["proto"])?;

    Ok(())
}