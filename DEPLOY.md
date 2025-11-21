# Deploying to Azure Static Web Apps

This guide explains how to deploy the 3D Snake game to Azure Static Web Apps.

## Prerequisites
- An [Azure Account](https://azure.microsoft.com/free/).
- A GitHub repository for this project.

## Method 1: Using Azure Portal

1.  **Push your code to GitHub**: Ensure your latest changes are pushed to your repository.
2.  **Create a Static Web App**:
    - Go to the [Azure Portal](https://portal.azure.com).
    - Search for "Static Web Apps" and click "Create".
    - **Subscription**: Select your subscription.
    - **Resource Group**: Create a new one (e.g., `snake3d-rg`).
    - **Name**: Give your app a name (e.g., `snake3d-rs`).
    - **Plan Type**: Free (for hobby/personal projects).
    - **Deployment details**: Select "GitHub".
    - **Authorize**: Click "Sign in with GitHub" and authorize Azure.
    - **Organization/Repository/Branch**: Select your repo and the `main` branch.
3.  **Build Details**:
    - **Build Presets**: Select "Custom".
    - **App location**: `/`
    - **Api location**: (Leave empty)
    - **Output location**: `.` (This serves the root directory)
4.  **Review + Create**: Click "Review + create" and then "Create".

Azure will automatically create a GitHub Actions workflow in your repository and start building/deploying your app.

## Method 2: Using VS Code

1.  Install the **Azure Static Web Apps** extension for VS Code.
2.  Click the Azure icon in the sidebar.
3.  Right-click "Static Web Apps" and select "Create Static Web App... (Advanced)".
4.  Follow the prompts:
    - Select your subscription.
    - Enter a name.
    - Select a region.
    - Select "Custom" for the build preset.
    - Enter `/` for the location of your application code.
    - Enter `.` for the build output location.
5.  The extension will create the resources and the GitHub Action.

## Important: Build Configuration

Since this project uses a custom `build.sh` script (wrapping `wasm-pack`), you need to ensure the GitHub Action runs it.

Azure's default workflow might try to detect a build system. You may need to update the generated `.github/workflows/azure-static-web-apps-....yml` file.

**Add the build step to the workflow:**

Locate the `Build And Deploy` job in the generated YAML file and ensure it has the necessary tools (`wasm-pack`).

Example modification to the workflow file:

```yaml
      - name: Install wasm-pack
        run: curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

      - name: Build WASM
        run: ./build.sh
```

**Note**: Since `build.sh` runs `wasm-pack`, you just need to make sure `wasm-pack` is installed in the runner.

## Verifying Deployment

Once the GitHub Action completes, Azure will provide a URL (e.g., `https://gentle-river-123.azurestaticapps.net`). Click it to play your game!
