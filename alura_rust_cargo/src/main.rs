// Função principal, ponto de entrada do programa
fn main() {
    // Array de 4 notas, todas iniciadas com 6.5
    let notas:[f32; 4] = [6.5; 4];
    let inteiro: usize = 0;

    // Acessando elemento do array pelo índice
    println!("{}", notas[inteiro]);

    // Iterando sobre o array com for
    for indice in 0..notas.len() {
        println!("A nota {} é {}", indice + 1, notas[indice]);
    }

    matriz(); // Demonstra uso de matrizes (arrays 2D)
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Terca)); // Enum e match
    cores(); // Enum avançado e pattern matching
    Conteudo_opcional(); // Option e pattern matching
    vector(); // Uso de Vec (vetor dinâmico)
    conta_corrente(); // Structs e métodos
}

// Enum para os dias da semana
enum DiaDaSemana{
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

// Função que verifica se o dia é fim de semana usando match
fn eh_fim_de_semana(dia:DiaDaSemana) -> bool{
    match dia{
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false,
    }
}

// Enum para representar cores, incluindo variantes com dados
#[allow(dead_code)]
enum Color{
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    White,
    Purple,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8},
}

// Demonstra pattern matching com enums e destructuring
fn cores(){
    // let cor:Color = Color::CymkColor { cyan: 100, magenta: 50, yellow: 21, black: 255 };
    let cor:Color = Color::RgbColor(255, 64, 0);
    println!("Cor = {}", match cor{
        Color::Red => "Vermelho",
        Color::Green => "Verde",
        Color::Blue => "Azul",
        Color::Yellow => "Amarelo",
        Color::Black | 
            Color::RgbColor(0, 0, 0) |
            Color::CymkColor { cyan:_, magenta:_, yellow:_, black:255 }=> "Preto",
        Color::White => "Branco",
        Color::Purple => "Roxo",
        Color::RgbColor(_, green, _) => {
            println!("Verde = {}", green);    
            "Cor RGB Desconhecida"
        },
        Color::CymkColor { cyan:_, magenta:_, yellow:_, black:_ } => "CYMK desconhecido",
    })
}

// Demonstra uso de arrays 2D (matriz)
#[allow(dead_code)]
fn matriz(){
    let matriz = [
        [0.0, 1.0, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);            
        }
    }
}

// Demonstra uso de Option e pattern matching
fn Conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo{
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo não existe")
    };
    println!("{:?}", conteudo_arquivo);

    // if let para extrair valor de Option
    if let Some(valor) = conteudo_arquivo {
        println!("Agora sei que tem valor {}", valor);
    }
}

// Demonstra uso de Vec (vetor dinâmico) e métodos associados
#[allow(dead_code)]
fn vector() {
    let mut notas: Vec<f32> = Vec::with_capacity(7); // Vetor com capacidade inicial 7
    notas.push(2.4);
    notas.push(2.1);
    notas.push(7.9);
    println!("Capacidade = {}", notas.capacity());
    let indice:usize = 10;
    notas.push(10.0);
    println!("Capacidade = {}", notas.capacity());
    notas.push(8.98);
    notas.push(5.5);

    println!("{:?}", notas);

    notas.push(6.8);
    println!("Capacidade = {}", notas.capacity());
    println!("{:?}", notas);

    // Acessando elemento com get para evitar panic
    println!("nota[{}] = {}", indice, match notas.get(indice){
        Some(n) => *n,
        None => 0.0
    });

    // while let Some(nota) = notas.pop(){
    //     println!("Último valor = {}", nota);
    // }

    // Iterando sobre referências dos elementos
    for nota in &notas{
        println!("Notas = {}", nota);
    }
    println!("{:?}", notas);
}

// Função que simula leitura de arquivo, retorna Option
#[allow(dead_code)]
fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    // None
}

// Struct para conta bancária, com titular e saldo
struct Conta {
    titular: Titular,
    saldo: f64
}

// Implementação de métodos para Conta
impl Conta {
    // Método para sacar valor, requer &mut self
    fn sacar(&mut self, valor: f64){
        self.saldo -= valor;
    }
}

// Struct para titular da conta
struct Titular {
    nome: String,
    sobrenome: String
}

// Demonstra uso de structs, métodos e ownership
fn conta_corrente(){
    let  titular:Titular = Titular{nome: String::from("Joãozinho"), sobrenome: String::from("Silva")};
    let mut conta: Conta = Conta { titular, saldo: 2.0 };
    
    conta.sacar(50.9); // Tenta sacar valor maior que o saldo

    println!("Titular: {} {}\nSaldo: {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}