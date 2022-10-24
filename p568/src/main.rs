//Takes an array of numbers, squares each element, and sorts them based on size
fn main() {
    let mut input: [i32;5] = [-1,9,-2,0,5];
    for i in 0..input.len()
    {
        input[i] *= input[i];
    }
    bubble_sort(&mut input);
    println!("{:?}",input);
}

fn bubble_sort(arr: &mut [i32]){
    let mut swapped = true;
    let mut end_index = arr.len() - 1;
    while swapped {
        swapped = false;
        for i in 0..end_index {
            if arr[i] > arr[i+1] {
                let temp: i32 = arr[i];
                arr[i] = arr[i+1];
                arr[i+1] = temp;
                swapped = true;
            }
        }
        end_index -=1;
    }
}
