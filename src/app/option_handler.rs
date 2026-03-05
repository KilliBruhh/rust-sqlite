#[derive(Copy, Clone)]

#[allow(unused)]
pub enum CommandList {
    // Helper
    Help,
    Clear,
    Quit
}

    // INPUT test1 -t test1 -t test2 - t test3 -t test4 -tt test5
    // test1 -t test1 -t test2 - test 3 -t test
pub fn call_option_handler(option_string: &String) -> Vec<String> {         // TODO - Change Return type
    println!("## Option String {}", option_string);

    let option_pairings: Vec<String> = option_string
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .filter(|pair| {
            println!("## Pair[0] {}", pair[0]);
            println!("## Pair[1] {}", pair[1]);
            let valid = pair[0].starts_with('-')
                && pair[0].len() == 2
                && !pair[1].starts_with('-');
            // Base validarion
            // next step --> Ignore invalid until new valid pair[0]
            if valid {
                println!("--V PASS")
            } else {
                println!("--X NO PASS");
            }
            println!(" ");
            return pair[0] == option_string;
        })
        .map(|pair| pair.join(" "))
        .collect();
        println!("## Option List  {:?}", option_pairings);
        option_pairings
}


/*
    let option_list: Vec<String> = option_string
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .filter(|pair| {
            pair.len() == 2 && pair[0].starts_with('-')  && pair[0].len() > 1 && !pair[1].starts_with('-')
        })
        .map(|pair| pair.join(" "))
        .collect();
    println!("## Option List  {:?}", option_list);
    option_list
 */