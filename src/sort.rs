macro_rules! insertion_sort{
    (do_print) => 
(pub fn insertion_sort_debug(arr: &mut [f64]){
    for x in 1..arr.len(){
        let value = arr[x];
        let mut comparison_index = x - 1;
        println!("Value: {:?}, Comparison Value: {:?}", value, arr[comparison_index ]);
        loop{
            if arr[comparison_index] <= value{
                arr[comparison_index + 1] = value;
                println!("Swap back original: {:?}", arr);
                break;
            }
            arr[comparison_index + 1] = arr[comparison_index];
            println!("Swap: {:?}", arr);
            if comparison_index == 0{
                arr[comparison_index] = value;
                println!("Swap back original: {:?}", arr);
                break
            }
            comparison_index -= 1;
        }
    }
    println!("{:?}", arr);
    return
});
(dont_print) => (
    
pub fn insertion_sort(arr: &mut [f64]){
    for x in 1..arr.len(){
        let value = arr[x];
        let mut comparison_index = x - 1;
        loop{
            if arr[comparison_index] <= value{
                arr[comparison_index + 1] = value;
                break;
            }
            arr[comparison_index + 1] = arr[comparison_index];
            if comparison_index == 0{
                arr[comparison_index] = value;
                break
            }
            comparison_index -= 1;
        }
    }
    return
}

    )

}

insertion_sort!(do_print);
insertion_sort!(dont_print);
