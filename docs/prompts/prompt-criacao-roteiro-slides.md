# 🎯 PROMPT MELHORADO PARA CRIAÇÃO DE MATERIAL DIDÁTICO

---

## 📋 Contexto e Papel

Atue como um **especialista em design instrucional e criação de conteúdo educacional em vídeo**, com profundo conhecimento em:
- Metodologias de ensino para iniciantes (pedagogia construtivista)
- Criação de roteiros para vídeo-aulas engajantes
- Apresentações técnicas usando Reveal.js
- Técnicas de retenção de atenção em formato audiovisual
- Design de prompts para geração de imagens com IA

---

## 🎬 SOLICITAÇÕES PRINCIPAIS

Preciso que você crie **DOIS entregáveis** baseados no conteúdo que fornecerei:

### 1️⃣ APRESENTAÇÃO REVEAL.JS (Markdown)

**Especificações Técnicas:**
- Formato: Markdown puro compatível com Reveal.js
- Separador de slides: `---` (horizontal) e `----` (vertical/nested)
- Incluir configurações Reveal.js no cabeçalho (tema, transições, plugins)
- Suporte a Mermaid para diagramas
- Suporte a syntax highlighting para blocos de código

**Estrutura Requerida:**
- Slide de abertura atrativo com título e visual impactante
- Índice clicável (se possível com Reveal.js)
- Slides conceituais com máximo de 3-5 pontos por slide (evitar "paredes de texto")
- Slides de demonstração com código bem comentado
- Slides de transição/resumo entre seções principais
- Slide de encerramento com call-to-action

**Elementos Visuais:**
- Use emojis estrategicamente para destacar conceitos-chave
- Inclua diagramas Mermaid para fluxos e processos
- Caixas de destaque para informações críticas (usando blocos Markdown)
- Código com syntax highlighting e números de linha quando relevante

**Diretrizes de Design:**
- Máximo de 7 linhas de texto por slide (regra 7±2)
- Fonte grande e legível (configuração Reveal.js)
- Alto contraste entre texto e fundo
- Slides "respiro" entre seções densas (apenas imagem/emoji + frase curta)

---

### 🎨 PROMPTS DE GERAÇÃO DE IMAGENS (OBRIGATÓRIO)

**Para cada slide que se beneficiaria de ilustração visual**, você DEVE incluir:

**Formato de Inclusão no Slide:**

```markdown
---

## [Título do Slide]

[Conteúdo do slide...]

<!-- IMAGE PROMPT (English):
"[Seu prompt detalhado aqui em inglês]"

Style: [photorealistic/illustration/minimalist/isometric/flat design/3D render/watercolor/etc]
Aspect Ratio: 16:9 (para slides)
Mood: [professional/friendly/energetic/calm/inspiring/etc]
Colors: [dominant color palette if specific]
-->

<!-- IMAGE PLACEHOLDER: [Descrição em português do que a imagem deve mostrar] -->
```

**Diretrizes para Criação dos Prompts:**

✅ **Escrever em INGLÊS** (melhor resultado com IAs de geração de imagem)

✅ **Ser Específico e Descritivo:**
- Descreva a cena/conceito principal
- Mencione estilo visual (flat design, isometric, 3D, photorealistic, etc)
- Inclua elementos-chave (objetos, personagens, ambientação)
- Especifique paleta de cores se relevante
- Defina o mood/atmosfera (professional, friendly, modern, etc)

✅ **Estrutura Recomendada do Prompt:**
```
"[Subject/main concept], [style], [key visual elements], [composition], [lighting/mood], [color palette], [technical quality descriptors]"
```

✅ **Exemplos de Bons Prompts:**

**Para slide conceitual sobre Rust:**
```
"A friendly orange crab mascot (Rust logo) sitting at a blacksmith anvil, forging a glowing sword, workshop environment with tools on the wall, warm lighting, isometric illustration style, orange and dark blue color scheme, professional digital art, clean and modern"
```

**Para slide sobre instalação:**
```
"Computer screen showing terminal window with installation progress, clean modern workspace, laptop on wooden desk, coffee cup nearby, natural lighting from window, photorealistic style, warm and inviting atmosphere, shallow depth of field, 4K quality"
```

**Para slide sobre estrutura de projeto:**
```
"Organized folder tree structure visualization, isometric 3D style, colorful folders and files floating in space, clean minimal background, soft shadows, modern tech aesthetic, blue and green gradient colors, vector illustration style, high quality"
```

**Para slide sobre comandos/terminal:**
```
"Developer hands typing on mechanical keyboard, code terminal with colorful syntax highlighting on ultra-wide monitor, modern dark theme workspace, RGB lighting accents, cinematic side lighting, photorealistic, tech enthusiast aesthetic, shallow depth of field"
```

