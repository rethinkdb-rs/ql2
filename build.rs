fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/ql2.proto"], &["src/"])?;
    Ok(())
}
