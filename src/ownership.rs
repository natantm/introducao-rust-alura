fn main() {
  ownership();
}

// Demonstra conceitos de ownership e borrowing em Rust
fn ownership() {
  let uma_string = String::from("Uma string"); // Cria uma String
  rouba(&uma_string); // Passa uma referência (borrowing) para a função
  println!("A string foi devolvida {}", uma_string); // A String ainda pode ser usada aqui
}

// Função que recebe uma referência para uma String (não toma posse)
fn rouba (s: &String) {
  println!("Roubei a string {}", s);
}