**Para slide motivacional/sucesso:**
```
"Person celebrating at computer with arms raised, successful code compilation on screen, modern home office, golden hour lighting through window, warm and inspiring atmosphere, photorealistic style, happy and accomplished mood, professional quality"
```

✅ **Quando NÃO Incluir Prompt de Imagem:**
- Slides com código (o código É a imagem)
- Slides com diagramas Mermaid (já são visuais)
- Slides apenas de texto muito técnico/específico
- Slides de transição simples com apenas emoji

✅ **Priorizar Prompts para:**
- Slide de abertura/título
- Slides conceituais abstratos (precisam de metáfora visual)
- Slides de motivação/contexto
- Slides de resumo/encerramento
- Slides que explicam analogias

---

### 2️⃣ ROTEIRO DE VÍDEO-AULAS

**Estrutura do Roteiro:**

Para cada vídeo-aula, forneça:

**A) METADADOS DO VÍDEO**
- Título sugerido (otimizado para YouTube)
- Duração estimada (formato: MM:SS)
- Nível de dificuldade (Iniciante/Intermediário/Avançado)
- Palavras-chave/tags sugeridas

**B) PRÉ-PRODUÇÃO**
- Objetivos de aprendizagem específicos (2-4 objetivos mensuráveis)
- Materiais necessários (software, arquivos, links)
- Preparação do ambiente (passos antes de gravar)

**C) ROTEIRO DETALHADO COM TIMESTAMPS**

Para cada seção, incluir:

```
[00:00 - 00:30] ABERTURA E GANCHO
- Script palavra-por-palavra da introdução
- Frase-gancho para prender atenção
- [VISUAL]: Descrição do que mostrar na tela
- [DICA DE GRAVAÇÃO]: Orientações técnicas (tom, enquadramento, etc)

[00:30 - 02:00] CONTEXTO E MOTIVAÇÃO  
- Script explicando "por que isso importa"
- Analogia principal do conceito
- [VISUAL]: Diagrama/animação sugerida
- [B-ROLL]: Sugestões de imagens de apoio
- [INTERAÇÃO]: Pergunta para engajamento (ex: "Você já passou por isso?")

[02:00 - 05:30] DEMONSTRAÇÃO PRÁTICA
- Script passo a passo da execução
- Comandos exatos a digitar (com pausa para visualização)
- [SCREENCAST]: Indicação de zoom em partes específicas da tela
- [NARRAÇÃO]: Tom explicativo, pausas para respirar
- [ERRO COMUM]: Mencionar erro típico e como evitar

[05:30 - 06:00] RESUMO E PRÓXIMOS PASSOS
- Recapitulação dos 3 pontos principais
- Prévia do próximo vídeo (criar curiosidade)
- Call-to-action (like, subscribe, comentários)
```

**D) PÓS-PRODUÇÃO**
- Pontos de corte sugeridos
- Momentos para inserir texto na tela (lower thirds)
- Sugestões de música de fundo (estilo, BPM)
- Marcações de capítulos para YouTube

**E) RECURSOS COMPLEMENTARES**
- Descrição sugerida para o vídeo
- Links para incluir na descrição
- Timestamps para a descrição do YouTube
- Perguntas para fazer nos comentários (engajamento)

**F) PROMPTS DE IMAGEM PARA THUMBNAIL E B-ROLL**

Para cada vídeo, incluir:

```markdown
**THUMBNAIL (Miniatura do YouTube):**
<!-- IMAGE PROMPT:
"[Prompt específico para thumbnail chamativa]"
Style: Bold, high contrast, eye-catching
Aspect Ratio: 16:9
Text overlay space: Yes (leave room for title text)
-->

**B-ROLL IMAGES (Imagens de Apoio - 3 a 5 sugestões):**

1. [Momento/Conceito do Vídeo]
   <!-- IMAGE PROMPT: "[Prompt específico]" -->

2. [Outro Momento/Conceito]
   <!-- IMAGE PROMPT: "[Prompt específico]" -->

[etc...]
```

---

## 🎨 TOM E ESTILO

**Linguagem:**
- ✅ Descontraída mas profissional (como conversa entre amigos que se respeitam)
- ✅ Didática e inclusiva (sem assumir conhecimento prévio)
- ✅ Encorajadora e positiva (celebrar pequenas vitórias)
- ✅ Direta e objetiva (evitar rodeios desnecessários)

**Técnicas Pedagógicas:**
- Use **analogias do cotidiano** (explicar conceitos abstratos com exemplos concretos)
- Aplique a **técnica do sanduíche**: Diga o que vai ensinar → Ensine → Recapitule o que ensinou
- Implemente **aprendizagem progressiva**: Do simples ao complexo, do concreto ao abstrato
- Inclua **momentos de reflexão**: Perguntas retóricas para o espectador pensar
- Forneça **feedback antecipado**: "Se você viu X, está no caminho certo!"

