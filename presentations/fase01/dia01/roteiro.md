# 🎬 ROTEIRO COMPLETO DE VÍDEO-AULAS - DIA 1
## Setup e Hello World: A Oficina do Ferreiro

---

## 📋 VISÃO GERAL DO CURSO EM VÍDEO

**Estrutura:** 5 vídeos independentes (mas sequenciais)
**Duração Total:** ~90 minutos
**Público-alvo:** Iniciantes absolutos em programação
**Tom:** Descontraído, didático, encorajador

---

## 🎥 VÍDEO 1: INTRODUÇÃO E INSTALAÇÃO DO RUST
**Duração:** ~20 minutos
**Arquivo:** `dia01_video01_instalacao.mp4`

---

### [00:00 - 00:45] ABERTURA E BOAS-VINDAS

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal mostrando o apresentador
- Iluminação natural ou ring light
- Fundo neutro ou desfocado
- Expressão animada e acolhedora

**🎤 TEXTO PARA O APRESENTADOR:**

> E aí, pessoal! Sejam muito bem-vindos ao nosso curso de Rust do absoluto zero! 🦀
>
> Eu sou o **Thiago Bianeck** e hoje é um dia especial - é o DIA 1 da sua jornada como desenvolvedor Rust!
>
> Agora, antes de você pensar "ah, programação é muito difícil, não é pra mim" - CALMA! Respira fundo! 
>
> Eu vou te contar um segredo: **todo mundo** que hoje é programador experiente já foi um completo iniciante um dia. Inclusive eu!
>
> E sabe o que mais? Rust é uma das melhores linguagens para você começar! Sim, você ouviu certo. Muita gente fala que Rust é difícil, mas eu discordo completamente. Rust é **desafiador**, sim, mas ele te ensina os conceitos certos desde o início.
>
> É como aprender a dirigir com um instrutor super atencioso do seu lado, que te avisa: "Opa, atenção aqui! Você esqueceu de olhar o retrovisor!" - antes de você cometer o erro.

**🎬 TRANSIÇÃO:** Fade suave para tela compartilhada

---

### [00:45 - 03:00] A HISTÓRIA DO FERREIRO (ANALOGIA PRINCIPAL)

**🎬 DICA DE GRAVAÇÃO:**
- Tela dividida: apresentador em janela pequena (canto inferior direito)
- Slide ou animação ilustrando a analogia
- Ícones/imagens de ferreiro, oficina, ferramentas

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de instalarmos qualquer coisa, deixa eu te contar uma história...
>
> Imagina que você acabou de herdar uma oficina de ferreiro antiga do seu avô. Você abre a porta e... a oficina tá VAZIA! Não tem bigorna, não tem martelo, não tem NADA!
>
> Aí você tem três opções:
>
> **OPÇÃO 1** - você vai no mercado e compra cada ferramenta separadamente. Compra um martelo aqui, uma bigorna ali, um forno acolá... Mas aí você descobre que o martelo não encaixa direito na bigorna, o forno não aquece na temperatura certa... É uma bagunça!
>
> Essa é a realidade de muitas linguagens de programação antigas. Você instala uma coisa aqui, outra ali, e torce pra funcionar junto.
>
> **OPÇÃO 2** - você contrata um mestre ferreiro que traz as próprias ferramentas dele. Tudo funciona perfeitamente... mas você nunca aprende como as ferramentas funcionam de verdade. Você fica dependente dele pra sempre.
>
> Isso é tipo usar plataformas no-code ou low-code. É rápido, mas você não aprende de verdade.
>
> **OPÇÃO 3** - e aqui é onde Rust entra! Você recebe um **KIT COMPLETO DE FERREIRO PROFISSIONAL**! Vem tudo junto: bigorna, martelo, forno, moldes... E melhor ainda: vem com um **manual ilustrado** super didático que te ensina a usar cada ferramenta!
>
> E sabe o mais legal? Esse kit funciona **exatamente igual** no Windows, no Linux, no Mac... É como ter uma oficina mágica que se adapta a qualquer lugar do mundo!
>
> **ISSO É RUST!** 🦀
>
> Quando você instala Rust, você não recebe só um "compilador" - aquela ferramenta que transforma código em programa. Você recebe um pacote COMPLETO:
>
> - **rustc** → o ferreiro mestre que transforma metal bruto em espadas afiadas
> - **cargo** → o gerente da oficina que organiza tudo pra você
> - **rustfmt** → o polidor que deixa suas peças bonitas e uniformes
> - **rust-analyzer** → o assistente inteligente que te guia enquanto você trabalha
>
> E hoje, neste vídeo, a gente vai montar essa oficina completa no seu computador!

**🎬 TRANSIÇÃO:** Zoom suave no slide → transição para tela do navegador

---

### [03:00 - 03:30] OBJETIVOS DO VÍDEO

**🎬 DICA DE GRAVAÇÃO:**
- Tela compartilhada mostrando lista de objetivos
- Cursor destacando cada item conforme menciona
- Apresentador em janela PIP (picture-in-picture) opcional

**🎤 TEXTO PARA O APRESENTADOR:**

> Certo! Então o que a gente vai fazer neste vídeo especificamente?
>
> Ao final destes 20 minutinhos, você vai ter:
>
> ✅ **Instalado** o Rust completo no seu computador (Windows, Linux ou Mac)
> ✅ **Verificado** que tá tudo funcionando corretamente
> ✅ **Entendido** o que cada componente faz
>
> E eu vou te guiar **passo a passo**, pausando sempre que necessário pra você acompanhar. Inclusive, sinta-se à vontade pra pausar o vídeo e voltar quantas vezes precisar!
>
> Programação não é corrida - é maratona! Vai no seu ritmo! 🏃‍♂️

---

### [03:30 - 05:00] PREPARAÇÃO PRÉ-INSTALAÇÃO

**🎬 DICA DE GRAVAÇÃO:**
- Screencast mostrando o desktop limpo
- Mouse com destaque (círculo amarelo ao redor)

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de começar, deixa eu te dar umas dicas:
>
> **PRIMEIRA DICA:** Feche todos os programas que você não tá usando agora. Deixa só o navegador aberto. Isso evita conflitos durante a instalação.
>
> **SEGUNDA DICA:** Se você usa antivírus, ele pode reclamar durante a instalação. É normal! Rust é seguro, mas alguns antivírus são meio paranóicos. Se acontecer, clica em "permitir" ou "confiar".
>
> **TERCEIRA DICA:** A instalação vai baixar alguns arquivos da internet. Então certifica que sua internet tá funcionando, beleza?
>
> **QUARTA DICA - SUPER IMPORTANTE:** Eu vou mostrar a instalação no Windows aqui, mas se você usa Linux ou Mac, não se preocupa! O processo é bem parecido e eu vou deixar as instruções específicas na descrição do vídeo.
>
> Vamos começar!

---

### [05:00 - 07:30] INSTALAÇÃO NO WINDOWS - PARTE 1 (DOWNLOAD)

**🎬 DICA DE GRAVAÇÃO:**
- Screencast em tela cheia
- Mouse com círculo de destaque
- Zoom suave em elementos clicáveis
- Grave em resolução mínima 1080p

**🎤 TEXTO PARA O APRESENTADOR:**

> Beleza! Primeira coisa: abrir o navegador. Pode ser Chrome, Edge, Firefox... qualquer um serve.
>
> **[AÇÃO: Abrir navegador]**
>
> Agora, na barra de endereço aqui em cima, você vai digitar: `rustup.rs`
>
> **[AÇÃO: Digitar rustup.rs na barra]**
>
> Rust-up ponto R-S. "Rustup" é o nome do instalador oficial do Rust. Aperta ENTER.
>
> **[AÇÃO: Apertar Enter, aguardar carregar]**
>
> Olha que site minimalista! Isso é bem o estilo da comunidade Rust - direto ao ponto, sem frescura.
>
> Você vai ver um link grande aqui no centro: "rustup-init.exe (64-bit)". 
>
> **[AÇÃO: Mover mouse sobre o link, destacar]**
>
> Esse é o instalador pra Windows 64 bits. A maioria dos computadores hoje é 64 bits, então provavelmente é o seu caso.
>
> "Mas e se meu computador for 32 bits?" - Calma! Mais pra baixo na página tem outras opções. Mas 99% de vocês vai usar esse aqui mesmo.
>
> Vou clicar pra baixar.
>
> **[AÇÃO: Clicar no link]**
>
> O arquivo tem menos de 20 MB, então o download é rapidinho. Dependendo do seu navegador, o arquivo vai aparecer aqui embaixo ou vai direto pra pasta Downloads.
>
> **[AÇÃO: Mostrar o download acontecendo]**
>
> Enquanto baixa, deixa eu te explicar o que esse arquivo faz: ele é um "instalador inteligente". Ele vai detectar automaticamente o seu sistema, baixar os componentes certos, e instalar tudo configurado e pronto pra usar.
>
> **[AÇÃO: Download completa]**
>
> Pronto! Baixou!

---

### [07:30 - 12:00] INSTALAÇÃO NO WINDOWS - PARTE 2 (EXECUÇÃO)

**🎬 DICA DE GRAVAÇÃO:**
- Continuar screencast
- Aumentar zoom no terminal quando aparecer
- Deixar tempo para o espectador ler as mensagens

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora vamos executar o instalador. Vou abrir a pasta Downloads.
>
> **[AÇÃO: Abrir pasta Downloads]**
>
> Aqui! `rustup-init.exe`. Vou dar dois cliques.
>
> **[AÇÃO: Duplo clique no arquivo]**
>
> **[PAUSA - aguardar possível aviso do Windows]**
>
> Opa! O Windows mostrou um aviso dizendo "Aplicativo desconhecido" ou "Editor desconhecido". Isso é **super normal**! O Windows é meio desconfiado com arquivos novos.
>
> Não se preocupa! Clica em "Mais informações"...
>
> **[AÇÃO: Clicar em "Mais informações"]**
>
> E agora clica em "Executar assim mesmo".
>
> **[AÇÃO: Clicar em "Executar assim mesmo"]**
>
> Uma janela preta vai abrir - esse é o **terminal** ou **prompt de comando**. Não se assusta! Ele parece coisa de hacker de filme, mas é só uma interface de texto.
>
> **[AÇÃO: Terminal abre com mensagem de boas-vindas]**
>
> Olha só que legal! "Welcome to Rust!" - Bem-vindo ao Rust!
>
> Ele tá mostrando aqui as opções de instalação. Vamos ler juntos:
>
> ```
> Current installation options:
>    default host triple: x86_64-pc-windows-msvc
>      default toolchain: stable
>                profile: default
>   modify PATH variable: yes
> ```
>
> **[LER CADA LINHA APONTANDO COM O MOUSE]**
>
> O que isso significa?
>
> - **default host triple** → seu tipo de sistema (Windows 64-bit)
> - **default toolchain** → versão "stable" (estável, confiável)
> - **profile** → perfil padrão (instala tudo que você precisa)
> - **modify PATH** → vai configurar o sistema pra reconhecer os comandos Rust automaticamente
>
> Perfeito! É exatamente o que queremos!
>
> Agora olha as opções:
>
> ```
> 1) Proceed with standard installation (default - just press enter)
> 2) Customize installation
> 3) Cancel installation
> ```
>
> Como a gente tá começando, vamos de opção 1 - instalação padrão. É só apertar ENTER!
>
> **[AÇÃO: Apertar Enter]**
>
> E agora... olha aí! Um monte de texto passando rápido! 🚀
>
> **[DEIXAR MOSTRAR O PROCESSO POR ~5 SEGUNDOS]**
>
> Isso aqui são os componentes sendo baixados e instalados:
> - O compilador `rustc`
> - O gerenciador `cargo`
> - Bibliotecas padrão
> - Documentação
> - Ferramentas auxiliares
>
> Dependendo da sua internet, isso pode levar de 2 a 5 minutos. Eu vou acelerar aqui no vídeo pra não ficar entediante pra vocês!
>
> **[AÇÃO: Time-lapse ou corte para o final da instalação]**
>
> **[MOSTRAR MENSAGEM DE SUCESSO]**
>
> E... PRONTO! Olha a mensagem:
>
> ```
> Rust is installed now. Great!
> ```
>
> "Rust está instalado agora. Ótimo!" 🎉
>
> Agora vem uma parte **SUPER IMPORTANTE** que muita gente esquece e aí fica achando que não funcionou:
>
> **Você PRECISA fechar esse terminal e abrir um NOVO!**
>
> Por quê? Porque o instalador modificou as "variáveis de ambiente" do sistema - basicamente configurações que dizem onde o Rust tá instalado. Mas o terminal atual ainda não sabe dessa mudança. Só um terminal novo vai reconhecer.
>
> É tipo assim: imagina que você mudou sua senha do Wi-Fi. Seu celular que já tava conectado continua funcionando, mas se você quer conectar um celular novo, precisa da senha nova. Entendeu?
>
> Então: vou fechar esse terminal aqui.
>
> **[AÇÃO: Fechar o terminal]**

