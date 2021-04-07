fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["src/ql2.proto"], &["src/"])?;
    Ok(())
}
