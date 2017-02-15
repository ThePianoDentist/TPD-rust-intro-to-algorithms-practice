macro_rules! insertion_sort{
    ($print:expr, $func_name: ident) =>( 
pub fn $func_name(arr: &mut [f64]){
    for x in 1..arr.len(){
        let value = arr[x];
        let mut comparison_index = x - 1;
        if $print{println!("Value: {:?}, Comparison Value: {:?}", value, arr[comparison_index])}
        loop{
            if arr[comparison_index] <= value{
                arr[comparison_index + 1] = value;
                if $print{println!("Swap back original: {:?}", arr)}
                break;
            }
            arr[comparison_index + 1] = arr[comparison_index];
            if $print{println!("Swap: {:?}", arr)}
            if comparison_index == 0{
                arr[comparison_index] = value;
                if $print{println!("Swap back original: {:?}", arr)}
                break
            }
            comparison_index -= 1;
        }
    }
    return
}    
)
}

insertion_sort!(true, insertion_sort_debug);
insertion_sort!(false, insertion_sort);
