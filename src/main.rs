// Constante global, valor fixo em tempo de compilação
const PI:f32 = 3.14;

// Variável global estática, pode ser alterada em bloco unsafe
static mut NUM:u64 = 312908864587657847;

// Função que soma dois números inteiros e imprime o resultado
fn soma(a:i32, b:i32) -> i32{
  println!("{} + {} = {}", a, b, a+b);
  a + b
}

// Demonstração de shadowing (sombreamento) de variáveis e escopo de blocos
fn sombra(){
  let a = 123;
  {
    let b = 456;
    println!("dentro, b = {}", b);
    let a = 777; // 'a' é sobrescrito apenas neste bloco
    println!("dentro, a = {}", a);
  }
    println!("fora, a = {}", a); // 'a' volta ao valor original fora do bloco
}

// Demonstração de tipos primitivos, constantes, variáveis mutáveis e uso de unsafe
fn escopo(){
  println!("PI = {}", PI);
  unsafe{
    println!("num = {}", NUM); // Acesso a variável estática mutável requer unsafe
  }

  let var:i16 = 128;
  println!("var = {}, tamanho = {} bytes", var, std::mem::size_of_val(&var));

  let dec:f32 = 2.5;
  println!("decimal = {}", dec);

  let mut bool = false;
  println!("var = {}, tamanho = {} bytes", bool, std::mem::size_of_val(&bool));

  let c:char = 'C';
  println!("Tamanho de char = {}", std::mem::size_of_val(&c));
}

// Demonstração do uso do match statement (switch/case do Rust)
fn exemplo_match() {
    let numero = 5;
    match numero {
        1 => println!("Um"),
        2 | 3 | 4 => println!("2,3 ou 4"),
        5..=10 => println!("Entre 5 e 10"),
        _ => println!("Outro número"),
    }

    let resultado = match numero {
        1 => "Um",
        2 | 3 | 4 => "Pequeno",
        5..=10 => "Médio",
        _ => "Grande",
    };
    println!("O número é: {}", resultado);
}

// Demonstração dos conceitos de ownership e borrowing em Rust
fn exemplo_ownership() {
    // Ownership: s1 é movido para s2, s1 não pode mais ser usado
    let s1 = String::from("hello");
    let s2 = s1; // s1 é movido para s2
    // println!("{}", s1); // Isso geraria erro
    println!("{}", s2);

    // Clone: s3 é clonado para s4, ambos podem ser usados
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4);

    // Ownership ao passar para função: s5 é movido para a função
    let s5 = String::from("ownership");
    toma_posse(s5);
    // println!("{}", s5); // Isso geraria erro
}

// Função que toma posse de uma String (ownership)
fn toma_posse(texto: String) {
    println!("{}", texto);
}

fn main (){

  escopo(); // Demonstra tipos e variáveis
  sombra(); // Demonstra escopo e shadowing
  println!("{}",  soma(2, 3)); // Soma dois números
  verifica_idade(16); // Verifica idade com if/else
  
  exemplo_match(); // Demonstra uso de match
  exemplo_ownership(); // Demonstra ownership

  let multplicador:u8 = 5;

  let mut contador:u8 = 0;

  // Estrutura de repetição while
  while contador < 10 {
    contador += 1;
    if contador == 4 {
      continue; // Pula o valor 4
    }
    println!("{} x {} = {}", multplicador, contador, multplicador * contador);
  }

  contador = 0;
  // Estrutura de repetição loop (loop infinito com break)
  loop {
    contador += 1;
    println!("{} x {} = {}", multplicador + 1, contador, multplicador + 1 * contador);

    if contador == 10 {
      break; // Sai do loop quando contador chega a 10
    }
  }
  
  // Estrutura de repetição for (intervalo de 1 a 10)
  for i in 1..11 { // 1 a 10 1..=10 tbm serve
    println!("{} x {} = {}", multplicador + 2, i, multplicador + 2 * i);
  }
}

// Função que demonstra uso de if/else e atribuição condicional
fn verifica_idade(idade:u8) {
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    if eh_maior {
        println!("Pode entrar na balada");
    } else if idade > 16 || responsavel_autorizou {
        println!("Pode entrar na bla");
    } else {
        println!("Não!!!!");
    }

    // Uso de if como expressão para atribuição
    let condicao = if eh_maior { "maior" } else { "menor" };

    println!("condicao = {}", condicao);
}