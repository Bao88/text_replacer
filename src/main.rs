use std::env;

fn main() {
    // Skip the first argument
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() == 3 {
        let replace_from: &String = &args[0];
        let replace_to: &String = &args[1];
        let text: &String = &args[2];

        println!("replaceFrom: {}", replace_from);
        println!("replaceTo: {}", replace_to);
        println!("Original text: {}", text);

        println!("New Text: {}", text.replace(replace_from, replace_to));
    } else {
        let message: &str;
        if args.len() > 3 {
            message = "There are too many arguments:";
        } else {
            message = "There are too few arguments!";
        }
        println!("{}", message);
        for arg in &args {
            println!("{}", arg)
        }
    }
}
