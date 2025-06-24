#[warn(dead_code)] 
pub fn test_ref_borrow()
{
    let mut s = String::from("hello");

    let r1 = &s;         // 不可变借用
    let r2 = &s;         // 另一个不可变借用
    println!("{} and {}", r1, r2); // 两个不可变借用都有效
    
    let r3: &mut String = &mut s;     // 可变借用
    // println!("{}", r1); // 错误！可变借用与不可变借用冲突
    r3.push_str(", world");

    println!("{}", r3);  // 正确
}


pub fn test_ref_1()
{
    let mut s1 = String::from("hello");  // s1 拥有这个字符串
    let s2 = &s1;                 // 不可变借用

    println!("{}", s1);    
    // 不可变引用本身不阻止原始变量被修改，但修改操作可能会使引用失效       
    // s1.push_str("string eee");

    println!("{}", s2);           
}