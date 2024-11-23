// Usa a lib trait FromSrt, com uma coleção de métodos.
// Por mais que não use o FromSrt em lugar algum, precisa estar no escopo pra usar
use std::env; // Lib pra usar o ambiente
use std::str::FromStr;

mod euclid;
fn main() { // Como não retorna nada, não precisa usar o ->
    // Declara uma variável local e inicia um Vetor (Vec) vazio. 
    let mut numbers = Vec::new(); // Vetor tipo u64 

    // Um for loop para processar coisas do CLI, definindo a variavel arg para cada argumento
    // e avaliando o corpo do loop

    // o modulo args retonar um iterador, que produz cada argumento sob demanda e indica quando terminamos. 
    // Iteradores são onipresentes em Rust.
    for arg in env::args().skip(1) { // Skip é pra pular o primeiro argumento do iterador, que é o nome do pragrama
        // Retorna um resultado se o parse funcionou ou não.
        numbers.push(u64::from_str(&arg).expect("error parsing argument")); 
    }

    // Verifica se tem pelo menos um elemento no vetor
    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ..."); // printa o error
        std::process::exit(1);
    }

    let mut d = numbers[0]; // d é mutable, pois guarda todos os valores do GCD 
    for m in &numbers[1..] { // Itera sobre uma referência aos elementos do vetor numbers começando do segundo elemento (ou seja, ignorando o primeiro elemento).
        // *m desreferencia m, obtendo o valor do elemento atual
        // Atualiza d com o resultado da função gcd, que recebe d e o valor atual *m como argumentos.
        d = euclid::gcd(d, *m); 
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

