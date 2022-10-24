use std::collections::HashMap;
//Given any a list of numbers and some number N, determines whether any two elements in the list sum to N.
fn main() 
{  
    let mut result = false;
    let mut vals: HashMap<i32, bool> = HashMap::new();
    let input: [i32 ; 5] = [19,-1,2,3,7];
    let target: i32 = 18;
    for v in input
    {
        if vals.contains_key(&(target - v))
        {
            result = true;
            break;
        }
        vals.insert(v,true);
    }
    println!("{:?}",result);
}