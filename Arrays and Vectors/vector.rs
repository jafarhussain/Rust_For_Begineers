
// Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it
fn main(){
    
let v = vec![1, 2, 3, 4, 5];

println!("Vector is {:?}", v);




// To create a vector and then add elements to it, we can use the push method
// If we want to be able to change the value of any variable in Rust, we need to make it mutable using the mut keyword

let mut v = Vec::new();
v.push(2);
v.push(4);
v.push(6);

println!("After pushing values to vector: {:?}", v);

}