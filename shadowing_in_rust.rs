
// this program will show shadowing in rust

fn main() {
    // let create a variable
    let tempvar = 2000;
    println!("var ::{}", tempvar);
    // update this var 
    let tempvar = tempvar* tempvar;
    println!("var ::{}", tempvar);
    // add some value to previous value
    let tempvar:f64 = 3.14 *tempvar as f64;
    println!("var ::{}", tempvar);

    let tempvar = "          ";
    println!("var ::{}", tempvar);
    let tempvar = tempvar.len();
    println!("var ::{}", tempvar);




}