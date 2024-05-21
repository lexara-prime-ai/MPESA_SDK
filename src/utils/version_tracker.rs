const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn display_version() -> &'static str {
    println!(
        r#"
        [MPESA SDK version {}]
    "#,
        VERSION
    );
    VERSION
}