---

### [12:00 - 14:30] VERIFICAÇÃO DA INSTALAÇÃO

**🎬 DICA DE GRAVAÇÃO:**
- Mostrar como abrir o terminal (Windows + R, digitar cmd)
- Zoom no terminal
- Destacar os comandos digitados

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora vamos abrir um terminal novo e verificar se deu tudo certo!
>
> Vou ensinar o jeito mais rápido de abrir o terminal no Windows:
>
> **Apertar a tecla Windows + R** (ao mesmo tempo)
>
> **[AÇÃO: Apertar Win + R]**
>
> Abre uma janelinha pequenininha chamada "Executar". Aqui eu digito: `cmd`
>
> **[AÇÃO: Digitar cmd]**
>
> C-M-D, de "command" (comando). Aperto ENTER.
>
> **[AÇÃO: Enter, terminal abre]**
>
> E aí está nosso terminal novinho! 
>
> Agora vou digitar o comando mágico que testa se o Rust foi instalado:
>
> ```
> rustc --version
> ```
>
> **[DIGITAR DEVAGAR, MOSTRANDO CADA LETRA]**
>
> Deixa eu soletrar: R-U-S-T-C (tudo junto) ESPAÇO TRAÇO-TRAÇO (dois traços) V-E-R-S-I-O-N
>
> Esse comando pergunta pro compilador Rust: "qual sua versão?"
>
> Vou apertar ENTER...
>
> **[AÇÃO: Enter]**
>
> E...! 🥁
>
> ```
> rustc 1.91.1 (ed61e7d7e 2025-11-07)
> ```
>
> BOOOA! Apareceu a versão! Isso significa que o Rust tá instalado E funcionando perfeitamente! 🎉
>
> A versão pode ser diferente da minha aqui - Rust é atualizado a cada 6 semanas, então não esquenta se o número for outro!
>
> Agora vamos testar o Cargo, o gerenciador de projetos:
>
> ```
> cargo --version
> ```
>
> **[DIGITAR E EXECUTAR]**
>
> ```
> cargo 1.91.0 (e4b1c380b 2025-10-29)
> ```
>
> Perfeito! Cargo também tá funcionando!
>
> **Se você viu essas duas versões aparecerem, PARABÉNS! 🎉 Você instalou Rust com sucesso!**

---

### [14:30 - 16:00] INSTALAÇÃO NO LINUX (RESUMO RÁPIDO)

**🎬 DICA DE GRAVAÇÃO:**
- Pode ser slide ou screencast
- Mostrar comandos em texto grande e legível

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora, pra quem tá no Linux, o processo é ainda mais simples!
>
> Abre o terminal (Ctrl + Alt + T) e cola esse comando:
>
> ```bash
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
> ```
>
> **[MOSTRAR COMANDO EM TELA CHEIA]**
>
> Esse comando baixa e executa o instalador automaticamente.
>
> Quando ele perguntar as opções, escolhe a opção 1 (instalação padrão).
>
> No final, execute:
>
> ```bash
> source $HOME/.cargo/env
> ```
>
> **[MOSTRAR COMANDO]**
>
> E verifica com:
>
> ```bash
> rustc --version
> ```
>
> Se aparecer a versão, tá instalado! Simples assim! 😎

---

### [16:00 - 17:30] INSTALAÇÃO NO MAC (RESUMO RÁPIDO)

**🎬 DICA DE GRAVAÇÃO:**
- Similar ao Linux
- Mencionar possível instalação do Xcode Command Line Tools

**🎤 TEXTO PARA O APRESENTADOR:**

> E pro pessoal do Mac, é praticamente igual ao Linux!
>
> Abre o Terminal (pode procurar no Spotlight) e cola o mesmo comando:
>
> ```bash
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
> ```
>
> Uma observação importante: o Mac pode pedir pra instalar o "Xcode Command Line Tools". Se aparecer, clica em "Instalar" - são ferramentas de desenvolvimento que o Rust precisa.
>
> Depois da instalação, mesma coisa:
>
> ```bash
> source $HOME/.cargo/env
> rustc --version
> ```
>
> Versão apareceu? Instalado! 🎉

---

### [17:30 - 19:30] O QUE FOI INSTALADO (TOUR RÁPIDO)

**🎬 DICA DE GRAVAÇÃO:**
- Diagrama ou animação mostrando os componentes
- Pode usar o Mermaid diagram do material original

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de encerrar, deixa eu te mostrar o que exatamente foi instalado no seu computador.
>
> **[MOSTRAR DIAGRAMA/SLIDE]**
>
> Quando você instalou o Rust, vieram esses componentes:
>
> **1. rustc** - O compilador
> - Transforma código Rust em executável
> - É o coração de tudo
>
> **2. cargo** - O gerenciador
> - Cria projetos
> - Compila código
> - Gerencia dependências (bibliotecas externas)
> - Basicamente, seu melhor amigo!
>
> **3. rustup** - O atualizador
> - Mantém tudo atualizado
> - Gerencia versões do Rust
>
> **4. rustfmt** - O formatador
> - Deixa seu código bonito e padronizado
>
> **5. clippy** - O analisador
> - Dá dicas de como melhorar seu código
> - É tipo um professor particular!
>
> **6. rust-docs** - Documentação offline
> - Toda documentação no seu computador
> - Funciona sem internet!
>
> Tudo isso, de graça, funcionando junto, perfeitamente integrado! Por isso que eu disse que Rust é o kit completo! 🧰

---

### [19:30 - 20:00] ENCERRAMENTO DO VÍDEO 1

**🎬 DICA DE GRAVAÇÃO:**
- Voltar para câmera frontal mostrando o apresentador
- Expressão animada e encorajadora

**🎤 TEXTO PARA O APRESENTADOR:**

> E é isso, pessoal! Rust instalado com sucesso! 🦀
>
> Eu sei que pode parecer pouco, mas você acabou de dar um passo GIGANTE! Muita gente desiste antes mesmo de instalar. Mas você não! Você tá aqui, com tudo pronto, e agora vem a parte divertida!
>
> No próximo vídeo, a gente vai configurar o VSCode - nosso editor de código - e transformar ele em uma bancada de trabalho profissional pra programar em Rust!
>
> Se você gostou do vídeo, deixa aquele like aí, se inscreve no canal, e ativa o sininho pra não perder os próximos vídeos!
>
> Qualquer dúvida, comenta aqui embaixo que eu respondo todo mundo!
>
> Te vejo no próximo vídeo! Até lá! 👋

**[FIM DO VÍDEO 1]**

---
---

## 🎥 VÍDEO 2: CONFIGURANDO O VSCODE
**Duração:** ~15 minutos
**Arquivo:** `dia01_video02_vscode.mp4`

---

### [00:00 - 00:30] ABERTURA E RECAP

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Energia e entusiasmo

**🎤 TEXTO PARA O APRESENTADOR:**

> E aí, pessoal! Bem-vindos de volta! 🎉
>
> No vídeo anterior, a gente instalou o Rust completo no seu computador. Se você ainda não viu, pausa aqui e vai assistir - é rapidinho e super importante!
>
> **[PAUSA DE 2 SEGUNDOS]**
>
> Agora, neste vídeo, a gente vai preparar nossa **bancada de trabalho** - o editor de código onde você vai escrever seus programas Rust!
>
> E o editor que a gente vai usar é o **VSCode** - Visual Studio Code. É de graça, é leve, e é usado por milhões de desenvolvedores no mundo todo!
>
> Bora lá!

---

### [00:30 - 02:00] POR QUE VSCODE?

**🎬 DICA DE GRAVAÇÃO:**
- Slide comparando editores
- Ou screencast mostrando o VSCode

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de instalar, deixa eu te explicar por que VSCode.
>
> Existem vários editores de código por aí: Vim, Emacs, Sublime Text, IntelliJ... Então por que VSCode?
>
> **RAZÃO 1:** É **gratuito e open-source** (código aberto)
>
> **RAZÃO 2:** É **leve** - não vai travar seu computador
>
> **RAZÃO 3:** Tem **extensões incríveis** pra Rust (vamos instalar daqui a pouco)
>
> **RAZÃO 4:** É **multiplataforma** - funciona no Windows, Linux e Mac
>
> **RAZÃO 5:** É o editor **mais usado** pela comunidade Rust
>
> Lembra da analogia da oficina? O Rust é o martelo e a bigorna. O VSCode é a **bancada iluminada** com réguas, lupas, e um assistente ao seu lado te guiando!
>
> Vamos instalar!

---

### [02:00 - 05:00] INSTALANDO O VSCODE

**🎬 DICA DE GRAVAÇÃO:**
- Screencast completo
- Mostrar cada passo claramente

**🎤 TEXTO PARA O APRESENTADOR:**

