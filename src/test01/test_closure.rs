// test_closure.rs

pub fn test_closure() 
{
    let mut num = String::from("123456");

    // 按引用捕获
    let print_num = || println!("num = {}", num);
    print_num(); // 输出: num = 5
 

    // 按值捕获
    let take_num = move || println!("num taken = {}", num);
    // num.push_str("-7890"); // num 所有权已经被转移
    take_num(); // 输出: num taken = 5
    // num =111;
    // println!("{}", num); // 若取消注释，将报错，num 所有权被转移

    // 可变借用捕获
    // let mut change_num = || num.push_str("-dddd");
    // change_num();  
    // println!("num after closure = {}", num); // 输出: num after closure = 6
}