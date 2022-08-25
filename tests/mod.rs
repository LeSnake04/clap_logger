mod p_clap_command;
#[cfg(feature = "custom_env")]
mod p_cenv_loglevel_invalid;
#[cfg(feature = "custom_env")]
mod p_cenv_loglevel_valid;
mod p_verbose_overuse;

#[test]
fn ui() {
	let t = trybuild::TestCases::new();
	t.pass("tests/p_*.rs");
	t.compile_fail("tests/f_*.rs");
}