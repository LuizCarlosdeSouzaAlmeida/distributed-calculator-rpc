# Distributed Calculator RPC

Este projeto é uma calculadora distribuída que utiliza RPC (Remote Procedure Call) para realizar operações aritméticas básicas (adição, subtração, multiplicação e divisão) entre um cliente e um servidor.

## Instalação do Rust

Para instalar o Rust, execute o seguinte comando no terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Como rodar o projeto

1. Clone o repositório:

```sh
git clone git@github.com:LuizCarlosdeSouzaAlmeida/distributed-calculator-rpc.git
cd distributed-calculator-rpc
```

2. Compile o projeto:

```sh
cargo build
```

3. Rode o servidor:

```sh
cargo run --bin server
```

4. Em outro terminal, rode o cliente:

```sh
cargo run --bin client
```

## Sobre RPC

RPC (Remote Procedure Call) é um protocolo que permite que um programa execute uma sub-rotina ou procedimento em outro endereço de espaço (normalmente em outro computador na rede) como se fosse uma chamada local. Isso facilita a comunicação entre diferentes sistemas e a distribuição de tarefas.

## Biblioteca tarpc

A biblioteca `tarpc` é uma biblioteca de Rust que facilita a criação de sistemas distribuídos utilizando RPC. Ela permite definir serviços como traits e automaticamente gera o código necessário para a comunicação entre cliente e servidor. Isso simplifica o desenvolvimento de aplicações distribuídas, abstraindo muitos dos detalhes de implementação de RPC. Para mais informações, visite o [repositório no GitHub](https://github.com/google/tarpc).

## Calculadora

A calculadora distribuída implementa as seguintes operações:

- Adição
- Subtração
- Multiplicação
- Divisão

O cliente envia a operação e os operandos para o servidor, que realiza o cálculo e retorna o resultado.
