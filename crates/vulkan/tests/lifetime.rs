//! Compile-fail tests for the lifetime rules of the generated structs.
//!
//! Every file in `ui/fail` stores a pointer to something that does not live long enough,
//! or aliases something Vulkan may write to, and must be rejected by the compiler.
//! `ui/pass` holds the same shapes with valid lifetimes.
//!
//! Regenerate the expected compiler output with `TRYBUILD=overwrite cargo test -p vulkan
//! --test lifetime` after a toolchain update, and review the diff.

#[test]
fn lifetimes() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
    t.pass("tests/ui/pass/*.rs");
}
