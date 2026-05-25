use mock_object::mock_object;

#[mock_object(
    mock_one = [true, 89],
)]
struct TestObj {
    value_one: String,
    value_two: u32,
}

impl TestObj {
    fn new(value_one: &str, value_two: u32) -> Self {
        Self {
            value_one: value_one.to_string(),
            value_two
        }
    }
}

fn main() {
    let test_obj_mock_one = TestObj::mock_one();
}
