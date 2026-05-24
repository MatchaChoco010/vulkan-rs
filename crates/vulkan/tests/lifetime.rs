#[test]
fn lifetime_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/lifetime-temporary-slice.rs");
    t.compile_fail("tests/ui/pnext-temporary.rs");
    t.compile_fail("tests/ui/shader-code-temporary.rs");
    t.pass("tests/ui/pnext-and-shader-code-live.rs");
}
