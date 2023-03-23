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
    let numbers = &[1, 2, 3, 4, 5];
    println!("Sum: {:?}", sum_u32(numbers)); // 输出：Sum: Some(15)

    let large_numbers = &[u32::MAX, 1];
    println!("Sum: {:?}", sum_u32(large_numbers)); // 输出：Sum: None
}
