use std::io;

fn main () {
    let a = String::from("Car");
    println!("{}", Pig_latin(&a));
    let b = String::from("Apple");
    println!("{}", Pig_latin(&b));

}

/*fn Pig_Latin (String: &String)  {
let ch = String.chars().nth(0)..unwrap();
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", s).into(),
        'a'..='z' => format!("{}-{}ay", it.as_str(), first).into(),
        _ => s.into(),
    }

}
 */

fn Pig_latin (String: &String) -> String {
    let str_String = &String[..];
    let a = String.chars().nth(0).unwrap();
    let b = String::from("-hay");
    let d = format!("-{}ay", a);

    match a {
        'a' | 'e' | 'i' | 'o' | 'u' |
        'A' | 'E' | 'I' | 'O' | 'U' => format!("{}-hay", String),
        _ => format!("{}{}", str_String[1..].to_string(), d)
    }
    }