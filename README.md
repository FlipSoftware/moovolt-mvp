<div align='center'>

  # MOOV.OLT
  **_Transformando a Mobilidade Elétrica no Brasil_**

</div>

<br>
<br>

## Visão Geral da Aplicação
![App-workflow](https://github.com/FlipSoftware/moov.olt-mvp/assets/66398400/0e6e848c-1ecb-4307-974c-59997179a5a0)

# Especificação da Aplicação

> [!IMPORTANT] 
> ## Metodologia
> Esta especificação detalha o plano de desenvolvimento do Moov.olt. Projetada para monitorar e controlar carregadores de veículos elétricos usando o protocolo [OCPP](https://en.wikipedia.org/wiki/Open_Charge_Point_Protocol). A aplicação consistirá em dois componentes principais: <br>
> 1. Um serviço de ponto de carga responsável pela interação direta com estações de carregamento físicas, e um sistema de gerenciamento que trata da lógica de negócios, como permissões, controle do processo de carregamento e pagamentos. 
> 2. A comunicação entre esses componentes será facilitada por meio de uma fila de mensagens utilizando o protocolo AMQP.

## Componentes
### 1. Serviço de Ponto de Recarga (SPR)
- Responsável pela interação direta com as estações de carregamento físicas.
- Estabelece e gerencia conexões [Websocket](https://pt.wikipedia.org/wiki/WebSocket).
- Recebe e envia dados de/para as estações de carregamento.
- Não toma decisões sobre permissões ou capacidade de carregamento.
- Executa tarefas fornecidas pelo **Sistema de Gerenciamento (SG)**.
- Utiliza o protocolo [AMQP](https://pt.wikipedia.org/wiki/Advanced_Message_Queuing_Protocol) para comunicação com o sistema de gerenciamento.

### 2. Sistema de Gerenciamento (SG)
- Gerencia a lógica de negócios, incluindo permissões, controle do processo de carregamento e pagamentos.
- Não tem conhecimento sobre o funcionamento interno do **SPR**.
- Aceita dados dos serviços de ponto de carga, toma decisões e envia tarefas de volta para execução.
- Utiliza o protocolo **AMQP** para comunicação com os **SPRs**.

## Tecnologias
- **Backend:** Rust, OpenAPI, RabbitMQ (protocolo AMQP), PostgreSQL, Docker
- **Frontend:** TypeScript, React, TailwindCSS

## Recursos Principais
- **Escalabilidade:** 
  - A arquitetura permite fácil escalabilidade adicionando serviços de ponto de carga adicionais, tornando-a adequada para gerenciar um grande número de estações de carregamento físicas.
- **Flexibilidade e Extensibilidade:** 
  - A separação de funções entre o serviço de ponto de carga e o sistema de gerenciamento permite a fácil adição de novos recursos sem alterações significativas na arquitetura geral do sistema.
- **Gerenciamento de Desempenho:**
  - O sistema baseado em fila de mensagens permite controle sobre o desempenho e a prioridade de processamento, garantindo uma resposta rápida às solicitações dos clientes.
- **Abertura e Extensibilidade:**
  - Utilizando padrões abertos e tecnologias open-source populares, permite fácil integração com outros sistemas e serviços, como sistemas de pagamento e plataformas de controle.

## Plano de Desenvolvimento
- **1. Configuração do Backend:**
  -  Desenvolvimento e configuração dos componentes do backend, incluindo o serviço de ponto de carga e o sistema de gerenciamento.
- **2. Desenvolvimento do Frontend:**
  -  Desenvolvimento da interface do usuário para interagir com a aplicação.
- **3. Modelagem do Banco de Dados:**
  -  Design e implementação do esquema do banco de dados.
- **4. Desenvolvimento:**
  -  Implementação da lógica e dos recursos da aplicação de acordo com os requisitos.
