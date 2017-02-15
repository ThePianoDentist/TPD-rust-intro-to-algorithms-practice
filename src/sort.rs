pub fn debug_insertion_sort(arr: &mut [f64]) -> &mut [f64]{
    for x in 1..arr.len(){
        let value = arr[x];
        let mut comparison_index = x as i64 - 1;
        println!("Value: {:?}, Comparison Value: {:?}", value, arr[comparison_index as usize]);
        while comparison_index >= 0 && arr[comparison_index as usize] > value{
            arr[comparison_index as usize + 1] = arr[comparison_index as usize];
            println!("Swap: {:?}", arr);
            comparison_index -= 1;
        }
        arr[(comparison_index + 1) as usize] = value;
        if comparison_index != x as i64 - 1{
            println!("Swap back original: {:?}", arr);
        }
    }
    println!("{:?}", arr);
    return arr
}