> Abre o navegador e vai em: `code.visualstudio.com`
>
> **[AÇÃO: Navegar para o site]**
>
> Code ponto Visual Studio ponto com.
>
> Aqui na página inicial, tem um botão grandão "Download for Windows" (ou Mac/Linux, dependendo do seu sistema).
>
> **[AÇÃO: Clicar em Download]**
>
> O VSCode detecta automaticamente seu sistema operacional. Inteligente, né?
>
> O download vai começar - são uns 70-80 MB.
>
> **[ENQUANTO BAIXA]**
>
> Enquanto baixa, sabia que o VSCode foi criado pela Microsoft? Mas é de código aberto! Milhares de desenvolvedores contribuem pra ele ficar cada vez melhor.
>
> **[DOWNLOAD COMPLETA]**
>
> Pronto! Agora vou abrir o instalador.
>
> **[AÇÃO: Abrir instalador]**
>
> A instalação é bem padrão:
>
> 1. "I accept the agreement" → Aceitar os termos
> 2. **[AÇÃO: Marcar checkbox, Next]**
>
> 3. Escolher pasta de instalação → Pode deixar o padrão
> 4. **[AÇÃO: Next]**
>
> 5. **IMPORTANTE!** Nessa tela de "Tarefas Adicionais", marca essas opções:
>    - ✅ "Add 'Open with Code' to context menu" 
>    - ✅ "Add to PATH"
>
> **[AÇÃO: Marcar checkboxes]**
>
> Essas opções permitem abrir pastas com o VSCode clicando com botão direito - super útil!
>
> 6. **[AÇÃO: Next → Install]**
>
> Agora é só aguardar instalar... E pronto! 🎉
>
> **[AÇÃO: Finish - pode deixar marcado "Launch VSCode"]**

---

### [05:00 - 06:30] PRIMEIRO CONTATO COM O VSCODE

**🎬 DICA DE GRAVAÇÃO:**
- VSCode abre pela primeira vez
- Tour rápido pela interface

**🎤 TEXTO PARA O APRESENTADOR:**

> E aí está! O VSCode aberto pela primeira vez! 
>
> **[PAUSA PARA MOSTRAR A INTERFACE]**
>
> Deixa eu te dar um tour rápido:
>
> **[APONTAR CADA ELEMENTO COM O MOUSE]**
>
> - **Barra lateral esquerda** → Navegação (arquivos, busca, extensões)
> - **Área central** → Onde você vai escrever código
> - **Barra inferior** → Terminal integrado e informações
> - **Canto superior** → Abrir pastas e arquivos
>
> Por enquanto tá vazio porque não abrimos nenhum projeto ainda. Mas logo logo vai estar cheio de código Rust aqui!
>
> Agora vem a **parte mais importante deste vídeo**: instalar a extensão que faz o VSCode entender Rust!

---

### [06:30 - 10:00] INSTALANDO RUST-ANALYZER

**🎬 DICA DE GRAVAÇÃO:**
- Zoom na barra lateral de extensões
- Mostrar o processo passo a passo
- Destacar os recursos da extensão

**🎤 TEXTO PARA O APRESENTADOR:**

> A extensão que vamos instalar se chama **rust-analyzer**. 
>
> Ela é tipo assim: imagina que o VSCode é um tradutor que só fala português. E você quer falar japonês com ele. O rust-analyzer é o **intérprete simultâneo** que traduz tudo!
>
> Com ele, o VSCode vai:
> - ✅ Completar código pra você automaticamente
> - ✅ Mostrar erros enquanto você digita (antes de compilar!)
> - ✅ Explicar funções quando você passa o mouse
> - ✅ Formatar código automaticamente
> - ✅ Navegar entre arquivos facilmente
>
> É basicamente ter um **professor particular de Rust dentro do seu editor**! 🤓
>
> Vamos instalar! Olha aqui na barra lateral esquerda - esse ícone de quadradinhos.
>
> **[AÇÃO: Clicar no ícone de Extensions]**
>
> Esse é o gerenciador de extensões. Ou você pode usar o atalho **Ctrl + Shift + X** (Windows/Linux) ou **Cmd + Shift + X** (Mac).
>
> **[MOSTRAR O ATALHO NA TELA]**
>
> Agora, na caixa de busca aqui em cima, digita: `rust-analyzer`
>
> **[AÇÃO: Digitar devagar]**
>
> R-U-S-T-TRAÇO-A-N-A-L-Y-Z-E-R
>
> **[AÇÃO: Aparecem resultados]**
>
> Olha! Primeira opção: "rust-analyzer" - com logo laranja/vermelho e autor "rust-lang.org".
>
> **[DESTACAR A EXTENSÃO CORRETA]**
>
> **ATENÇÃO!** Tem outras extensões com nomes parecidos. Certifica que é essa aqui, a oficial, com o ícone de caranguejo! 🦀
>
> **[AÇÃO: Clicar na extensão para abrir detalhes]**
>
> Olha as informações:
> - ⭐ Avaliação alta
> - 📥 Milhões de downloads
> - ✅ Mantida oficialmente pela equipe do Rust
>
> Confiável demais!
>
> Agora clica no botão azul "Install".
>
> **[AÇÃO: Clicar em Install]**
>
> **[MOSTRAR INSTALAÇÃO ACONTECENDO]**
>
> A instalação vai levar uns 10-15 segundos...
>
> Quando terminar, o botão muda pra "Uninstall" (desinstalar) - isso significa que tá instalado!
>
> **[AÇÃO: Instalação completa]**
>
> Pronto! Agora o VSCode fala Rust fluentemente! 🦀
>
> Mas vamos fazer mais uma configuração pra deixar tudo **perfeito**!

---

### [10:00 - 12:30] CONFIGURAÇÕES RECOMENDADAS

**🎬 DICA DE GRAVAÇÃO:**
- Navegar pelas configurações
- Explicar cada opção

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora vamos ativar algumas configurações que vão facilitar MUITO sua vida.
>
> Abre as configurações: **Ctrl + ,** (Control + Vírgula) no Windows/Linux, ou **Cmd + ,** no Mac.
>
> **[AÇÃO: Abrir Settings]**
>
> Aqui temos zilhões de configurações! Mas vamos mexer só nas importantes.
>
> **CONFIGURAÇÃO 1: Format On Save**
>
> Na caixa de busca, digita: `format on save`
>
> **[AÇÃO: Digitar e buscar]**
>
> Aqui! "Editor: Format On Save". Marca essa caixinha.
>
> **[AÇÃO: Marcar checkbox]**
>
> O que isso faz? Toda vez que você salvar um arquivo Rust (Ctrl + S), o código vai ser **formatado automaticamente**! 
>
> É tipo assim: você escreveu um texto meio bagunçado, e quando salva, ele automaticamente corrige os espaços, alinha tudo bonitinho...
>
> Programadores profissionais **sempre** usam formatação automática! Economiza tempo e evita briga na equipe sobre "onde colocar o espaço". 😄
>
> **CONFIGURAÇÃO 2 (Opcional): Auto Save**
>
> Agora busca: `auto save`
>
> **[AÇÃO: Buscar]**
>
> "Files: Auto Save" - troca de `off` pra `afterDelay`.
>
> **[AÇÃO: Mudar opção]**
>
> Isso faz o VSCode salvar automaticamente depois de 1 segundo que você para de digitar. Nunca mais perde código por esquecer de salvar!
>
> Mas essa é **opcional** - se você prefere controlar quando salva, deixa em `off` mesmo.
>
> **CONFIGURAÇÃO 3: Inlay Hints (Dicas na Tela)**
>
> Busca: `rust-analyzer inlay hints`
>
> **[AÇÃO: Buscar]**
>
> Aqui você vai ver várias opções de "Rust-analyzer › Inlay Hints". Recomendo deixar **todas ativadas**!
>
> Essas "inlay hints" mostram informações extras no código - tipos de variáveis, nomes de parâmetros... Vão fazer muito mais sentido quando a gente começar a programar!
>
> Pronto! Configurações feitas! ✅

---

### [12:30 - 13:30] EXTENSÕES OPCIONAIS (MENÇÃO RÁPIDA)

**🎬 DICA DE GRAVAÇÃO:**
- Mostrar extensões rapidamente
- Não instalar agora, só mencionar

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de terminar, deixa eu mencionar algumas extensões **opcionais** que você pode instalar depois:
>
> **[MOSTRAR LISTA EM SLIDE]**
>
> - **Error Lens** → Mostra erros coloridos direto na linha (bem visual!)
> - **Better TOML** → Facilita editar arquivos Cargo.toml
> - **CodeLLDB** → Pra fazer debug avançado no futuro
> - **Material Icon Theme** → Ícones bonitinhos (puramente estético!)
>
> Mas por enquanto, **só o rust-analyzer já é mais que suficiente**! 
>
> Não precisa encher o VSCode de extensões agora. Instala mais coisas conforme você for sentindo necessidade!

---

### [13:30 - 14:30] DEMONSTRAÇÃO RÁPIDA (TEASER)

**🎬 DICA DE GRAVAÇÃO:**
- Abrir um arquivo .rs de exemplo
- Mostrar autocompletar funcionando
- NÃO explicar o código ainda

**🎤 TEXTO PARA O APRESENTADOR:**

> Deixa eu te mostrar rapidinho como o rust-analyzer é poderoso!
>
> Vou criar um arquivo de teste só pra demonstrar...
>
> **[AÇÃO: Criar arquivo test.rs na área de trabalho]**
>
> Agora vou começar a digitar código Rust...
>
> **[AÇÃO: Digitar "fn mai"]**
>
> Olha! Ele já sugere `main`! Se eu apertar Tab, ele completa automaticamente!
>
> **[AÇÃO: Completar e continuar digitando]**
>
> E olha aqui - quando eu começo a digitar `println`, ele já mostra a documentação, como usar...
>
> **[MOSTRAR TOOLTIP]**
>
> Incrível, né? É como ter um assistente super inteligente! 🧙‍♂️
>
> No próximo vídeo, a gente vai escrever nosso primeiro programa de verdade e você vai ver isso em ação!
>
> **[FECHAR O ARQUIVO SEM SALVAR]**

---

### [14:30 - 15:00] ENCERRAMENTO DO VÍDEO 2

**🎬 DICA DE GRAVAÇÃO:**
- Voltar pra câmera frontal
- Tom motivador

**🎤 TEXTO PARA O APRESENTADOR:**

> E pronto! VSCode configurado e pronto pra programar Rust! 🦀
>
> Recapitulando o que fizemos:
> - ✅ Instalamos o VSCode
> - ✅ Instalamos o rust-analyzer
> - ✅ Configuramos formatação automática
> - ✅ Testamos que tá tudo funcionando
>
> Agora sim, sua oficina tá completa! Ferramentas instaladas, bancada organizada. Falta só uma coisa: **forjar sua primeira peça**!
>
> E é exatamente isso que a gente vai fazer no próximo vídeo - criar nosso primeiro projeto Rust e escrever o famoso "Hello, World!"!
>
> Se você tá gostando da série, deixa aquele like, compartilha com os amigos, e comenta aqui embaixo: "Configurado e pronto!"
>
> Te vejo no próximo vídeo! Até já! 👋

**[FIM DO VÍDEO 2]**

---
---

## 🎥 VÍDEO 3: CRIANDO SEU PRIMEIRO PROJETO
**Duração:** ~18 minutos
**Arquivo:** `dia01_video03_primeiro_projeto.mp4`

---

### [00:00 - 00:45] ABERTURA E CONTEXTO

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Energia crescente

**🎤 TEXTO PARA O APRESENTADOR:**

> E aí, galera! Chegou a hora! 🎉
>
> Nos últimos dois vídeos, a gente preparou o terreno: instalamos Rust, configuramos o VSCode... E AGORA, finalmente, vamos **escrever código de verdade**!
>
> Neste vídeo, você vai:
> - Criar seu primeiro projeto Rust
> - Entender como projetos são organizados
> - Escrever o famoso "Hello, World!"
> - Rodar seu programa pela primeira vez
>
> E eu vou te garantir uma coisa: até o final deste vídeo, você vai ter criado um **programa executável de verdade** que roda no seu computador! 
>
> Não vai ser "código de mentirinha" rodando num site. Vai ser um programa DE VERDADE no seu sistema!
>
> Preparado? Bora lá! 🚀

