use std::io::stdin;

fn input(x:String) -> String {
    println!("{x}");
    let mut a: String = String::new();
    stdin().read_line(&mut a).expect("enter something... \n if you did there a was an error 🤷‍♂️🤷‍♂️🤷‍♂️?????????");
    return a;
}

fn main() {
    let num1: String = input("hello the message is working".to_string());
    println!("{num1}")
    
}
