#[derive(Copy, Clone)]

#[allow(unused)]
pub enum CommandList {
    // Helper
    Help,
    Clear,
    Quit
}

pub fn call_option_handler(option_string: &String) -> Vec<String> {         // TODO - Change Return type
    println!("## Option String [ {} ]", option_string);
    let option_list: Vec<String> = option_string.split(" ").map(String::from).collect();

    println!("## Option List  {:?}", option_list);
    option_list
}

fn detect_options(option_list: Vec<String>) {
    for option in option_list {
        println!("## Option {}", option);
        if option.starts_with("-") {
            // Option type
            // step 2 detect the option parameter (next in line)

        } else {
            // non type -> exit
        }
    }
}