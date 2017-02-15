pub fn debug_insertion_sort(arr: &mut [f64]) -> &mut [f64]{
    for x in 1..arr.len(){
        let value = arr[x];
        let mut comparison_index = x - 1;
        println!("Value: {:?}, Comparison Value: {:?}", value, arr[comparison_index ]);
        let mut offset = 0; // This avoids lots of casting
        while arr[comparison_index] > value{
            arr[comparison_index  + 1] = arr[comparison_index ];
            println!("Swap: {:?}", arr);
            if comparison_index == 0{
                offset += 1;
                break
            }
            comparison_index -= 1;
        }
        arr[comparison_index + 1 - offset] = value;
        if comparison_index != x - 1{
            println!("Swap back original: {:?}", arr);
        }
    }
    println!("{:?}", arr);
    return arr
}

