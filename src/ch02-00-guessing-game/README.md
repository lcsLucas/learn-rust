# Jogo de Adivinhação

`[1]` Referência: <https://rust-book.cs.brown.edu/ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess>

Nesse jogo o programa sorteia um número aleatório de 1..100 e o usuário tem que adivinhar o número dando seus palpites através do prompt. A cada tentativa do usuário o programa verifica se ele acertou. Caso tenha acertado mostra uma mensagem parabenizando-lo e encerrando o programa. Senão, mostra uma mensagem de feedback informando se o número sorteado está acima ou abaixo do palpite informado.

## Iniciando o projeto

```bash
cargo new --vsc none guessing_game

cd guessing_game
```

## Capturando o palpite do usuário

```rust
use std::io;

fn main() {
    println!("\n### Guess the Number! ###");
    println!("\nEnter your guess: ");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Falha ao ler a linha");

    print!("Seu palpite: {guess}");
}
```

## Testando a Primeira Parte

```bash
cargo run
```

Se tudo tiver Ok, o programa executará e mostra a mensagem:

```bash
...

Guess the Number!

Enter your guess:
10
Your Guess: 10
```

## Gerando o número secreto aleatório

Iremos utilizar um pacote que nos gerará o número aleatório que precisamos para deixar o jogo divertido.

Para continuarmos adicionaremos na seção `[dependencies]` do arquivo `Cargo.toml` o seguinte texto: `rand = "0.8.3"`

Após adicionado a dependência, execute:

```bash
cargo build
```

Para utilizar o pacote, execute o código:

```bash
let secret_number = rand::thread_rng().gen_range(1..=100);
```

> ##### Obs: Você terá que importar o pacote acima no arquivo `use rand::Rng`

## Comparando o palpite com o número gerado

Para comparar o input do usuário com o número gerado utilizaremos o enum `Ordering` da biblioteca `match`. `Ordering` possui três variantes que são: `Less`, `Greater` e `Equal`. veja o código abaixo:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too Big!"),
    Ordering::Equal => println!("You win!"),
}
```

> ##### Obs: Para utilizar o `Ordering` você precisa adicionar o `use std::cmp::Ordering`, `Ordering` é um método para comparar quaisquer valores que podem ser comparados (números, strings...)

Utilizamos a expressão `match`, que possui suas ramificações (braços). Muito parecido com a expressão switch em outras linguagens.

Provavelmente nesse momento seu código não deve estar rodando, certo!? e porque isso. Se você prestar atenção a variável `guess` que recebe a sugestão do usuário é do tipo String, enquanto o `number_secret`: u32 e como Rust é uma linguagem fortemente tipada, não é possível comparar variáveis de tipos diferentes. Então para resolver isso temos que converter uma das variáveis para o mesmo tipo da outra para podermos compará-las.

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

> ##### Obs: Essa linha teve ser inserida antes da expressão de `match`

Sim! Em Rust existe o conceito de `sombrear` variável, onde você pode reutilizar uma variável já existente com outro tipo. Como se fosse uma redeclaração da variável com outro tipo.
Uma vantagem do `sombreamento` em rust é de não precisar criar variações de nomes de variáveis por causa de tipo, por exemplo: `salary_str`(tipo: String) e `salary`(tipo: u32).

O método `parser` de String converte uma String para outro tipo, `:` serve para anotar um tipo, então parse sabe qual o tipo deve transformar o valor.

## Permitindo múltiplos palpites

Ok, nosso usuário já está podendo informar seu palpite. Mas isso apenas uma vez, seria interessante permitir que ele informe seu palpite até que acerte o número secreto, e podemos fazer isso com o `loop`. `Loop` é uma palavra-chave que faz com que o código dentro dele fique em loop infinito, e para parar ele utilizaremos outra palavra-chave chamada `Break`. Veja o código:

```rust
loop {
    ...
    match guess.cmp(&secret_number) {
        ...
        Ordering::Equal => {
            println!("You Win!");
            break; //<- interrompe o loop
        }
    }
}
```

> ##### Lembrando que o usuário sempre poderá interromper o programa pressionando Ctrl + C

## Aprimorando um pouco o código

Na parte de `parse` podemos apenas ignorar os inputs que ão são números, veja a modificação:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
}
```
