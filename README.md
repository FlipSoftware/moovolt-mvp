<div align='center'>

  # MOOV.VOLT
  Transformando a Mobilidade Elétrica no Brasil

</div>

<br>
<br>
<br>
<br>

![App-workflow](https://github.com/amindWalker/moov.olt-mvp/assets/66398400/26cfcad7-c13c-4790-ba02-c5b2d14042b3)

> [!IMPORTANT]
> ## Arquitetura do Servidor
> A aplicação adere ao protocolo [**OCPP**](https://en.wikipedia.org/wiki/Open_Charge_Point_Protocol). Ela consistirá num esquema cliente-servidor com dois componentes principais: <br>
> 1. [**Serviço de Ponto de Recarga (SPR)**]()
>    - API de interação direta com estações de carregamento físicas
> 2. [**Sistema de Gerenciamento (SG)**]()
>    - Responsável por permissões, pagamentos, etc. O [**SG**]() consiste em um modelo [**cliente-servidor**]() com o [**Servidor**]() se comunicando via protocolo [**AMQP**](https://pt.wikipedia.org/wiki/Advanced_Message_Queuing_Protocol) com os [**SPRs**]() (clientes)

# 1. Serviço de Ponto de Recarga (SPR)

![SPR-API](https://github.com/amindWalker/moov.olt-mvp/assets/66398400/ca002796-e967-4a56-a226-1e0506acd47e)

- Executa tarefas fornecidas pelo [**Servidor**]().
- Responsável pela interação direta com as estações de carregamento físicas.
- Estabelece e gerencia conexões [Websocket](https://pt.wikipedia.org/wiki/WebSocket).
- Recebe e envia dados de/para as estações de carregamento.
- Não toma decisões sobre permissões ou capacidade de carregamento.

# 2. Sistema de Gerenciamento (SG)

![SG](https://github.com/amindWalker/moov.olt-mvp/assets/66398400/e26be7b5-b054-4b9d-8bc8-a353313181b4)

- Gerencia a lógica de negócios, incluindo permissões, controle do processo de carregamento e pagamentos.
- Não tem conhecimento sobre o funcionamento interno do [**SPR**]().
- Aceita dados dos serviços de [**SPR**](), toma decisões e envia tarefas de volta para execução baseda no tipo de mensagem (serviço solicitado).
- Utiliza o protocolo [**AMQP**]() para comunicação com os [**SPRs**]().

> [!IMPORTANT]
> # BACKEND

# **Rust**

![Computer](https://github.com/amindWalker/moov.olt-mvp/assets/66398400/497be7d9-ecb9-4a24-8046-c0dea8a0a7b5)

- **Alta performance e vasto ecossistema Web**
- **Uso eficiente de recursos do sistema**
- **Segurança verificável e garantida**

#### Exemplo - Servidor Axum
```rust twoslash
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));
    let sock = TcpListener::bind("0.0.0.0:3000")
      .await.unwrap(); // run async with `hyper`
    axum::serve(sock, app).await.unwrap();
}
// responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
```

# OpenAPI

<img src="https://github.com/amindWalker/moov.olt-mvp/assets/66398400/add86e0d-7bcb-4e48-91c6-1ff9926c7a74" width="500">

- **[OpenAPI](https://www.openapis.org/)** é compatível com diversos ferramentas de desenvolvimento oferecendo flexibilidade na seleção de fornecedores.
- O conhecimento comum do OpenAPI entre desenvolvedores e engenheiros proporciona flexibilidade na contratação de pessoal.
- A abstração multi-linguagem facilita a adoção de inovações nos comportamentos da API, evitando a necessidade de reescritas totais.

# RabbitMQ

<img src="https://github.com/amindWalker/moov.olt-mvp/assets/66398400/7c906ae4-238c-45dd-8466-ad02087e7216" width="500">

- O [RabbitMQ](https://www.rabbitmq.com/) suporta vários protocolos padrão abertos, incluindo [**AMQP**](https://pt.wikipedia.org/wiki/Advanced_Message_Queuing_Protocol) e [**MQTT**](https://pt.wikipedia.org/wiki/MQTT). Existem várias bibliotecas de cliente disponíveis, que podem ser usadas com a linguagem de programação de sua escolha. Sem bloqueio de fornecedor!
- Oferece muitas opções para definir como suas mensagens vão do publicador (aplicação) para um ou muitos consumidores (roteamento, filtragem, streaming, etc).
- Garantia que a troca de mensagens não será interceptada, fornecendo segurança ao consumidores da aplicação.

# PostgreSQL

<img src="https://github.com/amindWalker/moov.olt-mvp/assets/66398400/963d9eba-97a1-4cd4-a7df-f76c8c04e5a1" width="500">

- Conformidade com SQL
- Variedade rica de tipos de dados proporcionando flexibilidade na criação de diversas estruturas de dados
- [**Multi Processamento**](): recursos de indexação, transações e particionamento de tabelas favorecem operações concorrentes e processamento de alta performance.
- [**Segurança**](): possui um framework de segurança robusto com suporte para vários métodos de autenticação garantindo integridade e confidencialidade dos dados.

# Docker

![Docker](https://github.com/amindWalker/moov.olt-mvp/assets/66398400/8f2cd365-c604-4f75-b77a-eef6d0207fa9)

- Oferece ambientes isolados ([**Máquinas Virtuais Linux**](https://pt.wikipedia.org/wiki/Virtualiza%C3%A7%C3%A3o)) para aplicativos, garantindo que cada aplicativo execute sem interferência de outros.
- Os contêineres compartilham o mesmo kernel do sistema operacional em uso, resultando em uma utilização mais eficiente de recursos em comparação com máquinas virtuais tradicionais.
- [**Portabilidade garantida**](): os contêineres podem ser executados em qualquer lugar (`localhost` ou nuvem) mantendo o mesmo comportamento.

> [!IMPORTANT]
> # FRONTEND

# TypeScript

![TypeScript](https://github.com/amindWalker/moov.olt-mvp/assets/66398400/5ae875c0-b212-4b69-85a4-0cd1e0751e76)

- [**Integração com JavaScript**](): sendo apenas um superconjunto do [**JavaScript**](), oferece compatibilidade total com o vasto ecossistema [**JavaScript**]().
- [**Adição de Tipos**](): com tipagem forte e um conceito de interfaces, o TypeScript torna mais fácil trabalhar em projetos grandes e complexos, fornecendo maior clareza e segurança.
- [**Tipagem Estática**](): oferece verificação de tipos estáticos durante o desenvolvimento, detectando erros antes mesmo da execução do código.

# React

![UI-UX](https://github.com/amindWalker/moov.olt-mvp/assets/66398400/cb61cdfc-5a07-46d7-85bc-3fe9bd9fb492)

- [**Componentização**](): permite dividir a interface do usuário em componentes reutilizáveis, facilitando o desenvolvimento e manutenção de aplicações.
- [**Fluxo unidirecional**](): simplifica o gerenciamento de estado, tornando-o mais previsível e fácil de depurar. Extensível através do [**Redux**](https://redux-toolkit.js.org/).
- [**Virtual DOM**](): oferece uma atualização de página apenas nas partes necessárias da interface, resultando em um melhor desempenho e experiência do usuário.

# TailWindCSS

<img src="https://github.com/amindWalker/moov.olt-mvp/assets/66398400/069243d9-2920-49c5-b7b6-8007502328b8" width="500">

- [**Produtividade**](): oferece classes pré-definidas para estilos comuns, acelerando o processo de desenvolvimento e permitindo prototipagem rápida.
- [**Customização Flexível**](): com base em classes utilitárias, facilita a personalização de estilos sem a obrigatoriedade de escrever [**CSS**]() personalizado, proporcionando flexibilidade total.
- [**Manutenção Simplificada**](): A abordagem baseada em utilitários torna a manutenção do código mais simples, pois as alterações de estilo são centralizadas e facilmente identificáveis.

# Recursos Principais

### **Escalabilidade**
- A arquitetura permite fácil escalabilidade adicionando [**SPRs**]() adicionais, tornando-a adequada para gerenciar um grande número de estações de carregamento físicas sem depender de um único [**Servidor**]().
### **Flexibilidade e Extensibilidade**
- A separação de funções entre o [**SPR**]() e o [**SG**]() permite a fácil adição de novos recursos sem alterações significativas na arquitetura geral do sistema.
### **Gerenciamento de Desempenho**
- O sistema baseado em fila de mensagens, aliado ao backend escrito em [**Rust**](), permite multi-processamento, controle e prioridade de processamento, garantindo uma resposta rápida às solicitações dos clientes.
### **Abertura e Extensibilidade**
- Utilizando padrões abertos e tecnologias open-source populares, permite fácil integração com outros sistemas e serviços, como sistemas de pagamento, plataformas de controle e aplicações de terceiros.

# [Plano de Desenvolvimento]()

## Desenvolvimento dos componentes do backend, incluindo o [**SPR**]() e o [**SG**](), com as seguintes prioridades:
  1. Primeiramente offline (conexão de internet apenas para sincronia de dados)
  2. Monitoramento das funções do hardware
  3. Gestão financeira
  4. Cadastro de clientes
## Desenvolvimento do Frontend
  - Integração da [UI]() com o backend logo após a finalização do mesmo.
## Modelagem do Banco de Dados
  - Design e implementação do esquema do banco de dados após o cadastro de clientes e todos os testes de integridade de dados passarem como válidos.
## Melhorias e Novas Funcionalidades
  - Após todos os testes passarem como válidos, pensaremos em novas ideias e melhorias para o futuro da aplicação.
