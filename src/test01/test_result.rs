fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}

fn g(i: i32) -> Result<i32, bool> {
    let t = f(i)?;

    println!("int  g(i: i32): after f is called");
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

pub fn test_result() 
{
    let r = g(-10000);
    if let Ok(v) = r {
        println!("Ok: g(10000) = {}", v);
    } else {
        println!("Err");
    }
}
 