---

### [00:45 - 03:00] O QUE É O CARGO (REVISÃO E ANALOGIA)

**🎬 DICA DE GRAVAÇÃO:**
- Slide ou animação explicando o Cargo
- Diagrama mostrando o que Cargo gerencia

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de criar o projeto, preciso te apresentar direito o **Cargo**.
>
> Lembra que eu disse que quando você instalou o Rust, veio um "kit completo"? O Cargo é uma das ferramentas mais importantes desse kit!
>
> **Analogia do Gerente de Oficina:**
>
> Imagina que você vai fazer um bolo. Você poderia:
> - Pegar os ingredientes de qualquer lugar
> - Usar qualquer vasilha
> - Ligar o forno no chute
> - Organizar tudo do seu jeito
>
> Funciona... mas é uma bagunça! E se outra pessoa for fazer o mesmo bolo, vai fazer tudo diferente!
>
> O Cargo é tipo um **chef organizador**. Ele:
> - Cria uma estrutura de pastas padrão (todo projeto Rust tem a mesma cara)
> - Gerencia dependências (bibliotecas externas que você usa)
> - Compila seu código (transforma .rs em executável)
> - Roda testes
> - Gera documentação
>
> **[MOSTRAR DIAGRAMA]**
>
> É o **gerente da sua oficina de desenvolvimento**! E o melhor: todo desenvolvedor Rust usa Cargo. Então quando você olha projeto de outra pessoa, já sabe exatamente onde tudo está!
>
> Vamos ver ele em ação!

---

### [03:00 - 05:30] CRIANDO O PROJETO - CARGO NEW

**🎬 DICA DE GRAVAÇÃO:**
- Screencast
- Terminal grande e legível
- Explicar cada comando antes de executar

**🎤 TEXTO PARA O APRESENTADOR:**

> Primeira coisa: precisamos de uma pasta organizada pros nossos projetos.
>
> Eu recomendo criar uma pasta chamada `projetos_rust` dentro de Documentos.
>
> Vou abrir o VSCode primeiro.
>
> **[AÇÃO: Abrir VSCode]**
>
> Agora vou abrir o terminal integrado. Menu **Terminal → New Terminal**, ou **Ctrl + '** (Control + Aspas Simples).
>
> **[AÇÃO: Abrir terminal no VSCode]**
>
> Legal! Terminal integrado aberto aqui embaixo. Assim não preciso ficar alternando entre janelas!
>
> Agora vou navegar até Documentos e criar a pasta:
>
> **[NO WINDOWS]**
> ```bash
> cd C:\Users\SeuNome\Documents
> ```
>
> **[AÇÃO: Executar comando cd]**
>
> `cd` significa "Change Directory" - mudar de pasta. É tipo clicar duas vezes numa pasta!
>
> Agora vou criar a pasta:
>
> ```bash
> mkdir projetos_rust
> ```
>
> **[AÇÃO: Executar mkdir]**
>
> `mkdir` = "Make Directory" = criar pasta.
>
> Entrar nela:
>
> ```bash
> cd projetos_rust
> ```
>
> **[AÇÃO: cd projetos_rust]**
>
> Perfeito! Agora vem o comando mágico do Cargo:
>
> ```bash
> cargo new hello_rust
> ```
>
> **[MOSTRAR O COMANDO DESTACADO NA TELA]**
>
> Vou soletrar: C-A-R-G-O ESPAÇO N-E-W ESPAÇO H-E-L-L-O-UNDERLINE-R-U-S-T
>
> Esse comando diz: "Cargo, crie um **novo** projeto chamado **hello_rust**!"
>
> Aperto Enter...
>
> **[AÇÃO: Executar]**
>
> E olha a mensagem:
>
> ```
>      Created binary (application) `hello_rust` package
> ```
>
> "Criado pacote binário (aplicação) `hello_rust`"! 🎉
>
> **Binário** significa que vai gerar um executável - um programa que roda!
>
> O Cargo acabou de criar uma pasta inteira com tudo configurado! Vamos explorar!

---

### [05:30 - 09:00] EXPLORANDO A ESTRUTURA DO PROJETO

**🎬 DICA DE GRAVAÇÃO:**
- Abrir o projeto no VSCode
- Navegação visual pela árvore de arquivos
- Destaque cada arquivo conforme explica

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora vamos abrir essa pasta no VSCode.
>
> No terminal, digito:
>
> ```bash
> cd hello_rust
> code .
> ```
>
> **[AÇÃO: Executar comandos]**
>
> O `code .` abre o VSCode na pasta atual (o ponto significa "pasta atual").
>
> **[VSCODE ABRE OU RECARREGA]**
>
> E olha só! O VSCode abriu com nosso projeto! 
>
> **[MOSTRAR A ÁRVORE DE ARQUIVOS NA LATERAL]**
>
> Vamos explorar arquivo por arquivo:
>
> **[CLICAR EM CADA ARQUIVO CONFORME EXPLICA]**
>
> ### **1. Pasta raiz: `hello_rust/`**
>
> Essa é a pasta principal do projeto. Tudo do seu programa fica aqui dentro.
>
> ### **2. Arquivo `Cargo.toml`**
>
> **[ABRIR Cargo.toml]**
>
> Esse é o **cérebro do projeto**! É a "certidão de nascimento" do seu programa.
>
> Olha o conteúdo:
>
> ```toml
> [package]
> name = "hello_rust"
> version = "0.1.0"
> edition = "2024"
>
> [dependencies]
> ```
>
> Deixa eu traduzir:
>
> - **[package]** → Seção de informações do pacote
> - **name** → Nome do seu programa
> - **version** → Versão (começa em 0.1.0)
> - **edition** → Qual versão da linguagem Rust usar (2024 é a mais recente!)
> - **[dependencies]** → Lista de bibliotecas externas (por enquanto vazio)
>
> É tipo assim: imagina um formulário de cadastro. Aqui você tá dizendo pro Rust: "Meu programa se chama isso, é a versão X, usa a edição Y..."
>
> ### **3. Pasta `src/`**
>
> **[CLICAR NA PASTA src]**
>
> `src` vem de "source" (fonte, código-fonte). É aqui que **todo seu código** vai ficar!
>
> Por enquanto tem só um arquivo...
>
> ### **4. Arquivo `src/main.rs`**
>
> **[ABRIR main.rs]**
>
> E aqui está a **MÁGICA**! 🪄
>
> Olha o código:
>
> ```rust
> fn main() {
>     println!("Hello, world!");
> }
> ```
>
> **O Cargo já criou um programa completo pra gente!** 
>
> Esse código imprime "Hello, world!" na tela. Vamos rodar antes de entender linha por linha!
>
> ### **5. Arquivo `.gitignore`**
>
> **[MOSTRAR BREVEMENTE]**
>
> Esse arquivo diz pro Git (sistema de controle de versão) quais arquivos ignorar. Não precisa se preocupar com ele agora!

---

### [09:00 - 11:00] RODANDO O PROGRAMA PELA PRIMEIRA VEZ

**🎬 DICA DE GRAVAÇÃO:**
- Terminal em destaque
- Slow motion na saída do programa (edição)
- Comemoração! 🎉

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora vem o momento que você tava esperando: **RODAR O PROGRAMA**! 🚀
>
> Lá no terminal integrado (aqui embaixo), vou digitar o comando mais importante que você vai usar:
>
> ```bash
> cargo run
> ```
>
> **[MOSTRAR O COMANDO DESTACADO]**
>
> Simples assim! `cargo run` = "Cargo, rode o programa!"
>
> Esse comando faz duas coisas:
> 1. **Compila** o código (transforma .rs em executável)
> 2. **Executa** o programa
>
> Vou apertar Enter... Cruza os dedos! 🤞
>
> **[AÇÃO: cargo run]**
>
> **[DEIXAR A SAÍDA APARECER]**
>
> ```
>    Compiling hello_rust v0.1.0 (C:\...\hello_rust)
>     Finished dev [unoptimized + debuginfo] target(s) in 2.45s
>      Running `target\debug\hello_rust.exe`
> Hello, world!
> ```
>
> **[PAUSA DRAMÁTICA]**
>
> OLHA AÍ! "Hello, world!" 🎉🎊🥳
>
> **PARABÉNS! VOCÊ ACABOU DE RODAR SEU PRIMEIRO PROGRAMA RUST!**
>
> Pode não parecer muito, mas você acabou de fazer algo **incrível**:
> - Compilou código-fonte
> - Gerou um executável
> - Rodou no seu sistema operacional
>
> Esse é um programa DE VERDADE! Vamos entender o que aconteceu linha por linha:

---

### [11:00 - 13:30] ENTENDENDO A SAÍDA DO CARGO RUN

**🎬 DICA DE GRAVAÇÃO:**
- Zoom na saída do terminal
- Destacar cada linha conforme explica

**🎤 TEXTO PARA O APRESENTADOR:**

> Vamos entender cada linha da saída do `cargo run`:
>
> **[DESTACAR LINHA 1]**
>
> ```
> Compiling hello_rust v0.1.0 (C:\...\hello_rust)
> ```
>
> **"Compilando hello_rust versão 0.1.0"**
>
> O Cargo pegou seu código Rust (main.rs) e começou a transformar em código de máquina (executável).
>
> Compilar é tipo traduzir: você escreveu em "Rust" (linguagem humana-ish), e o compilador traduziu pra "binário" (linguagem que o computador entende).
>
> **[DESTACAR LINHA 2]**
>
> ```
> Finished dev [unoptimized + debuginfo] target(s) in 2.45s
> ```
>
> **"Finalizado dev [não otimizado + info de debug] em 2.45 segundos"**
>
> - **dev** → modo desenvolvimento (compila rápido, programa roda mais devagar)
> - **unoptimized** → código não otimizado (otimizar demora mais)
> - **debuginfo** → informações extras pra ajudar a encontrar bugs
> - **2.45s** → demorou 2.45 segundos pra compilar
>
> Na primeira vez sempre demora um pouco. Nas próximas vezes é instantâneo se você não mudar o código!
>
> **[DESTACAR LINHA 3]**
>
> ```
> Running `target\debug\hello_rust.exe`
> ```
>
> **"Executando o arquivo target\debug\hello_rust.exe"**
>
> Agora o Cargo tá **rodando** o executável que ele acabou de criar!
>
> Olha o caminho: `target/debug/hello_rust.exe` - é um arquivo `.exe` de verdade! Um programa Windows!
>
> **[DESTACAR LINHA 4]**
>
> ```
> Hello, world!
> ```
>
> **E ESSA é a saída do seu programa!** 🎉
>
> Foi o `println!` lá no código que imprimiu isso!
>
> Tudo antes dessa linha foi o Cargo trabalhando. Tudo depois (nesse caso, só essa linha) é o SEU programa rodando!

---

### [13:30 - 15:00] EXPLORANDO O EXECUTÁVEL GERADO

