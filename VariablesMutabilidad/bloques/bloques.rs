fn main() {
    let dato = "Rustacean";

    {
        let dato = dato.len();
        println!("Dentro: {dato}");
    }

    println!("Fuera: {dato}");
}