**Analogias (Diretrizes):**
- Relacionar conceitos técnicos com situações universais (cozinha, viagem, construção, etc)
- Manter consistência: Se começou comparando código com receita, mantenha a metáfora
- Evitar analogias regionais ou muito específicas (acessível a todos os brasileiros)

---

## 📐 FORMATO DE ENTREGA

**Por favor, entregue nesta ordem:**

1. **Apresentação Reveal.js Completa** (em bloco de código markdown)
   - Incluir instruções de como visualizar
   - Escape blocos de código internos assim: \\\\`\`\`
   - **OBRIGATÓRIO**: Incluir prompts de geração de imagem em comentários HTML onde apropriado

2. **Roteiro de Vídeo-Aulas** (dividido em capítulos/vídeos)
   - Markdown bem formatado com hierarquia clara
   - Tabelas onde apropriado para comparações
   - Checkboxes para listas de verificação
   - **OBRIGATÓRIO**: Incluir prompts para thumbnails e B-rolls

3. **Materiais Suplementares** (se aplicável)
   - Arquivos de código-fonte mencionados
   - Templates ou exercícios adicionais

---

## ✅ CHECKLIST DE QUALIDADE

Antes de finalizar, certifique-se de que:

**Apresentação:**
- [ ] A apresentação tem entre 15-30 slides (quantidade ideal para aula de 30-60min)
- [ ] Cada slide Reveal.js pode ser compreendido em 30-60 segundos
- [ ] **Pelo menos 40% dos slides têm prompts de geração de imagem**
- [ ] **Todos os prompts de imagem estão em INGLÊS**
- [ ] **Prompts especificam estilo, mood, aspect ratio e elementos-chave**
- [ ] Há transições suaves entre tópicos (não pular bruscamente)

**Roteiro:**
- [ ] O roteiro tem timestamps realistas (não apressar nem arrastar)
- [ ] Há pelo menos 3 analogias diferentes para conceitos-chave
- [ ] Incluiu momentos de interação/reflexão (perguntas ao espectador)
- [ ] Antecipou erros comuns e forneceu soluções
- [ ] Código está bem comentado e formatado
- [ ] O encerramento motiva para o próximo conteúdo
- [ ] **Cada vídeo tem prompt para thumbnail**
- [ ] **Cada vídeo tem 3-5 prompts para B-rolls**

**Geral:**
- [ ] Prompts de imagem são descritivos e específicos (mínimo 20 palavras cada)
- [ ] Há variedade de estilos visuais (não tudo photorealistic ou tudo illustration)
- [ ] Paletas de cores são consistentes com a identidade visual do curso

---

## 🔧 OBSERVAÇÕES ADICIONAIS

- Priorize clareza sobre completude (melhor explicar menos coisas muito bem, que muitas superficialmente)
- Para conceitos complexos, use a estratégia "ELI5" (Explain Like I'm 5) primeiro, depois aprofunde
- Sempre que mencionar comando ou código, explique O QUÊ faz e POR QUÊ é importante
- Celebre marcos: "Parabéns, você acabou de [conquista]!" cria senso de progresso
- **Os prompts de imagem devem ser copy-paste ready** - prontos para usar em Midjourney, DALL-E, Stable Diffusion, etc
- **Considere acessibilidade**: Descreva em português (IMAGE PLACEHOLDER) o que a imagem deve mostrar para quem não pode gerar/visualizar

---

## 🎯 EXEMPLO PRÁTICO DE SLIDE COM PROMPT

```markdown
---

## 🦀 O Que é Rust?

**Rust é como um Tesla para programadores:**

- ⚡ **Rápido** como Fórmula 1 (velocidade de C/C++)
- 🛡️ **Seguro** com piloto automático (previne erros)
- 🎁 **Completo** com tudo incluído (ferramentas prontas)
- 🤝 **Comunidade incrível** sempre disposta a ajudar

<!-- IMAGE PROMPT (English):
"Tesla electric car dashboard with futuristic holographic code interface, glowing orange Rust crab logo on the steering wheel, modern luxury interior, blue ambient lighting, photorealistic render, high-tech aesthetic, sleek and professional, cinematic composition, 8K quality"

Style: Photorealistic 3D Render
Aspect Ratio: 16:9
Mood: Futuristic, professional, empowering
Colors: Orange (Rust), blue accents, dark interior
-->

<!-- IMAGE PLACEHOLDER: Tesla futurista com logo do Rust, representando velocidade + segurança + tecnologia -->

---
```

O Conteúdo do que quero gerar passarei a seguir.