**🎬 DICA DE GRAVAÇÃO:**
- Navegador de arquivos mostrando target/debug/
- Executar o .exe direto (fora do Cargo)
- Provar que é um programa real

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora vem uma parte legal: vamos encontrar o executável que foi criado!
>
> Lembra que a saída disse `target\debug\hello_rust.exe`? Vamos até lá!
>
> **[AÇÃO: Abrir explorador de arquivos na pasta do projeto]**
>
> Olha, tem uma pasta nova aqui: `target/`
>
> **[ENTRAR EM target/]**
>
> Dentro dela: `debug/`
>
> **[ENTRAR EM debug/]**
>
> E aqui! Vários arquivos, mas olha esse aqui: `hello_rust.exe`! 
>
> **[DESTACAR O .exe]**
>
> Esse é seu programa! Um executável de verdade! Você pode rodar ele **sem o Cargo**, sem o Rust instalado, até em outro computador!
>
> Vou clicar duas vezes nele...
>
> **[AÇÃO: Duplo clique no .exe]**
>
> Uma janela preta abre e fecha rapidinho... Você viu "Hello, world!" aparecer?
>
> Ela fecha rápido porque o programa termina na hora. Pra ver melhor, vamos rodar pelo terminal:
>
> **[NO TERMINAL, NAVEGAR ATÉ target/debug/]**
>
> ```bash
> cd target\debug
> .\hello_rust.exe
> ```
>
> **[EXECUTAR]**
>
> ```
> Hello, world!
> ```
>
> Olha aí! Rodando direto, sem Cargo! Esse é um programa **standalone** - autossuficiente! 🎉
>
> Você poderia copiar esse .exe pra um pen drive, levar pra outro computador, e rodar lá!

---

### [15:00 - 16:30] COMANDOS CARGO: BUILD vs RUN vs CHECK

**🎬 DICA DE GRAVAÇÃO:**
- Diagrama comparando os 3 comandos
- Executar cada um e mostrar a diferença

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de encerrar, deixa eu te mostrar 3 comandos essenciais do Cargo:
>
> **[VOLTAR PRA PASTA RAIZ DO PROJETO]**
>
> ### **1. cargo run** (o que já usamos)
>
> ```bash
> cargo run
> ```
>
> - ✅ Compila o código
> - ✅ Executa o programa
> - 📌 Mais usado no dia a dia!
>
> ### **2. cargo build**
>
> ```bash
> cargo build
> ```
>
> **[EXECUTAR]**
>
> ```
>     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
> ```
>
> Olha! Foi super rápido (0.01s) porque já tava compilado!
>
> - ✅ Compila o código
> - ❌ NÃO executa
> - 📌 Use quando só quer verificar se compila
>
> ### **3. cargo check**
>
> ```bash
> cargo check
> ```
>
> **[EXECUTAR]**
>
> ```
>     Checking hello_rust v0.1.0
>     Finished dev [unoptimized + debuginfo] target(s) in 0.05s
> ```
>
> - ✅ Verifica se compila
> - ❌ NÃO gera executável
> - ❌ NÃO executa
> - ⚡ É o MAIS RÁPIDO!
> - 📌 Use pra checar erros rapidinho enquanto programa
>
> **[MOSTRAR TABELA COMPARATIVA NA TELA]**
>
> | Comando | Compila? | Gera .exe? | Executa? | Velocidade |
> |---------|----------|------------|----------|------------|
> | `cargo check` | ✅ | ❌ | ❌ | ⚡⚡⚡ |
> | `cargo build` | ✅ | ✅ | ❌ | ⚡⚡ |
> | `cargo run` | ✅ | ✅ | ✅ | ⚡ |

---

### [16:30 - 18:00] ENCERRAMENTO E TEASER

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Tom de conquista e empolgação

**🎤 TEXTO PARA O APRESENTADOR:**

> E é isso, pessoal! Seu primeiro projeto Rust criado e rodando! 🦀🎉
>
> Vamos recapitular o que você aprendeu:
>
> ✅ Criar projetos com `cargo new`
> ✅ Entender a estrutura de pastas (Cargo.toml, src/main.rs)
> ✅ Rodar programas com `cargo run`
> ✅ Diferença entre build, run e check
> ✅ Encontrar o executável gerado
>
> Você pode não perceber, mas você já sabe mais que 90% das pessoas que **pensam** em aprender programação mas nunca começam de verdade!
>
> No próximo vídeo, a gente vai **dissecar** esse "Hello, world!" linha por linha. Você vai entender:
> - O que é `fn main()`
> - Por que `println!` tem um ponto de exclamação
> - Como modificar o programa
> - E muito mais!
>
> Esse próximo vídeo é onde a programação de verdade começa! Você vai **entender** o que tá acontecendo, não só copiar e colar!
>
> Se você tá empolgado, deixa aquele like maroto, se inscreve se ainda não é inscrito, e comenta: "Primeiro projeto criado! 🦀"
>
> Nos vemos no próximo vídeo! Até lá! 👋

**[FIM DO VÍDEO 3]**

---
---

## 🎥 VÍDEO 4: ANATOMIA DO HELLO WORLD
**Duração:** ~20 minutos
**Arquivo:** `dia01_video04_anatomia_codigo.mp4`

---

### [00:00 - 01:00] ABERTURA EMPOLGANTE

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Tom misterioso virando empolgante

**🎤 TEXTO PARA O APRESENTADOR:**

> Fala, galera! Prepara que esse vídeo vai mudar como você vê código pra sempre! 🤯
>
> Até agora, a gente instalou Rust, configurou editor, criou um projeto, rodou um programa... Mas você realmente **entendeu** o código?
>
> Aquelas 3 linhas do "Hello, world!" parecem simples, mas por trás delas tem MUITA coisa acontecendo!
>
> Neste vídeo, a gente vai fazer uma verdadeira **autópsia** do código - linha por linha, palavra por palavra, até ponto e vírgula!
>
> E eu vou te explicar usando analogias tão boas que até sua avó vai entender programação! 😄
>
> Ao final deste vídeo, você vai **realmente entender** o que tá escrevendo. Não vai mais ser "copiar e colar mágico" - vai ser **conhecimento real**!
>
> Bora lá! 🚀

---

### [01:00 - 02:30] REVISÃO RÁPIDA DO CÓDIGO

**🎬 DICA DE GRAVAÇÃO:**
- VSCode aberto com main.rs
- Código em tela cheia, bem legível
- Fonte grande (zoom)

**🎤 TEXTO PARA O APRESENTADOR:**

> Primeiro, vamos relembrar o código completo:
>
> **[MOSTRAR main.rs]**
>
> ```rust
> fn main() {
>     println!("Hello, world!");
> }
> ```
>
> São **apenas 3 linhas**! Mas olha quantas coisas diferentes tem aqui:
>
> **[DESTACAR CADA ELEMENTO CONFORME MENCIONA]**
>
> - `fn` - duas letrinhas misteriosas
> - `main` - uma palavra específica
> - `()` - parênteses vazios
> - `{}` - chaves
> - `println!` - palavra com ponto de exclamação
> - `"Hello, world!"` - texto entre aspas
> - `;` - ponto e vírgula
>
> Cada um desses elementos tem um **propósito específico**! E você vai aprender TODOS agora!
>
> Vamos começar pela primeira linha...

---

### [02:30 - 07:00] DISSECANDO LINHA 1: fn main() {

**🎬 DICA DE GRAVAÇÃO:**
- Zoom extremo na linha 1
- Animações destacando cada parte
- Analogias com imagens ilustrativas

**🎤 TEXTO PARA O APRESENTADOR:**

> ### **Parte 1: `fn`**
>
> **[DESTACAR APENAS "fn"]**
>
> `fn` é abreviação de **function** (função).
>
> **"Mas o que é uma função?"**
>
> Ótima pergunta! Deixa eu te explicar com uma analogia:
>
> **ANALOGIA DA RECEITA:**
>
> Imagina que você tem um caderno de receitas. Cada receita tem:
> - Um **nome** ("Bolo de Chocolate")
> - Uma **lista de ingredientes**
> - Um **passo a passo** do que fazer
>
> Uma **função** em programação é exatamente isso! É uma **receita para o computador seguir**!
>
> - Tem um **nome** (que você escolhe)
> - Pode ter **ingredientes** (chamados "parâmetros")
> - Tem um **passo a passo** (o código dentro da função)
>
> Sempre que você quer usar aquela receita, só precisa falar o nome: "Faça o Bolo de Chocolate!" E o computador executa todos os passos!
>
> Então `fn` é como você diz pro Rust: "Ó, agora eu vou te ensinar uma receita nova!"
>
> ---
>
> ### **Parte 2: `main`**
>
> **[DESTACAR "main"]**
>
> `main` é o **nome** da função.
>
> Mas esse não é um nome qualquer! É um **nome MÁGICO** em Rust!
>
> **ANALOGIA DA PORTA DA FRENTE:**
>
> Imagina uma casa gigante com centenas de quartos, corredores, portas... Como um visitante sabe por onde entrar?
>
> Pela **porta da frente**! É a entrada principal!
>
> A função `main` é a **porta da frente do seu programa**! 🚪
>
> Quando você roda um programa Rust, o sistema operacional procura por `fn main()` e começa a executar por ali. É **obrigatório**! Todo programa Rust executável precisa ter uma função `main`.
>
> Se você mudar o nome pra `fn inicio()` ou `fn comeco()`, vai dar erro! O Rust vai reclamar: "Cadê a main? Não sei por onde começar!"
>
> Então **sempre** vai ter `fn main()` nos seus programas! Marca isso! 📝
>
> ---
>
> ### **Parte 3: `()`**
>
> **[DESTACAR OS PARÊNTESES]**
>
> Esses parênteses são a **lista de ingredientes** da receita!
>
> Voltando à analogia: se você tem uma receita "Fazer Suco", os ingredientes poderiam ser: "fruta" e "quantidade de água".
>
> Em programação, esses ingredientes são chamados de **parâmetros**.
>
> **Por que os parênteses estão vazios aqui?**
>
> Porque a função `main` não precisa de ingredientes! Ela funciona sozinha!
>
> É tipo uma receita "Servir Água" - não precisa de ingredientes, só pega um copo e coloca água. Simples assim!
>
> Mais pra frente, você vai criar funções com parâmetros:
>
> ```rust
> fn somar(a, b) { ... }
> ```
>
> Mas no `main`, sempre vai ser `()` vazio!
>
> ---
>
> ### **Parte 4: `{`**
>
> **[DESTACAR A CHAVE DE ABERTURA]**
>
> Essa chavezinha `{` marca o **início do corpo da função** - onde fica o passo a passo!
>
> **ANALOGIA DO COMEÇO DA RECEITA:**
>
> É tipo quando você lê uma receita:
>
> ```
> Bolo de Chocolate:
> Ingredientes: ...
> Modo de Preparo:   ← Aqui é a "chave de abertura"
>     1. Bata os ovos
>     2. Adicione farinha
>     ...
> ```
>
> Tudo entre `{` e `}` é o que a função **faz**.
>
> **REGRA DE OURO:** Toda chave aberta `{` precisa de uma chave fechada `}` correspondente!
>
> Se você esquecer, o Rust vai reclamar: "Você abriu uma chave mas não fechou!"

---

### [07:00 - 12:00] DISSECANDO LINHA 2: println!("Hello, world!");

