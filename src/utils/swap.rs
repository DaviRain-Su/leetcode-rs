pub fn swap_array_element<T: Default>(array: &mut [T], x: usize, y: usize) -> Result<(), String> {
    if x > array.len() || y > array.len() {
        return Err(format!(
            "x({x}) or y({y}) out of the array length({})",
            array.len()
        ));
    }
    array.swap(x, y);
    Ok(())
}

#[test]
fn test_swap_array_element() {
    let mut array = vec![1, 2, 3];
    swap_array_element(&mut array, 0, 1).unwrap();
    println!("array = {array:?}")
}
