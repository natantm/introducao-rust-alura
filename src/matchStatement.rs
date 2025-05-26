
fn main() {
  match_statement();    // Demonstra uso básico de match
  pattern_matching();   // Demonstra pattern matching com intervalos e condições
  erros();             // Demonstra tratamento de erros com Result
}

// Demonstra tratamento de erros usando Result e match
fn erros() {
  // panic!("Erro!"); // Descomente para forçar um panic
  match result() {
    Ok(v) => println!("Resultado: {}", v),
    Err(e) => println!("Erro: {}", e)
  }
}

// Função que retorna um Result, podendo ser Ok ou Err
fn result() -> Result<String, u8> {
    // Ok(String::from("Ok tudo certo"))  // Para sucesso, descomente esta linha e comente a próxima
    Err(4) // Retorna um erro com valor 4
}

// Demonstra pattern matching com intervalos, múltiplos padrões e guards
fn pattern_matching() {
  for x in 1..20 {
    println!("{} : {}", x, match x {
      1 | 2 => "Pouco",           // Para 1 ou 2
      3 | 4 => "Meio",            // Para 3 ou 4
      5..10 => "muito",           // Para 5 até 9
      _ if x % 2 == 0 => "Par",   // Para números pares (exceto os já tratados)
      _ => "Muito mesmo",         // Para todos os outros casos
    });
  }

  // Exemplo comentado de pattern matching com tuplas
  // let point = (x, y);
  // match point {
  //   (0, 0) => println!("Origem"),
  //   (_, y) => println!("Eixo Y: {}", y),
  //   (x, _) => println!("Eixo X: {}", x),
  // }
}

// Demonstra uso básico de match para seleção de valor baseado em string
fn match_statement() {
  let linguagem = "Java";
  let proposito = match linguagem{
    "PHP" => "Web",
    "Kotlin" => "Android",
    "Python" => "Data Science",
    _ => "Desconhecido" // Caso padrão
  };
  // println!("O proposito de {} é {}", linguagem, proposito);
}