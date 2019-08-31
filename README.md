# RUST - Conceitos da Linguagem

Para criar um projeto em RUST, basta seguir o processor indicado no [link](https://www.rust-lang.org/learn/get-started) a seguir. No Windows é necessário instalar as C++ Build Tools mais recentes.

### Criando um projeto

Para criar um novo projeto é simples, devemos utilizar o gerenciador de pacotes da linguagem, que neste caso é o CARGO. Assim, no terminal podemos entrar com o comando:

```
cargo new nomedoprojeto
```

Um novo projeto será criado, sendo possível notar o arquivo src/main.rs. Este será o arquivo principal do projeto. Para compilar o programa, devemos utilizar o seguinte comando:

```
cargo run build
```
### Adicionando Dependências

Para adicionar dependências no projeto é necessário entrar no arquivo Cargo.toml, informar a dependencia a ser utilizada:

```
[dependencies]
nomedopacote = "versaodopacote"
```

E em seguinda rodar uma nova build do programa:

```
cargo build
```



