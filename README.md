# Comics List

Comics List is a work-in-progress desktop application for organizing a personal comic book collection. It is designed to manage publishers, series, and individual issues while keeping the collection in a local SQLite database.

The project also serves as a practical learning environment for building a desktop application with Rust and a modern web frontend.

## Purpose

The main goals of this repository are to:

- build a useful local-first comic collection manager;
- learn Rust through a concrete application rather than isolated exercises;
- understand communication between a Vue frontend and a Rust backend through Tauri commands;
- gain experience with SQLite persistence, data modeling, testing, and desktop application architecture;
- document an intentional and transparent approach to AI-assisted software development.

## Current scope

The application currently focuses on:

- publishers;
- comic series and their publication years;
- individual issues and publication metadata;
- local SQLite persistence through the Rust backend;
- a mock API for developing and testing the frontend in a browser.

The project is still under active development, so features, data structures, and APIs may change.

## Technology stack

### Frontend

- Vue 3
- TypeScript
- Vite
- Vue Router
- Tailwind CSS
- DaisyUI
- Vitest and Testing Library

### Desktop and backend

- Tauri
- Rust
- SQLite through `rusqlite`

## AI-assisted development disclosure

This repository has been developed through an intentional combination of human-written code and AI-assisted work.

The primary educational objective is to learn Rust and the surrounding concepts required to build a Tauri desktop application. AI has therefore been used as a learning and engineering support tool, particularly to:

- explain Rust, Tauri, TypeScript, Vue, and SQLite concepts;
- discuss possible implementation approaches and their trade-offs;
- help diagnose compiler, runtime, dependency, and test errors;
- review code and identify inconsistencies or missing edge cases;
- suggest focused refactors and improvements;
- assist with test design and technical documentation.

Development has not been delegated to AI as an autonomous process. Work has alternated between manual implementation and AI-assisted sessions. Requirements, product direction, architectural choices, acceptance of proposed changes, and final responsibility for the code remain human decisions.

AI-generated suggestions are treated in the same way as suggestions from any other external source: they must be understood, reviewed, adapted to the project, and validated before being retained. The purpose of using AI here is not to bypass learning, but to support it by providing explanations, feedback, and debugging assistance while working on a real codebase.

As a result, the repository should be understood as an AI-assisted learning project, not as either entirely AI-generated software or a demonstration of development performed without external assistance. This disclosure is included so that contributors and future readers can evaluate the project with an accurate understanding of how it was created.

## Project structure

```text
src/                    Vue and TypeScript frontend
src/components/         Reusable Vue components
src/pages/              Application pages
src/api/                Frontend API adapter
src/mocks/              Browser development mocks
src/tests/              Frontend tests
src-tauri/              Rust, Tauri, and SQLite backend
```

## Development

Install the JavaScript dependencies:

```sh
npm install
```

Run the frontend in the browser:

```sh
npm run dev
```

Run the desktop application in development mode:

```sh
npm run tauri:dev
```

Running the desktop application requires Rust and the platform-specific Tauri prerequisites.

## Testing

Run the frontend test suite in watch mode:

```sh
npm run test
```

Run the test suite once:

```sh
npm run test -- --run
```

Build the frontend for production:

```sh
npm run build
```

## Status

Comics List is an educational work in progress. It is not currently intended to be treated as production-ready software.
