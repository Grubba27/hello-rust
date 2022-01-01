use std::io;

fn main() {
    println!("Chute um número");

    println!("Começe colocando o número que quer");

    let mut guessed_number = String::new();

    io::stdin()
        .read_line(&mut guessed_number)
        .expect("Coloque um valor na linha");

    println!("O valor chutado foi {}", guessed_number);
}
