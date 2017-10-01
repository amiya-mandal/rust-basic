// this program will show how const and mut and immut var work

const CHECKVAR: u32 = 300; // this is a const var
fn checkContanFun(){
    println!("consta var at diffrent scope {}", CHECKVAR);
}


fn main() {
    
    let checkvar2 : u32 = 400; // this is an immutable var
    let mut checkvar3 : u32 = 500; // this is an mut var
    // CHECKVAR = 400;  this is wrong
    checkvar3 = CHECKVAR;
    println!("const var {}", CHECKVAR);
    checkContanFun();

}