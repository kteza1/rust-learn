pub mod mqtt;

//cargo test -- --nocapture
#[test]
fn it_works() {
	mqtt::test();
}
