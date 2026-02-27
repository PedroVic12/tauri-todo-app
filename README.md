# 📝 Todo App - Tauri + Rust + Express

Gerenciador de Tarefas com Categorias usando:
- **Tauri 2.0** - Framework desktop multiplataforma
- **Rust + Rocket** - Backend com POO
- **Express.js** - API REST para testes
- **SQLite3** - Banco de dados local
- **React Vanilla** - Frontend sem bibliotecas

## 🚀 Setup Rápido

### Pré-requisitos
- Rust (https://rustup.rs/)
- Node.js 18+ (https://nodejs.org/)
- Tauri CLI: `cargo install tauri-cli`

### Instalação

```bash
# Clonar repo
git clone https://github.com/PedroVic12/tauri-todo-app.git
cd tauri-todo-app

# Instalar dependências
npm install
cd express-api && npm install && cd ..
cd src-tauri && cargo build && cd ..
```

### Rodar em Desenvolvimento

```bash
# Terminal 1 - Tauri App
npm run tauri

# Terminal 2 (opcional) - Express API
npm run api:dev
```

## 📁 Estrutura do Projeto

```
tauri-todo-app/
├── src/                    # Frontend (React Vanilla)
│   ├── index.html
│   ├── app.js
│   └── styles.css
├── src-tauri/              # Backend Rust + Tauri
│   ├── src/
│   │   ├── main.rs        # Entry point Tauri
│   │   ├── models.rs      # Structs (Task, Category)
│   │   ├── database.rs    # Database class
│   │   ├── handlers.rs    # Tauri commands
│   │   └── lib.rs
│   └── Cargo.toml
├── express-api/            # API Express (testes)
│   ├── api.js
│   └── package.json
└── README.md
```

## 📚 API Endpoints (Express)

```
POST   /api/tasks              - Criar tarefa
GET    /api/tasks              - Listar todas
GET    /api/tasks/:id          - Uma tarefa
GET    /api/tasks/category/:id - Por categoria
PUT    /api/tasks/:id          - Atualizar
DELETE /api/tasks/:id         - Deletar

POST   /api/categories         - Criar categoria
GET    /api/categories         - Listar
DELETE /api/categories/:id     - Deletar

GET    /api/stats              - Estatísticas
```

## 💡 Exemplos de Teste

```bash
# Criar categoria
curl -X POST http://localhost:3001/api/categories \
  -H "Content-Type: application/json" \
  -d '{"name":"Trabalho","color":"#667eea"}'

# Criar tarefa
curl -X POST http://localhost:3001/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"title":"Aprender Rust","description":"Estudar POO","category_id":"uuid-aqui"}'

# Ver tudo
curl http://localhost:3001/api/tasks
```

## 🎯 Features

✅ CRUD de Tarefas e Categorias
✅ Código Rust com POO (Structs + Métodos)
✅ Banco SQLite3 local
✅ IPC Tauri (commands)
✅ Frontend Vanilla JS (sem React lib)
✅ API Express para testes
✅ Responsivo e moderno

## 👨‍💻 Desenvolvedor

PedroVic12

---

**Bora desenvolver!** 🚀