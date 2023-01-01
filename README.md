


## Vector
```rs
let vec1 = vec![12, 32, 45];
let mut vec2 = Vec::new();
vec2.push(10);
vec2.push(20);
println!("{}", vec1[1]);

for i in vec2 {
    println!("{}", i);
}
```

## Ownership
Rust guarantees memory safety with a feature called ownership. **Ownership** works differently from a garbage collector in other languages because it simply consists of a set of rules that the compiler needs to check at compile time. The compiler will not compile if ownership rules are not followed. The borrow checker is a compiler component that ensures your code follows ownership.
For languages that **don’t have a garbage collector, you need to explicitly allocate and free memory space**. This can quickly become tedious and challenging when it involves large codebases.

Thankfully, memory management is handled by the Rust compiler using the ownership model. A Rust compiler will automatically insert a drop statement to free the memory. **It uses the ownership model to decide where to free memory; when the owner goes out of scope, the memory is freed.**

**Ownership Rules**
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Structs
Some functions are connected to a particular type. These come in two forms: associated functions, and methods. **Associated functions** are functions that are defined on a type generally, while **methods** are associated functions that are called on a particular instance of a type.

A method must have a parameter named `self` of type Self as its first parameter. The type Self is an alias for the type that the `impl` block is for.
```rs
struct Programmer {
    email: String,
    github: String,
    blog: String,
}

impl Programmer {
    fn is_same_as(&self, other: Programmer) -> bool {
        return self.email == other.email;
    }
}

let pg1 = Programmer {
        email: String::from("abxd@gmail.com"),
        github: String::from("https://github.com"),
        blog: String::from("https://dev.com"),
};
println!("pg1 same as pg2? {}", pg1.is_same_as(pg2));
```