pub fn validate(source: &str) -> Result<(), String> {
    let s = source.trim();
    if !s.starts_with("(startofimagedata") {
        return Err("image must begin with '(startofimagedata'".into());
    }
    if !s.ends_with("EOID)") {
        return Err("image must end with 'EOID)'".into());
    }
    if s.contains("structure;/->") && !s.contains("structure;/->()") {
        return Err("invalid image structure declaration".into());
    }
    Ok(())
}
