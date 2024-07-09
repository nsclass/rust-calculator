# Rust Simple Text Calculator ![tests](https://github.com/nsclass/rust-calculator/workflows/Rust/badge.svg)

Simple calculator by using post fix algorithm in Rust. It supports operators: +, -, * and /

Main purpose of this project is to show how to use Axum Rust web framework for backend and React with TypeScript for frontend UI.


### Backend
Axum framework used for HTTP server

#### REST API

- Calculation

```http request
POST http://localhost:3000/calculate
Content-Type: application/json

{
  "infix": "1 + 2 * (3 + 4) / 2"
}
```

### Frontend

Frontend application is developed with TypeScript, ReactJS and Tailwind with Vite

- Supporting internationalization(English, Korean)

