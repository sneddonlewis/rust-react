# My Web App
[![CI](https://github.com/sneddonlewis/rust-react/actions/workflows/ci.yml/badge.svg)](https://github.com/sneddonlewis/rust-react/actions/workflows/ci.yml)


Generate TypeScript type definitions.  
From `./server`
```
tsync -i ./src -o ../client/src/types.d.ts
```
## Dev

### Requirements
- Rust
- Node
- Yarn

To run the backend and the frontend concurrently, run the command from the `./client` directory:
```
yarn dev:fs
```
On termination both processes should be stopped. Otherwise it's kill -9 no more CPU time 😉
```
kill -9 $(lsof -t -i :4000)
```
```
kill -9 $(lsof -t -i :5173)
```

## TODOs

- Move gitignores to repo level

## Roadmap

This is a play about/learning repo so some things to add:
- CRUD with diesel and SQLite then PostgreSQL
- Frontend does CRUD stuff
- Auth
- TLS
- Production Ready
- Little bit biz logic
- Tests
- CI

