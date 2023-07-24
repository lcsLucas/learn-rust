# Variáveis e Mutabilidade

`[1]` Referência: <https://rust-book.cs.brown.edu/ch03-01-variables-and-mutability.html>

Em Rust por padrão as variáveis são imutáveis (não podem ser alteradas após a definição), isso ajuda no desenvolvimento de um sistema, impedindo que uma variável seja alterada acidentalmente comprometendo o funcionamento correto do programa, lembrando que isso não é um erro da linguagem, e sim do programador, ou seja um bug.

```bash
fn main() {
    let x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}
```

Como dito anteriormente, variáveis por padrão são imutáveis, se você tentar executar o código acima, você receberá uma mensagem de erro do compilador, dizendo algo como: `cannot assign twice to immutable variable`. Mas não se preocupe, você pode facilmente assinar uma variável como mutável quando necessário. Vamos explorar esse conceito a seguir.

#### Transformando uma variável imutável em mutável

Para transformar uma variável imutável em mutável basta adicionar a palavra `mut` antes do nome da variável.

```bash
fn main() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}
```

Agora você verá que o código acima funcionará normalmente.

## Constantes

Constantes são como variáveis imutáveis por padrão, a diferença é que constantes nunca pode ser tornar mutável como as variáveis (`let mut`). Vamos ver as c caraterísticas especificas das constantes:

* Constantes também sempre vem com sua anotação de tipo, ou seja você sempre deve informar o tipo da constante.

* Constantes podem ser declaradas em qualquer escopo, incluindo o global, tornando assim acessível por toda a parte do código.

* Constantes só aceitam expressões constantes e sua definição, ou seja não aceita o resultado de uma operação em tempo de execução (como acontece com variáveis).

```bash
const PI: f64 = 3.14159265359;
```

> Por convenção a nomeação de constantes são sempre letras maiúsculas e sublinhados nos nomes compostos (SECONDS_IN_MINUTES).

## Sombreamento (Shadowing)

Sombreamento de variável é algo particular da linguagem Rust, ela permite que uma variável seja "redeclarada", exemplo:

```bash
fn main() {
    let x = 5;

    {
        let x = 10.5;
        println!("x in scope is: {x}")
    }

    println!("x value is: {x}")

}
```

Como você pode ver no código acima, a variável x foi reutilizada no escopo, e o compilador não acusou de erro.

A vantagem de utilizar esse recurso em Rust é que você pode alterar o tipo da variável, mas manter o mesmo nome. Ex:

```bash
    let spaces =  "   ";
    let spaces = spaces.len();
```

Isso pode ser bem útil para reutilizar variáveis a qual no decorrer do código ele pode ter seu papel redefinido, como mostrado acima.

> Obs: Perceba que nesse exemplo não podemos utilizar o `mut` para fazer isso, pois a variável `spaces` muda seu tipo de `string` para `usize`.
