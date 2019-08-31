extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


// Simples jogo implementado usasndo a dependência rand
fn main() {
    println!("Adivinhe o núumero");

    // Usando o rand, gera um número aleatório entre 1 e 100
    let resposta = rand::thread_rng().gen_range(1, 101);

    // Entra em um loop até encontrar um break
    loop {
        println!("Entre com seu palpite:");

        // Por padrão o tipo let não permite mudanças na variável
        // Para que isso aconteça devemos usar o mut

        // Cria uma variável mutável do tipo string, armazenando a mesma
        let mut palpite = String::new();

        // Ao ler a linha utilizando o stdin, é referenciada apenas o seu endereço de memória
        // Assim evitando que seu valor seja copiado 
        io::stdin().read_line(&mut palpite).expect("Erro ao ler a linha.");

        // Utiliza o match no palpite para verificar caso seja um numero ou não
        // Neste programa, caso a entrada seja feita na forma de string o programa apenas ignora e continua a rodar
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue ,
        };

        println!("Você chutou: {}", palpite);

        // Comparando a respota utilizando um outro match, com o palpite e o endereço de memória da resposta
        match palpite.cmp(&resposta) {
            Ordering::Less    => println!("A resposta é maior!"),
            Ordering::Greater => println!("A resposta é menor!"),
            Ordering::Equal   => {
                println!("Você acertou!");
                break;
            }
        }
    }
}

// Programa simples para entender strings

// fn main() {
//     println!("Digite seu nome: ");

//     // Por padrão o tipo let não permite mudanças na variável
//     // Para que isso aconteça devemos usar o mut

//     // Cria uma variável mutável do tipo string, armazenando a mesma
//     let mut nome = String::new();

//     // Ao ler a linha utilizando o stdin, é referenciada apenas o seu endereço de memória
//     // Assim evitando que seu valor seja copiado
//     io::stdin().read_line(&mut nome).expect("Erro ao ler a linha.");

//     bem_vindo(&mut nome)
// }

// Para passar valores em uma função precisamos da declaração do tipo e tamanho dos parâmetros
// fn soma(x: i8, y: i8){
//     println!("{}", x + y);
// }

// Para passar uma string em uma função é preciso determinar seu tamanho
// Uma solução para essa caso seria utilizar o operador & para tornar o parâmetro dinâmico
// fn bem_vindo(nome: &str){
//     println!("Olá, {}", nome);
// }