# Interlinked: a proof-of-concept link shortener with PoW verification

## Project Structure

The project is organized as a monorepo with the following components:

- `server/`: The backend server that handles API requests and serves the frontend.
- `frontend/`: The WASM-based frontend application.
- `cli/`: A command-line interface for accessing the backend.
- `shared/`: A shared Rust crate for code used by other components (e.g., data models).

## Installation

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/ipsavitsky/interlinked.git
    cd interlinked
    ```

2.  **Build the project:**
    This command compiles all the Rust crates.
    ```sh
    just build
    ```

3.  **Generate WASM bindings:**
    This creates the WebAssembly package for the frontend.
    ```sh
    just generate_bindings
    ```

4.  **(Optional) Set up the database:**
    This runs the necessary database migrations.
    ```sh
    just create_db
    ```

## Usage

To run the application, you'll need to start both the backend and frontend services in separate terminal sessions.

-   **Run the backend server:**
    ```sh
    just serve_backend
    ```

-   **Run the frontend server:**
    ```sh
    just serve_frontend
    ```

Once both are running, you can access the application in your browser, typically at `http://localhost:3001`, or use the cli to access store some links!
