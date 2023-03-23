fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    // arr_i32
    let mut arr_i32: Vec<i32> = vec![1, 2, 3, 4, 5];
    bubble_sort(&mut arr_i32);
    println!("Sorted i32 array: {:?}", arr_i32);
    // arr_f64
    let mut arr_f64: Vec<f64> = vec![5.2, 1.3, 1.4, 33.68, 57.56];
    bubble_sort(&mut arr_f64);
    println!("Sorted f64 array: {:?}", arr_f64);
    // arr_str
    let mut arr_str: Vec<&str> = vec!["oneblock", "Substrate", "rust", "node", "validator"];
    bubble_sort(&mut arr_str);
    println!("Sorted str array: {:?}", arr_str);
}
