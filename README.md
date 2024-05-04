# Rust Simple Text Calculator ![tests](https://github.com/nsclass/rust-calculator/workflows/Rust/badge.svg)

Simple calculator by using post fix algorithm in Rust. It supports operators: +, -, * and /

Main purpose of this project is to show how to use Axum Rust web framework for backend and React with TypeScript for frontend UI.


### Backend
Axum framework used for HTTP server

#### API

- Calculation

```http request
POST http://localhost:3000/calculate
Content-Type: application/json

{
  "infix": "1 + 2 * (3 + 4) / 2"
}
```
### Frontend

Frontend application is developed with Vite for bundler, React for JS framework and Tailwind for CSS

