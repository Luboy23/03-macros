use macros::my_vec;
use anyhow::Result;

fn main() -> Result<()>{
    // let v = my_vec![1; 4];
    // let v = vec![1, 2, 3];
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
        ];

    
    println!("{:?}", v);
    Ok(())
}