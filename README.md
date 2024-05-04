# Rust Simple Text Calculator ![tests](https://github.com/nsclass/rust-calculator/workflows/Rust/badge.svg)

Simple calculator application by using post fix algorithm in Rust

### Backend
Axum framework used for HTTP server

#### API

- Calculation

```http request
POST http://localhost:3000/calculation
Content-Type: application/json

{
  "infix": "1 + 2 * (3 + 4) / 2"
}
```
### Frontend

Frontend application is developed with vite, react and tailwind css

