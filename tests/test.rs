/*NOTE:
* Rust has a built in test frame work
* to run all the tests you use `cargo test` or a specific test `cargo test <test name>`
* when you write a test function you need to prefix it with #[test]
* the tests would be ran on seprate threads and will be considered
* failed if the thread pandics (or doesnt panic given you have prefixed the function with #[should_panic])
* otherwise it is considered as passed
* you can use assert! macro and its variants to force a panic on a condition and provide a message
* with it
* if you want to do fancy pants error handeling with the question mark you can make the function return a result
* type of Result<(),String> the test pases if it gets () and failes otherwise
* you can also ignore specific tests in the normal case using #[ignore]
* these will only run if you run `cargo test -- --ignored`
* the conventions is unit tests live in the same file (I dont do them alot so it probably wont
* matter for me ) and integration tests live in the tests folder
* */

#[test]
fn it_should_work() {
    assert_eq!(2 + 2, 4);
}
