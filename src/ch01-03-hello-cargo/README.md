# Hello Cargo

`cargo` é um sistema de construção e gerenciador de pacotes do Rust. Ele é como se fosse o `npm` que usamos para gerenciar dependências em javascript.

> #####  ⚠ Não é obrigatório o uso do cargo, mas em sistemas mais complexos ele se torna muito útil.

Para verificar se o `cargo` está instalado, execute:

```bash
$ cargo -version
$>_ cargo 1.69.0 (6e9a83356 2023-04-12)
```

## Iniciando um projeto com `Cargo`

Para iniciar um novo projeto utilizando o `cargo` execute o comando:

```bash
$ cargo new {{nome do projeto}}
```

Esse comando cria um diretório ({{nome do projeto}} que você definiu) e adiciona por default o sistema de versionamento `git`(para não incluir nenhum sistema de versionamento adicione a tag `--vcs none` ao comando), além dos arquivos `Cargo.toml`(contém informações do projeto e suas dependências) e o `src/main.rs`.

## Construindo e executando com `Cargo`

> #### Build

Para construir o binário de sua aplicação com o `cargo` execute o comando:

```bash
$ cargo build
```

Esse comando cria uma pasta `target` e dentro dela uma outra pasta chamada `debug`, dentro dessa pasta você encontrará o binário gerado pelo comando.

>##### Release

Quando você estiver confiante e com o projeto pronto para a produção você pode gerar o binário da aplicação com o comando:

```bash
$ cargo build --release
```

⚠️Lembrando que o binário gerado por `release` é mais otimizado que o binário de `debug`

> #### Run

Para simplesmente rodar o projeto, basta executar o comando:

```bash
$ cargo run
```

Ele faz exatamente o mesmo passos que o processo de build, a diferença é que ele já executa o binário para você logo em seguida do build.

O `Cargo` é inteligente suficiente para saber quando o projeto é alterado, necessitando construir a aplicação novamente, ou apenas rodar a construção anterior, por não haver nenhuma alteração no projeto.

> #### Check

Além desses comandos, também existe o `cargo check`, que é apenas uma checagem no código sem rodar ele, mostrando se existe possíveis erro ou se está tudo ok com o código.