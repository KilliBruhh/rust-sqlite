#[derive(Copy, Clone)]

#[allow(unused)]
pub enum CommandList {
    // Helper
    Help,
    Clear,
    Quit
}

    // INPUT test1 -t test1 -t test2 - t test3 -t test4 -tt test5
pub fn call_option_handler(option_string: &String) -> Vec<String> {         // TODO - Change Return type
    println!("## Option String {}", option_string);

//        1. Slide through tokens pairwise (not non-overlapping chunks)
//        2. Check if token[i] matches -X pattern (dash + single letter only)
//        3. And token[i+1] is NOT an option (doesn't start with -)
//        4. Then combine them

    let option_pairings: Vec<String> = option_string
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .filter(|pair| {
            println!("### Pairing [{} {}]", pair[0], pair[1]);
            if (pair.len() == 2 && pair[0].starts_with('-')  && pair[0].len() > 1 && !pair[1].starts_with('-')) {
                println!("-> TRUE");
            } else if (pair[0].contains(' ') || pair[1].contains(' '))  {
                println!("-> FALSE 1");
            } else {
                println!("-> NO DETECT");
            }

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