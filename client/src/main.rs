use std::io;

fn main() {
    println!("Digite seu nome: ");

    // Por padrão o tipo let não permite mudanças na variável
    // Para que isso aconteça devemos usar o mut

    // Cria uma variável mutável do tipo string, armazenando a mesma
    let mut nome = String::new();

    // Ao ler a linha utilizando o stdin, é referenciada apenas o seu endereço de memória
    // Assim evitando que seu valor seja copiado
    io::stdin().read_line(&mut nome).expect("Erro ao ler a linha.");

    bem_vindo(&mut nome)

}

// Passar valores em uma função necessita da declaração do tipo e tamanho dos parâmetros
fn soma(x: i8, y: i8){
    println!("{}", x + y);
}

// Para passar uma string em uma função é preciso determinar seu tamanho
// Uma solução para essa caso seria utilizar o operador & para tornar o parâmetro dinâmico
fn bem_vindo(nome: &str){
    println!("Olá, {}", nome);
}