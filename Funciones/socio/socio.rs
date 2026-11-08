fn precio_final(precio: u32, es_socio: bool) -> u32 {
    if !es_socio {
        return precio;
    }
    precio - precio / 10
}

fn main() {
    println!("Sin ser socio: {}", precio_final(100, false));
    println!("Siendo socio: {}", precio_final(100, true));
}