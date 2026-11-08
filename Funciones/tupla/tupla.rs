fn dividir(dividendo: u32, divisor: u32) -> (u32, u32) {
    let cociente = dividendo / divisor;
    let resto = dividendo % divisor;
    return (cociente, resto);
}

fn main() {
    let (cociente, resto) = dividir(17, 5);
    println!("17 entre 5 da {cociente} y sobran {resto}");
}