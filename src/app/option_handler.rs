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
        .collect::<Vec<&str>>()
        .chunks(2)
        .filter(|pair| {
            pair.len() == 2 && pair[0].starts_with('-') && !pair[1].starts_with('-')
        })
        .map(|pair| pair.join(" "))
        .collect();

    println!("## Option List  {:?}", option_list);
    detect_options(&option_list);
    option_list
}

fn detect_options(option_list: &Vec<String>) {
    for option in option_list {
        println!("\n## Option {}", option);
        if option.starts_with("-") {
            println!("### Match Option: {}", option);
            // Option type
            // step 2 detect the option parameter (next in line)

        } else {
            // non type -> exit
            println!("### Unknown Match Option: {}", option);
        }
    }
}

fn check_if_option_is_valid(option: &String) -> bool {
    //option must be -<OPTION_NAME> <PARAMETER>
    return false
}