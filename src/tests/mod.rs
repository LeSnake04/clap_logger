use trybuild::TestCases;

mod pass;

#[test]
fn ui() {
	let t: TestCases = TestCases::new();
	t.pass("pass/*");
	t.compile_fail("fail/*");
}
