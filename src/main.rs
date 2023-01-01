fn main() {
    let vec1 = vec![12, 32, 45];
    let mut vec2 = Vec::new();
    vec2.push(10);
    vec2.push(20);
    println!("{}", vec1[1]);

    for i in vec2 {
        println!("{}", i);
    }
}
