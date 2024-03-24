struct Retangulo {
    altura: u32,
    largura: u32
}

trait FormaGeometrica {
    fn calcular_area(&self) -> u32;
    fn new(largura: u32, altura: u32) -> Self;
}

impl FormaGeometrica for Retangulo {
    fn calcular_area(&self) -> u32 {
        self.largura * self.altura
    }

    fn new(largura: u32, altura: u32) -> Self {
        Self{largura, altura}
    }
}

fn main(){
    let retangulo = Retangulo::new(10, 20);

    let area:u32 = retangulo.calcular_area();

    println!("A área do retângulo é: {}",area);
}