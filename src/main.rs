
fn main() {
    println!("Hello, world!");
    let mut count = 0;
    for i in 0..10 {
        print!("num: {0}{0}\n count: {1}", i, count);
        count += 1;
    }

    let mut array = vec![1,2,3];
    for ar in &mut array {
        print!("num: {0}", ar);
    }
    array.push(2);
}


