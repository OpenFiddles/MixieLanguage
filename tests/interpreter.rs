use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn prints_hello_world() {
    let mut c = Command::new(env!("CARGO_BIN_EXE_mixie"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    c.stdin
        .as_mut()
        .unwrap()
        .write_all(b"print \"Hello, World!\";\n")
        .unwrap();
    let o = c.wait_with_output().unwrap();
    assert!(o.status.success());
    assert_eq!(String::from_utf8(o.stdout).unwrap(), "Hello, World!\n");
}

#[test]
fn rejects_unterminated_string() {
    let mut c = Command::new(env!("CARGO_BIN_EXE_mixie"))
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    c.stdin
        .as_mut()
        .unwrap()
        .write_all(b"print \"broken;")
        .unwrap();
    assert!(!c.wait_with_output().unwrap().status.success());
}
