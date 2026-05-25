use mock_object::mock_object;

#[mock_object(
    mock_one = ["Test value for mock one.", 89],
    mock_two = ["Test value for mock two.", 45],
)]
struct TestObj {
    value_one: String,
    value_two: u32,
}

fn main() {
    let test_obj_mock_one = TestObj::mock_one();
    let test_obj_mock_two = TestObj::mock_two();
}
