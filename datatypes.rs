fn main() {

// char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
 
let a = 'z';
let heart_eyed_cat = '😻';

//bool either true or false
let logical_var: bool = true;

//signed integers: i8, i16, i32, i64, i128 and isize (pointer size) 
let an_integer   = 5i64; // Suffix annotation

//unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)

//floating point: f32, f64
let a_float: f64 = 3.2;  // Regular annotation

//mutable variable 
let mut mutable = 49; // Mutable `i32`
mutable = 123;

println!("H:{}", a);

println!("H:{}", heart_eyed_cat);

println!("H:{}", logical_var);

println!("H:{}", an_integer);


println!("H:{}", a_float);

println!("H:{}", mutable);






}