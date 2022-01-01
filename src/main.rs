use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Chute um número");
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let x = String::from("Teste");
    println!("{}", x);
    println!("{}", &x);
    // print_type_of(x);
    print_type_of(&x);

    loop {
        println!("Começe colocando o número que quer");


        let mut guessed_number: String = String::new();

        io::stdin()
            .read_line(&mut guessed_number)
            .expect("Coloque um valor na linha");

        let guessed_number: u32 = guessed_number.trim().parse().expect(" Tem que ser número");

        println!("O valor chutado foi {}", guessed_number);
        match guessed_number.cmp(&secret_number) {
            Ordering::Less => println!("Está menor"),
            Ordering::Equal => {
                println!("acertou");
                break;
            }
            Ordering::Greater => println!("Está maior"),
        }
    }


}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}