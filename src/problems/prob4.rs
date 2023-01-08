pub fn p() {
    let mut x: i32 = 999;
    let mut y: i32 = 999;

    let mut product;
    let mut index;

    loop {
        product = x * y;

        if product.to_string().len() % 2 == 0 {
            index = product.to_string().len() / 2;
            
        } else if x == 1{
            y -= 1;
            x = 999;
        } else {
            x -= 1
        }
    }
    
    println!("{}", product);
    println!("{}", index);
}