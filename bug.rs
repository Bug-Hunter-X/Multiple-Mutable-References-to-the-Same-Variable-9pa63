fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &mut x; // this will cause an error
    *z = 10;
}