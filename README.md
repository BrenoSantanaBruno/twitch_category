# Twitch Stream Fetcher

Um pequeno projeto em Rust usando o framework Actix Web para buscar e imprimir streams do Twitch em uma determinada categoria.

## Descrição

Este projeto consiste em um servidor web simples que expõe um endpoint para buscar streams ao vivo do Twitch em uma categoria específica e uma função para buscar e imprimir essas streams no console. Ele utiliza a API oficial do Twitch e requer um token de acesso válido.

## Funcionalidades

- Busca e imprime streams ao vivo do Twitch em uma categoria específica.
- Exibe informações como nome do streamer, título da live, categoria, número de espectadores, horário de início, tags e URL da miniatura.

## Pré-requisitos

- Rust e Cargo instalados.
- Um token de acesso do Twitch.
- Um ID de cliente do Twitch.

## Instalação

1. Clone este repositório:

git clone https://github.com/BrenoSantanaBruno/Twitch-Stream-Fetcher.git


2. Na raiz do projeto, crie um arquivo `.env` e defina as variáveis de ambiente `TWITCH_ACCESS_TOKEN` e `TWITCH_CLIENT_ID` com seu token de acesso e ID de cliente do Twitch, respectivamente.

3. Compile e execute o projeto:

cargo run


## Uso

- Após iniciar o servidor, você pode acessar o endpoint `http://127.0.0.1:8080/api/twitch-streams` para obter as streams em formato JSON.

[//]: # (- As streams também serão impressas no console durante a execução.)

## Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para abrir issues e enviar pull requests.

## Licença

Este projeto é licenciado sob a Licença MIT. Consulte o arquivo LICENSE.md para obter mais detalhes.

## Autor

[Breno Santana](https://github.com/BrenoSantanaBruno)

## Agradecimentos

- [Actix Web](https://actix.rs/)
- [Reqwest](https://docs.rs/reqwest/)
- [Serde](https://serde.rs/)
- [dotenv](https://github.com/dotenv-rs/dotenv)

## Observações

Este projeto foi desenvolvido como parte de um estudo pessoal e pode não estar em conformidade com todas as práticas recomendadas ou requisitos de segurança. Use por sua própria conta e risco.
