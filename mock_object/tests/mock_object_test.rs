#[test]
fn test_mock_object_macro() {
    let t = trybuild::TestCases::new();
    // Run success cases
    t.pass("tests/cases/success/happy_with_args.rs");
    t.pass("tests/cases/success/happy_no_args.rs");

    // Run failure cases
    t.compile_fail("tests/cases/failure/no_constructor/no_constructor.rs");
    t.compile_fail("tests/cases/failure/invalid_syntax_arg/invalid_syntax_arg.rs");
    t.compile_fail("tests/cases/failure/invalid_syntax_brackets/invalid_syntax_brackets.rs");
    t.compile_fail("tests/cases/failure/invalid_syntax_no_list/invalid_syntax_no_list.rs");
    t.compile_fail("tests/cases/failure/invalid_args_count/invalid_args_count.rs");
    t.compile_fail("tests/cases/failure/invalid_arg_types/invalid_arg_types.rs");
}
