#[derive(Copy, Clone)]

#[allow(unused)]
pub enum CommandList {
    // Helper
    Help,
    Clear,
    Quit
}

pub fn call_option_handler(option_string: &String) -> Vec<String> {         // TODO - Change Return type
    println!("## Option String {}", option_string);
    let option_list: Vec<String> = option_string
        .split_whitespace()
        .collect::<Vec<&str>>() // Temporarily collect to use chunks
        .chunks(2)
        .map(|chunk| chunk.join(" ")) // Joins "-p" and "8080" back together
        .collect();
    println!("## Option List  {:?}", option_list);
    option_list
}

fn filter_option_list(option_list: &Vec<String>) {
    for option in option_list {
        if option.contains("-") {}
    }
}
