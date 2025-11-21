# 3D Snake on Cube

A modern, 3D reimagining of the classic Snake game, played on the surface of a voxel cube. Built with **Rust**, **WebAssembly**, and **Three-d**.

![Snake 3D Icon](favicon.png)

## Features

-   **Voxel Graphics**: A beautiful, semi-transparent blue voxel board with a glowing 3D grid.
-   **3D Gameplay**: The snake moves across all 6 faces of a 3D cube.
-   **Smooth Camera**: The camera automatically rotates and follows the snake as it traverses the cube faces.
-   **Performance**: Powered by Rust and WebAssembly for high performance and smooth rendering.

## Controls

-   **W / Up Arrow**: Move Up
-   **S / Down Arrow**: Move Down
-   **A / Left Arrow**: Move Left
-   **D / Right Arrow**: Move Right
-   **R**: Restart Game (when Game Over)

## Development

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install)
-   [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Build & Run

1.  **Build the project**:
    ```bash
    ./build.sh
    ```
    This script compiles the Rust code to WebAssembly.

2.  **Run a local server**:
    ```bash
    python3 -m http.server
    ```

3.  **Play**:
    Open your browser and navigate to `http://localhost:8000`.

## Deployment

For instructions on how to deploy this game to **Azure Static Web Apps**, please see [DEPLOY.md](DEPLOY.md).

## License

This project is open source.