**🎬 DICA DE GRAVAÇÃO:**
- Linha 2 em destaque
- Quebrar em partes menores
- Usar cores/destaques diferentes

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora a linha mais importante:
>
> ```rust
> println!("Hello, world!");
> ```
>
> Vamos por partes!
>
> ---
>
> ### **Parte 1: Os 4 Espaços no Começo**
>
> **[DESTACAR A INDENTAÇÃO]**
>
> Viu esses espaços antes do código? Isso se chama **indentação**.
>
> Em Rust, indentação não é obrigatória pra funcionar, mas é **extremamente recomendada** pra **legibilidade**!
>
> **ANALOGIA DO LIVRO BEM FORMATADO:**
>
> Imagina um livro sem parágrafos, sem espaços, tudo grudado:
>
> ```
> Erauma vezumprincipequevivianumaflorestaeleencontrouumdragon...
> ```
>
> Dá pra ler? Dá... mas é horrível!
>
> Agora com espaços:
>
> ```
> Era uma vez um príncipe que vivia numa floresta.
> Ele encontrou um dragão...
> ```
>
> Muito melhor! Indentação é isso - deixar o código legível!
>
> **Padrão Rust:** 4 espaços por nível de indentação.
>
> Como esse código tá **dentro** da função main, ele tá indentado 4 espaços!
>
> ---
>
> ### **Parte 2: `println!`**
>
> **[DESTACAR "println!"]**
>
> Vamos quebrar esse nome estranho:
>
> - **`print`** = imprimir
> - **`ln`** = line (linha)
> - **`!`** = ... opa, o que é isso?
>
> `println` = "print line" = **imprimir uma linha**!
>
> Quando você chama `println`, ele:
> 1. Imprime o texto na tela
> 2. Pula pra próxima linha (por isso o "ln")
>
> Se fosse só `print`, imprimiria mas não pularia linha!
>
> ---
>
> ### **Parte 3: O Ponto de Exclamação `!`**
>
> **[DESTACAR O !]**
>
> Esse é **super importante**!
>
> Em Rust, quando você vê um nome seguido de `!`, não é uma função - é uma **MACRO**!
>
> **"Macro? Que diabos é isso?"**
>
> Deixa eu explicar:
>
> **ANALOGIA DA RECEITA ADAPTÁVEL:**
>
> - **Função** = receita fixa. "Bolo de Chocolate" sempre faz do mesmo jeito.
> - **Macro** = receita **adaptável**! "Bolo" pode virar Bolo de Chocolate, Bolo de Cenoura, Bolo de Fubá... dependendo do que você pede!
>
> Macros são mais **poderosas** e **flexíveis** que funções normais!
>
> `println!` é uma macro porque ela se adapta ao que você quer imprimir:
>
> ```rust
> println!("Texto simples");
> println!("Número: {}", 42);
> println!("Nome: {}, Idade: {}", nome, idade);
> ```
>
> Ela funciona com qualquer coisa! Por isso é uma macro!
>
> **DICA:** Sempre que ver `!` no final, pensa: "Isso é uma macro, é mais poderosa!"
>
> ---
>
> ### **Parte 4: `("Hello, world!")`**
>
> **[DESTACAR O CONTEÚDO ENTRE PARÊNTESES]**
>
> Os parênteses contêm os **argumentos** - o que você tá passando pra macro.
>
> No caso, estamos passando um texto: `"Hello, world!"`
>
> **O que são as aspas duplas `"`?**
>
> Aspas duplas marcam uma **string** - uma sequência de caracteres!
>
> **ANALOGIA DO COLAR DE CONTAS:**
>
> Imagina um colar com várias contas coloridas. Cada conta é uma letra!
>
> `"Hello, world!"` é como um colar com as letras: H-e-l-l-o-,-w-o-r-l-d-!
>
> Em programação, texto sempre vai entre aspas duplas `"..."` para o Rust saber: "Isso é texto, não é código!"
>
> Se você escrever:
>
> ```rust
> println!(Hello);
> ```
>
> (sem aspas)
>
> O Rust vai procurar por uma variável chamada `Hello`! Vai dar erro porque ela não existe!
>
> As aspas dizem: "Isso aqui é texto puro, não procura no código!"
>
> **Você pode colocar qualquer texto entre as aspas:**
>
> ```rust
> println!("Olá, Brasil! 🇧🇷");
> println!("Rust é incrível! 🦀");
> println!("123 abc @#$ ツ");
> ```
>
> Tudo funciona! Emojis, números como texto, símbolos...
>
> ---
>
> ### **Parte 5: `;` (Ponto e Vírgula)**
>
> **[DESTACAR O ;]**
>
> Esse pontinho é **super importante** e muita gente iniciante esquece dele!
>
> O ponto e vírgula `;` marca o **fim de uma instrução**.
>
> **ANALOGIA DA FRASE:**
>
> Quando você escreve um texto, como você indica que uma frase terminou?
>
> Com um **ponto final**.
>
> ```
> Eu gosto de programar. Rust é legal. Vamos aprender!
> ```
>
> Cada frase termina com um ponto!
>
> Em Rust, o ponto e vírgula `;` é o "ponto final" das instruções!
>
> ```rust
> println!("Primeira instrução");
> println!("Segunda instrução");
> println!("Terceira instrução");
> ```
>
> **Se você esquecer o `;` vai dar erro!**
>
> ```rust
> println!("Ops")  ← ERRO! Cadê o ponto e vírgula?
> ```
>
> O compilador vai reclamar:
>
> ```
> error: expected `;`
> ```
>
> "Esperado ponto e vírgula!"
>
> **DICA:** Sempre coloca `;` no final de instruções! Vira hábito rápido!

---

### [12:00 - 14:00] DISSECANDO LINHA 3: }

**🎬 DICA DE GRAVAÇÃO:**
- Destaque na chave de fechamento
- Mostrar visualmente o par {  }

**🎤 TEXTO PARA O APRESENTADOR:**

> ### **A Chave de Fechamento `}`**
>
> **[DESTACAR A }]**
>
> Essa chavezinha fecha o corpo da função!
>
> Lembra que lá na linha 1 a gente abriu com `{`? Agora estamos fechando!
>
> **ANALOGIA DOS PARÊNTESES:**
>
> Lembra nas aulas de matemática quando você aprendia:
>
> ```
> (2 + 3) × 5
> ```
>
> Você abre parênteses `(` e **sempre** tem que fechar `)` !
>
> Em programação é igual! Toda chave aberta precisa de uma fechada!
>
> **REGRA VISUAL:**
>
> Repara que a chave de fechamento `}` tá **alinhada** com a `fn` lá de cima!
>
> **[MOSTRAR LINHA VISUAL CONECTANDO { e }]**
>
> Isso ajuda visualmente a ver onde a função começa e termina!
>
> Se você usar indentação correta (e formatação automática do VSCode), essas coisas ficam automáticas!
>
> **O que acontece se esquecer a }?**
>
> Erro de compilação:
>
> ```
> error: unexpected end of file, expected `}`
> ```
>
> "Fim inesperado do arquivo, esperava uma chave de fechamento!"

---

### [14:00 - 17:00] MODIFICANDO O PROGRAMA (PRÁTICA AO VIVO)

**🎬 DICA DE GRAVAÇÃO:**
- Live coding
- Modificar o código em tempo real
- Rodar após cada modificação

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora que você entendeu cada pedacinho, vamos **modificar** o programa!
>
> Essa é a melhor forma de aprender - mexendo no código!
>
> ### **Modificação 1: Mudar o Texto**
>
> Vou mudar `"Hello, world!"` pra `"Olá, Rust! 🦀"`
>
> **[MODIFICAR O CÓDIGO]**
>
> ```rust
> fn main() {
>     println!("Olá, Rust! 🦀");
> }
> ```
>
> Salvar (Ctrl + S) e rodar:
>
> ```bash
> cargo run
> ```
>
> **[EXECUTAR]**
>
> ```
> Olá, Rust! 🦀
> ```
>
> Funcionou! 🎉 Você acabou de **customizar** seu primeiro programa!
>
> ---
>
> ### **Modificação 2: Adicionar Mais Linhas**
>
> E se eu quiser imprimir várias coisas? Simples! Adiciono mais `println!`
>
> **[MODIFICAR]**
>
> ```rust
> fn main() {
>     println!("Olá, Rust! 🦀");
>     println!("Este é meu primeiro programa!");
>     println!("Estou aprendendo muito! 🚀");
> }
> ```
>
> **[EXECUTAR cargo run]**
>
> ```
> Olá, Rust! 🦀
> Este é meu primeiro programa!
> Estou aprendendo muito! 🚀
> ```
>
> Cada `println!` imprime uma linha! Simples assim!
>
> ---
>
> ### **Modificação 3: Provocar um Erro (Didático)**
>
> Agora deixa eu te mostrar o que acontece se você esquecer o `;`
>
> **[REMOVER O ; DA PRIMEIRA LINHA]**
>
> ```rust
> fn main() {
>     println!("Olá, Rust! 🦀")   ← Sem ;
>     println!("Ops!");
> }
> ```
>
> **[TENTAR COMPILAR]**
>
> ```bash
> cargo run
> ```
>
> **[MOSTRAR O ERRO]**
>
> ```
> error: expected `;`, found `println`
>  --> src/main.rs:2:34
>   |
> 2 |     println!("Olá, Rust! 🦀")
>   |                                  ^ help: add `;` here
> 3 |     println!("Ops!");
>   |     ------- unexpected token
> ```
>
> Olha que erro **didático**! O Rust não só diz qual o problema ("expected `;`"), mas também **onde** (linha 2, coluna 34) e até **como resolver** ("add `;` here")!
>
> Esse é um dos motivos que Rust é excelente pra aprender - os erros são super explicativos!
>
> Vou corrigir:
>
> **[ADICIONAR O ;]**
>
> ```rust
> println!("Olá, Rust! 🦀");
> ```
>
> **[RODAR DE NOVO]**
>
> Agora funciona! ✅
>
> **LIÇÃO:** Erros não são seus inimigos! São professores te ensinando! 📚

---

### [17:00 - 19:00] CURIOSIDADES E DETALHES AVANÇADOS

**🎬 DICA DE GRAVAÇÃO:**
- Slide com curiosidades
- Tom de "bônus extra"

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de encerrar, deixa eu te contar algumas curiosidades sobre "Hello, World"!
>
> ### **Curiosidade 1: A Tradição**
>
> "Hello, World!" é a **tradição universal** da programação!
>
> Desde os anos 1970, quando alguém aprende uma nova linguagem, o primeiro programa sempre é esse!
>
> É tipo um ritual de iniciação! 😄 Agora você faz parte da tradição!
>
> ---
>
> ### **Curiosidade 2: Por Que `println!` Tem "ln"?**
>
> Porque existe também `print!` (sem ln)!
>
> **Diferença:**
>
> ```rust
> print!("Olá");   // Imprime mas NÃO pula linha
> print!(" Mundo"); // Continua na mesma linha
> // Saída: Olá Mundo
> ```
>
> Vs.
>
> ```rust
> println!("Olá");   // Imprime e PULA linha
> println!("Mundo");  // Nova linha
> // Saída:
> // Olá
> // Mundo
> ```
>
> Geralmente você vai usar `println!` (com ln)!
>
> ---
>
> ### **Curiosidade 3: Formatação Avançada**
>
> `println!` pode fazer coisas **muito** mais poderosas:
>
> ```rust
> let nome = "João";
> let idade = 25;
> println!("Meu nome é {} e tenho {} anos!", nome, idade);
> // Saída: Meu nome é João e tenho 25 anos!
> ```
>
> Os `{}` são como "espaços em branco" que são preenchidos com as variáveis!
>
> Mas isso é assunto pro próximo dia! 😉

