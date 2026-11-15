#[test]
fn prints_hello_world(){assert_eq!(mixieland::run("print \"Hello, World!\";").unwrap(),"Hello, World!\n");}
#[test]
fn parses_kernel_source_pull(){let s=r#"pull { LinuxKernel, source = "https://github.com/torvalds/linux.git", revision = "master", build = "make" }"#;let o=mixieland::run(s).unwrap();assert!(o.contains("Builgine pull plan: LinuxKernel"));assert!(o.contains("build system: make"));}
#[test]
fn requires_binary_checksum(){let s=r#"pull { LinuxKernel, binary = "https://cdn.example.org/Image" }"#;assert!(mixieland::run(s).unwrap_err().contains("SHA256"));}
#[test]
fn rejects_http_binary(){let s=r#"pull { Kernel, binary = "http://example.org/Image", checksum = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" }"#;assert!(mixieland::run(s).unwrap_err().contains("HTTPS"));}
#[test]
fn rejects_unterminated_string(){assert!(mixieland::run("print \"broken;").is_err());}
#[test]
fn binary_pull_rejects_revision(){let spec=mixieland::ast::PullSpec{name:"kernel".into(),binary:Some("https://example.org/Image".into()),checksum:Some("SHA256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into()),revision:Some("main".into()),..Default::default()};assert!(mixieland::builgine::validate_pull(&spec).is_err());}
#[test]
fn build_requires_explicit_trust_confirmation(){let spec=mixieland::ast::PullSpec{name:"LinuxKernel".into(),source:Some("https://github.com/torvalds/linux.git".into()),build:Some("make".into()),..Default::default()};let plan=mixieland::builgine::validate_pull(&spec).unwrap();assert!(mixieland::builgine::build_source(&plan,".",false).unwrap_err().contains("trusted"));}
