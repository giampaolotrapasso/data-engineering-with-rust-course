use rand::Rng;
use vector_fruit_salad::FRUITS;
use std::io;




fn main() {

    let mut salad: Vec<&str> = Vec::new();


    println!("enter command: new, add, remove, exit");

    let mut input = String::new();
    
    let mut exit = false;
    while !exit { // change this to infinity if you want the loop to run indefinitely
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        input.pop(); // remove newline character

        match input.trim() {
            "new" => {
                salad.clear();
                for _i in 0..3 {
                    salad.push(FRUITS[rand::thread_rng().gen::<usize>() % FRUITS.len()]);
                }
                println!("New salad {:?}", salad);
            },
            "add" => {
                salad.push(FRUITS[rand::thread_rng().gen::<usize>() % FRUITS.len()]);
                println!("Updated salad {:?}", salad);
            }
            "remove" => {
                if !salad.is_empty() {
                    let index = rand::thread_rng().gen::<usize>() % salad.len();
                    salad.swap_remove(index);
                    println!("Updated salad {:?}", salad)
                } else {
                    println!("No fruit to remove");
                }
            }
            "exit" => {
                exit = true;
                println!("Bye");
            }
            _ => println!("command not found"),
        }
        input.clear();

      
    }
}