---

### [19:00 - 20:00] ENCERRAMENTO ÉPICO

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Tom de conquista

**🎤 TEXTO PARA O APRESENTADOR:**

> E aí está, pessoal! Você acabou de fazer uma **autópsia completa** do Hello World! 🔬
>
> Agora você não só sabe **escrever** o código - você **entende** ele!
>
> Recapitulando:
> - ✅ `fn` define uma função (receita)
> - ✅ `main` é a porta de entrada do programa
> - ✅ `()` são os parâmetros (vazio no main)
> - ✅ `{}` delimitam o corpo da função
> - ✅ `println!` é uma macro que imprime
> - ✅ `"texto"` é uma string
> - ✅ `;` termina instruções
>
> Você saiu de "copiar e colar misterioso" pra **entendimento real**! 🧠
>
> No próximo vídeo, o ÚLTIMO do Dia 1, você vai criar um **programa customizado** - um cartão de visitas digital! Vai ser épico!
>
> Se você chegou até aqui, deixa aquele super like, compartilha com quem também quer aprender, e comenta: "Código dissecado! 🔬"
>
> Nos vemos no último vídeo do Dia 1! Até já! 👋

**[FIM DO VÍDEO 4]**

---
---

## 🎥 VÍDEO 5: PROJETO PRÁTICO - CARTÃO DE VISITAS
**Duração:** ~25 minutos
**Arquivo:** `dia01_video05_projeto_pratico.mp4`

---

### [00:00 - 01:30] ABERTURA EMPOLGADA

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Muito entusiasmo!
- Gesticulação expressiva

**🎤 TEXTO PARA O APRESENTADOR:**

> FALA, GALERA! Chegou a hora da **VERDADE**! 🎉
>
> Nos últimos 4 vídeos, você aprendeu as bases:
> - Instalou Rust
> - Configurou o editor
> - Criou seu primeiro projeto
> - Entendeu cada linha do código
>
> E AGORA... você vai criar um programa **DE VERDADE**! Não vai ser só copiar "Hello World" - vai ser algo **SEU**, **CUSTOMIZADO**, **ÚNICO**!
>
> Neste vídeo, você vai criar um **Cartão de Visitas Digital** - um programa que exibe suas informações de forma bonita e profissional no terminal!
>
> E o melhor: **você** quem vai escrever o código! Eu vou guiar, mas o código vai ser seu!
>
> **Ao final deste vídeo, você vai ter:**
> - Um programa completamente customizado
> - Criado com suas próprias mãos
> - Que você pode mostrar pros amigos e falar: "EU FIZ ISSO!" 💪
>
> Preparado pra codar? BORA! 🚀

---

### [01:30 - 03:30] APRESENTAÇÃO DO PROJETO

**🎬 DICA DE GRAVAÇÃO:**
- Mostrar o resultado final primeiro
- Rodar o programa pronto
- Criar expectativa

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de começar a construir, deixa eu te mostrar o que vamos criar!
>
> **[RODAR UM EXEMPLO PRONTO]**
>
> ```bash
> cargo run
> ```
>
> **[MOSTRAR A SAÍDA]**
>
> ```
> ╔═════════════════════════════════════════╗
> ║                                         ║
> ║         🦀 CARTÃO DE VISITAS 🦀         ║
> ║                                         ║
> ║  Nome: João Silva                       ║
> ║  Profissão: Desenvolvedor Rust Júnior   ║
> ║                                         ║
> ║  📧 Email: joao@exemplo.com             ║
> ║  🐙 GitHub: github.com/joaosilva        ║
> ║                                         ║
> ║  "Aprendendo Rust, um dia por vez! 🚀" ║
> ║                                         ║
> ╚═════════════════════════════════════════╝
> ```
>
> **[PAUSA PARA IMPACTO]**
>
> Olha que **profissional**! Parece até interface gráfica, mas é só terminal! 🤩
>
> Imagina você chegando num evento de tecnologia, abrindo o notebook, rodando esse programa e mostrando pros recrutadores: "Esse é meu cartão de visitas digital que **EU** programei!"
>
> **Contexto do Projeto:**
>
> Você é um desenvolvedor iniciante e quer uma forma criativa de se apresentar. Em vez de um cartão físico chato, você vai ter um **programa** que exibe suas informações!
>
> Vamos criar isso juntos, passo a passo!

---

### [03:30 - 05:30] CRIANDO O PROJETO

**🎬 DICA DE GRAVAÇÃO:**
- Screencast do terminal
- Passo a passo bem pausado

**🎤 TEXTO PARA O APRESENTADOR:**

> Primeira coisa: criar o projeto!
>
> Abre o terminal (ou terminal integrado do VSCode) e navega até sua pasta de projetos:
>
> ```bash
> cd C:\Users\SeuNome\Documents\projetos_rust
> ```
>
> (ou `~/Documentos/projetos_rust` no Linux/Mac)
>
> **[EXECUTAR]**
>
> Agora vamos criar o projeto:
>
> ```bash
> cargo new cartao_visitas
> ```
>
> **[EXECUTAR]**
>
> ```
>      Created binary (application) `cartao_visitas` package
> ```
>
> Perfeito! Entrar na pasta:
>
> ```bash
> cd cartao_visitas
> ```
>
> E abrir no VSCode:
>
> ```bash
> code .
> ```
>
> **[VSCODE ABRE]**
>
> Show! Projeto criado! Agora vamos ao código!

---

### [05:30 - 08:00] ESTRATÉGIA DE DESENVOLVIMENTO

**🎬 DICA DE GRAVAÇÃO:**
- Slide ou whiteboard explicando a estratégia
- Tom de "arquiteto planejando"

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de sair escrevendo código feito louco, vamos **planejar**!
>
> Programadores profissionais sempre planejam antes de codificar!
>
> **O que nosso programa precisa fazer?**
>
> Imprimir várias linhas formando um cartão:
>
> 1. Borda superior
> 2. Linhas vazias (pra espaçamento)
> 3. Título centralizado
> 4. Informações pessoais
> 5. Contatos
> 6. Mensagem
> 7. Borda inferior
>
> **Como vamos fazer?**
>
> Com **vários** `println!` seguidos! Cada um imprime uma linha!
>
> **Estratégia Pedagógica:**
>
> Eu vou escrever as primeiras linhas explicando cada detalhe, e depois **você** vai continuar sozinho!
>
> Mas não se preocupa - eu vou dar dicas progressivas se você travar!
>
> Bora começar!

---

### [08:00 - 12:00] CODIFICAÇÃO GUIADA (PRIMEIRAS LINHAS)

**🎬 DICA DE GRAVAÇÃO:**
- Live coding em ritmo pedagógico
- Comentar CADA linha enquanto digita
- Não acelerar!

**🎤 TEXTO PARA O APRESENTADOR:**

> Abre o arquivo `src/main.rs`. Vamos substituir o Hello World!
>
> **[ABRIR main.rs, APAGAR O CONTEÚDO]**
>
> Começando do zero! Primeira coisa: a função main.
>
> ```rust
> fn main() {
> 
> }
> ```
>
> **[DIGITAR DEVAGAR]**
>
> Pronto! Estrutura básica! Agora vem o conteúdo...
>
> ### **Linha 1: Borda Superior**
>
> ```rust
> fn main() {
>     println!("╔═════════════════════════════════════════╗");
> }
> ```
>
> **[DIGITAR E EXPLICAR]**
>
> - `println!` - já conhecemos! Imprime uma linha
> - As aspas duplas começam a string
> - Esses caracteres especiais (`╔`, `═`, `╗`) criam a moldura
> - As aspas duplas fecham a string
> - `;` termina a instrução
>
> Vamos testar? Salvar (Ctrl + S) e rodar:
>
> ```bash
> cargo run
> ```
>
> **[EXECUTAR]**
>
> ```
> ╔═════════════════════════════════════════╗
> ```
>
> Funcionou! A borda superior apareceu! 🎉
>
> ---
>
> ### **Linha 2: Linha Vazia**
>
> Agora vamos adicionar uma linha vazia (com as bordas laterais):
>
> ```rust
> fn main() {
>     println!("╔═════════════════════════════════════════╗");
>     println!("║                                         ║");
> }
> ```
>
> **[ADICIONAR A LINHA]**
>
> **[RODAR NOVAMENTE]**
>
> ```
> ╔═════════════════════════════════════════╗
> ║                                         ║
> ```
>
> Perfeito! Tá tomando forma!
>
> ---
>
> ### **Linha 3: Título Centralizado**
>
> ```rust
> println!("║         🦀 CARTÃO DE VISITAS 🦀         ║");
> ```
>
> **[ADICIONAR]**
>
> **Detalhes importantes:**
> - Os espaços antes e depois centralizam o texto
> - Emojis funcionam perfeitamente! 🦀
> - Tudo entre `"` e `"` é impresso exatamente como tá
>
> **[RODAR]**
>
> ```
> ╔═════════════════════════════════════════╗
> ║                                         ║
> ║         🦀 CARTÃO DE VISITAS 🦀         ║
> ```
>
> SHOW! Tá ficando lindo! 🤩
>
> ---
>
> ### **Mais Uma Linha Vazia**
>
> ```rust
> println!("║                                         ║");
> ```
>
> **[ADICIONAR]**
>
> Agora vem a parte que **você** vai personalizar!

---

### [12:00 - 16:00] DESAFIO AO ESPECTADOR (COM DICAS PROGRESSIVAS)

**🎬 DICA DE GRAVAÇÃO:**
- Pausar a codificação
- Falar diretamente com o espectador
- Tom de incentivo

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora é com VOCÊ! ⏸️
>
> **PAUSA O VÍDEO AQUI** e tenta completar o código sozinho!
>
> Você precisa adicionar:
> - Nome (exemplo: "║  Nome: Seu Nome Aqui                    ║")
> - Profissão (exemplo: "║  Profissão: Estudante de Rust           ║")
> - Linha vazia
> - Email com emoji 📧
> - GitHub com emoji 🐙
> - Linha vazia
> - Mensagem motivacional entre aspas
> - Linha vazia
> - Borda inferior (╚═════...═╝)
>
> **DICA 1:** Copia a estrutura das linhas que já fizemos!
>
> **DICA 2:** Use `println!` para cada linha!
>
> **DICA 3:** Não esquece do `;` no final!
>
> **DICA 4:** Os emojis podem ser copiados de sites tipo emojipedia.org!
>
> Tenta fazer sozinho! Vou dar 3... 2... 1... **PAUSA!** ⏸️
>
> **[PAUSA DE 5 SEGUNDOS NO VÍDEO - TELA ESTÁTICA]**
>
> ...
>
> Conseguiu? Ótimo! Se não conseguiu, não tem problema! Vou mostrar a solução agora!
>
> **[CONTINUAR]**

---

