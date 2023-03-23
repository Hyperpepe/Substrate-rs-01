fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for &number in numbers {
        let temp_sum = sum.checked_add(number);
        match temp_sum {
            Some(s) => sum = s,
            None => return None,
        }
    }
    Some(sum)
}

fn main() {
    let numbers = &[5, 2, 0, 1, 3, 1, 4];
    println!("Sum: {:?}", sum_u32(numbers));

    let large_numbers = &[u32::MAX, 1];
    println!("Sum: {:?}", sum_u32(large_numbers));
}
