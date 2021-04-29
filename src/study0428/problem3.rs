use std::io;

pub fn run() {
    println!("# Problem 3");

    loop {
        let mut string = String::new();
        println!("Please insert a number");
        io::stdin()
            .read_line(&mut string)
            .expect("Failed to read a line");
        string = string.trim_end().to_owned();
        if is_number(&string) {
            print_number_pretty(&string);
            break;
        }
    }
}

pub fn is_number(string: &str) -> bool {
    string.chars().all(char::is_numeric)
}

pub fn tokenize_with_index(string: &str, index: usize) -> Vec<String> {
    let mut string_vector: Vec<String> = Vec::new();
    let num_of_semicolone = string.len() / index;

    // tokenize
    for i in 0..num_of_semicolone {
        let s = &string[index * i..index * i + index];
        string_vector.push(String::from(s));
    }

    // insert tokens into string_vector
    let last_string = &string[num_of_semicolone * index..];
    if string.len() % index != 0 {
        string_vector.push(String::from(last_string));
    }
    string_vector
}

pub fn print_number_pretty(string: &str) {
    let reverse_string: String = string.chars().rev().collect();
    let mut string_vector: Vec<String> = tokenize_with_index(&reverse_string, 3);
    // isnert a semi-colone into each String
    // string_vector.iter_mut().for_each(|x| {
    //     x.push(',');
    //     *x = x.chars().rev().collect::<String>();
    // });
    
    for i in 0..string_vector.len() {
        let mut _s = &mut string_vector[i];
        _s.push(',');
        *_s = _s.chars().rev().collect::<String>();
    }
    string_vector.reverse();

    // Combine Vec<String>
    let mut string_vector: String = string_vector.iter().map(String::as_str).collect();
    if string_vector.as_bytes()[0] == 44u8 {
        string_vector.remove(0);
    }
    println!("{:0>20}", string_vector);
}