### [16:00 - 18:30] SOLUÇÃO COMPLETA COMENTADA

**🎬 DICA DE GRAVAÇÃO:**
- Mostrar o código completo
- Rodar e mostrar resultado
- Comemorar!

**🎤 TEXTO PARA O APRESENTADOR:**

> Aqui está o código completo:
>
> ```rust
> fn main() {
>     // Borda superior
>     println!("╔═════════════════════════════════════════╗");
>     
>     // Linha vazia
>     println!("║                                         ║");
>     
>     // Título
>     println!("║         🦀 CARTÃO DE VISITAS 🦀         ║");
>     
>     // Linha vazia
>     println!("║                                         ║");
>     
>     // Informações pessoais - CUSTOMIZE AQUI!
>     println!("║  Nome: João Silva                       ║");
>     println!("║  Profissão: Desenvolvedor Rust Júnior   ║");
>     
>     // Linha vazia
>     println!("║                                         ║");
>     
>     // Contatos - CUSTOMIZE AQUI!
>     println!("║  📧 Email: joao@exemplo.com             ║");
>     println!("║  🐙 GitHub: github.com/joaosilva        ║");
>     
>     // Linha vazia
>     println!("║                                         ║");
>     
>     // Mensagem motivacional - CUSTOMIZE AQUI!
>     println!("║  \"Aprendendo Rust, um dia por vez! 🚀\" ║");
>     
>     // Linha vazia
>     println!("║                                         ║");
>     
>     // Borda inferior
>     println!("╚═════════════════════════════════════════╝");
> }
> ```
>
> **[MOSTRAR O CÓDIGO COMPLETO]**
>
> **Detalhes importantes:**
>
> 1. **Comentários** (`//`) explicam cada seção - super importante pra organização!
> 2. **Linhas vazias no código** (entre seções) deixam mais legível
> 3. As aspas `\"` dentro da string são **escapadas** (coloca `\` antes)
>
> Agora o momento da verdade! Vamos rodar:
>
> ```bash
> cargo run
> ```
>
> **[EXECUTAR]**
>
> **[MOSTRAR RESULTADO COMPLETO]**
>
> ```
> ╔═════════════════════════════════════════╗
> ║                                         ║
> ║         🦀 CARTÃO DE VISITAS 🦀         ║
> ║                                         ║
> ║  Nome: João Silva                       ║
> ║  Profissão: Desenvolvedor Rust Júnior   ║
> ║                                         ║
> ║  📧 Email: joao@exemplo.com             ║
> ║  🐙 GitHub: github.com/joaosilva        ║
> ║                                         ║
> ║  "Aprendendo Rust, um dia por vez! 🚀" ║
> ║                                         ║
> ╚═════════════════════════════════════════╝
> ```
>
> **SUCESSO ABSOLUTO!!!** 🎉🎊🥳
>
> **Você acabou de criar um programa COMPLETO, CUSTOMIZADO, PROFISSIONAL!**
>
> Agora substitui as informações pelas suas e você tem um cartão de visitas digital único! 💎

---

### [18:30 - 21:00] VARIAÇÕES E DESAFIOS BÔNUS

**🎬 DICA DE GRAVAÇÃO:**
- Mostrar variações rapidamente
- Tom de "se quiser ir além"

**🎤 TEXTO PARA O APRESENTADOR:**

> Agora que você dominou o básico, aqui vão **desafios bônus** pra quem quer ir além!
>
> ### **Variação 1: Adicionar Mais Informações**
>
> Você pode adicionar:
> - LinkedIn
> - Twitter
> - Site pessoal
> - Telefone
> - Localização
>
> É só adicionar mais linhas de `println!`!
>
> ---
>
> ### **Variação 2: ASCII Art**
>
> Adiciona um desenho ASCII! Exemplo:
>
> ```rust
> println!("║           ___                           ║");
> println!("║          (o o)                          ║");
> println!("║       ooO--(_)--Ooo                     ║");
> ```
>
> Procura "ASCII art generator" no Google pra criar desenhos!
>
> ---
>
> ### **Variação 3: Cores no Terminal (Avançado)**
>
> Você pode adicionar cores usando códigos ANSI!
>
> **[MOSTRAR BREVEMENTE]**
>
> ```rust
> const VERDE: &str = "\x1b[32m";
> const RESET: &str = "\x1b[0m";
>
> println!("{VERDE}╔═════════════╗{RESET}");
> ```
>
> Isso é mais avançado, mas funciona! Pesquisa "ANSI color codes Rust" pra aprender mais!
>
> ---
>
> ### **Variação 4: Diferentes Estilos de Borda**
>
> Experimente bordas diferentes:
>
> **Estilo 1: ASCII Simples**
> ```
> +----------------------------------------+
> |                                        |
> +----------------------------------------+
> ```
>
> **Estilo 2: Duplo**
> ```
> ╔════════════════════════════════════════╗
> ║                                        ║
> ╚════════════════════════════════════════╝
> ```
>
> **Estilo 3: Arredondado**
> ```
> ╭────────────────────────────────────────╮
> │                                        │
> ╰────────────────────────────────────────╯
> ```
>
> Escolhe o que você mais gosta!

---

### [21:00 - 23:00] LIÇÕES APRENDIDAS E REFLEXÃO

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Tom reflexivo e professor

**🎤 TEXTO PARA O APRESENTADOR:**

> Antes de encerrar, deixa eu te fazer refletir sobre o que você **realmente** aprendeu neste projeto:
>
> ### **1. Você Aprendeu a Estruturar Código**
>
> O programa não é uma bagunça - ele tem **seções lógicas**:
> - Borda
> - Conteúdo
> - Fechamento
>
> Isso é **arquitetura de software** no nível básico!
>
> ---
>
> ### **2. Você Usou Comentários**
>
> Os `//` não são só enfeite - eles **documentam** seu código!
>
> Código sem comentário é como mapa sem legendas - funciona, mas ninguém entende!
>
> ---
>
> ### **3. Você Debugou Erros**
>
> Aposto que você cometeu pelo menos um erro (esqueceu `;`, aspas, etc).
>
> E você **corrigiu**! Isso é **debugging** - parte essencial da programação!
>
> Programadores profissionais passam 50% do tempo corrigindo erros. É normal!
>
> ---
>
> ### **4. Você Customizou e Experimentou**
>
> Você não copiou cegamente - você **adaptou** pro seu caso!
>
> Isso é **pensamento criativo** aplicado à programação!
>
> ---
>
> ### **5. Você Criou Algo ÚTIL**
>
> Esse programa tem **uso real**! Você pode:
> - Mostrar em entrevistas
> - Colocar no GitHub
> - Usar em eventos
>
> Isso não é "exercício de mentirinha" - é um **projeto real**! 💼

---

### [23:00 - 25:00] ENCERRAMENTO ÉPICO E PRÓXIMOS PASSOS

**🎬 DICA DE GRAVAÇÃO:**
- Câmera frontal
- Energia máxima
- Comemoração genuína

**🎤 TEXTO PARA O APRESENTADOR:**

> E é isso, galera! Você COMPLETOU o **DIA 1**! 🎉🎊🥳
>
> Deixa eu recapitular TUDO que você conquistou hoje:
>
> ✅ Instalou Rust completo no seu computador
> ✅ Configurou VSCode como um pro
> ✅ Criou seu primeiro projeto
> ✅ Entendeu cada linha do código
> ✅ Criou um programa customizado e profissional
>
> Você saiu do **ZERO ABSOLUTO** pra ter um **PROGRAMA FUNCIONANDO**!
>
> **Estatística pra te motivar:**
>
> Sabe quantas pessoas **pensam** em aprender programação? Milhões.
>
> Sabe quantas **começam**? Algumas centenas de milhares.
>
> Sabe quantas **completam o primeiro dia**? Apenas algumas dezenas de milhares!
>
> **VOCÊ ESTÁ NESSA ELITE!** 💪
>
> Você não é mais um "interessado" - você é um **programador Rust iniciante**!
>
> ---
>
> ### **E Agora? O Que Vem Depois?**
>
> O Dia 2 vai ser INCRÍVEL! Você vai aprender:
>
> - **Variáveis** - como guardar informações
> - **Tipos de dados** - números, textos, booleanos
> - **Mutabilidade** - o conceito que faz Rust ser único!
>
> Seu cartão de visitas vai evoluir! Em vez de ser "fixo", ele vai poder **mudar** durante a execução!
>
> ---
>
> ### **Tarefa de Casa (Opcional, Mas Recomendado!):**
>
> 1. **Customize seu cartão** com suas informações reais
> 2. **Experimente** variações de estilo
> 3. **Mostre pra alguém** - amigo, familiar, colega
> 4. **Tire uma foto/print** do programa rodando e posta nas redes sociais com #AprendiRust
>
> E marca a gente! Eu vou **repostar** os melhores! 📸
>
> ---
>
> ### **Mensagem Final**
>
> Programação é uma jornada, não um sprint. Você não precisa entender tudo de uma vez.
>
> O importante é **avançar um pouquinho todo dia**.
>
> E hoje, você avançou MUITO! 🚀
>
> Eu tô super orgulhoso de você! E você deveria estar orgulhoso de si mesmo também!
>
> ---
>
> Se você gostou desse curso até aqui:
> - 👍 Deixa aquele SUPER LIKE!
> - 🔔 Se inscreve e ativa o sininho pra não perder o Dia 2!
> - 💬 Comenta: "Dia 1 completo! Rumo ao Dia 2! 🦀"
> - 📤 Compartilha com quem também quer aprender!
>
> Qualquer dúvida, comenta que eu respondo TODAS!
>
> **TE VEJO NO DIA 2!**
>
> Até lá, continue praticando, continue curioso, e continue programando!
>
> Um abraço, e nos vemos em breve! 👋🦀

**[TELA FINAL COM CALL-TO-ACTION]**
- Like 👍
- Subscribe 🔔
- Comment 💬
- Share 📤

**[FIM DO VÍDEO 5 E DO DIA 1]**

---
---

## 📊 RESUMO EXECUTIVO DO MATERIAL

### Estatísticas do Curso em Vídeo - Dia 1:

- **Total de Vídeos:** 5
- **Duração Total:** ~98 minutos (~1h38min)
- **Palavras no Roteiro:** ~12.000 palavras
- **Nível de Dificuldade:** Iniciante Absoluto
- **Taxa de Conclusão Esperada:** Alta (conteúdo gamificado e motivador)

### Estrutura Pedagógica:

1. **Vídeo 1 (20min):** Fundação técnica
2. **Vídeo 2 (15min):** Configuração profissional
3. **Vídeo 3 (18min):** Primeira experiência prática
4. **Vídeo 4 (20min):** Compreensão profunda
5. **Vídeo 5 (25min):** Aplicação criativa

### Diferenciais do Roteiro:

✅ **Analogias constantes** (oficina de ferreiro, receitas, porta da frente)
✅ **Tom descontraído** mas profissional
✅ **Pausas estratégicas** para o aluno tentar sozinho
✅ **Comemoração de conquistas** (gamificação)
✅ **Erros didáticos** (mostrar erro propositalmente pra ensinar)
✅ **Timestamps precisos** para facilitar edição
✅ **Dicas de gravação** em cada seção

---

🎉 **ROTEIRO COMPLETO! Pronto para gravação!** 🎬🦀