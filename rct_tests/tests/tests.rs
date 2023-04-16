#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/enum.rs");
    t.compile_fail("tests/ui/union.rs");
    t.pass("tests/ui/struct_named.rs");
    t.compile_fail("tests/ui/struct_unnamed.rs");
    t.compile_fail("tests/ui/struct_unit.rs");
}