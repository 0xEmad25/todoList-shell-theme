use std::io::{self, Write};

fn add(mut temp: Vec<&str>) -> String{
    temp.remove(0);
    return temp.join(" ");
}


fn main() {
    let mut list: Vec<String> = vec!["Emad".to_string()];
    loop{
        print!(">");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("the input wrong some how!!!");
        let temp: Vec<&str> = input.trim().split(" ").collect();
        if temp[0] == "add" {
            let x = add(temp);
            list.push(x.clone());
            println!("{} added to the list!", x);
        }
        else if temp[0] == "ls"{
            for i in &list{
                println!("{}", i);
            }
        }
        else if temp[0] == "exit"{
            break;
        }
        else if temp[0] == "done" {
            let num = temp[1].parse::<usize>().unwrap();
            if list.len() <= 0{
                println!("the list is empty");
                continue;
            }
            else if num > list.len(){
                println!("the number is higher than what is inside the list!");
            }
            else{
                list.remove(num);
                println!("{} has been deleted!", num);
            }
        }
    }
}

