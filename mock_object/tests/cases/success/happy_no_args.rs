use mock_object::mock_object;

#[mock_object(
    mock_one = [],
    mock_two = [],
)]
struct TestObj {}

impl TestObj {
    fn new() -> Self {
        Self {}
    }
}

fn main() {
    let _test_obj_mock_one = TestObj::mock_one();
    let _test_obj_mock_two = TestObj::mock_two();
}
