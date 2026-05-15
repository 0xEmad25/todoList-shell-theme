use std::{io::{self, Write}, usize};


fn main(){
    let mut list: Vec<String> = vec!["test".to_string()];
    loop{
        print!(">");
        io::stdout().flush().expect("There is a problem on the flush");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("The input is wrong fix it");
        let mut rest: Vec<&str> = input.split_whitespace().collect();
        let command = rest[0];
        rest.remove(0);
        let sentence = rest.join(" ");
        match command {
            "exit" => break,
            "ls" => {
                for i in &list{
                    println!("{}", i);
                }
            },
            "add" => {
                println!("{} has been added to the list.", sentence);
                list.push(sentence);
            },
            "done" => {
                match sentence.parse::<usize>() {
                    Ok(num) => {if num < list.len() {
                        println!("{} has been deleted from the list.", list[num]);
                        list.remove(num);
                    } else {
                        println!("Number is out of range");
                    }
                },
                    Err(_) => println!("Invalid number"),
                }
            },
            "help" => println!("commands you can use\nls: show all items in the list\nadd: add a new item to the list\ndone [num]: delete an item from the list with the index\nexit: close the program"),
            _ => println!("Invalid command please try again you can use help to list commnds"),
        }
    }

}