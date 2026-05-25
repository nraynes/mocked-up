use mock_object::mock_object;

#[mock_object(
    mock_one = ["Test value for mock one.", 89],
    mock_two = ["Test value for mock two.", 45],
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
    assert_eq!(test_obj_mock_one.value_one, "Test value for mock one.");
    assert_eq!(test_obj_mock_one.value_two, 89);

    let test_obj_mock_two = TestObj::mock_two();
    assert_eq!(test_obj_mock_two.value_one, "Test value for mock two.");
    assert_eq!(test_obj_mock_two.value_two, 45);
}
