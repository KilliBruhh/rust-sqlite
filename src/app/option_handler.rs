#[derive(Copy, Clone)]

pub enum CommandList {
    // Helper
    help,
    clear,
    quit
}

pub fn call_option_handler(option_string: &String) -> Vec<String> {         // TODO - Change Return type
    println!("## Option String {}", option_string);
    let 
    let option_list: Vec<String> = option_string.split(" ").map(String::from).collect();

    println!("## Option List  {:?}", option_list);
    option_list
}