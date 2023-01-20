# Incumbency Desktop

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode).

## Dev Setup

```bash
#  Make sure you have Tauri CLI installed
cargo install tauri-cli

# Install dependencies
npm i

# Run app for development
cargo tauri dev

# Build for production
cargo tauri build
```

## Debugger Setup
### VSCode
In the root directory:
- Create a `.vscode` folder

Inside the folder:
- Create a `launch.json`
- Create a `tasks.json`

`launch.json`:
```json
{
  // Use IntelliSense to learn about possible attributes.
  // Hover to view descriptions of existing attributes.
  // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Tauri Development Debug",
      "cargo": {
        "args": [
          "build",
          "--manifest-path=./src-tauri/Cargo.toml",
          "--no-default-features"
        ]
      },
      // task for the `beforeDevCommand` if used, must be configured in `.vscode/tasks.json`
      "preLaunchTask": "ui:dev",
      "cwd": "${workspaceFolder}/src-tauri"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Tauri Production Debug",
      "cargo": {
        "args": ["build", "--release", "--manifest-path=./src-tauri/Cargo.toml"]
      },
    //   // task for the `beforeBuildCommand` if used, must be configured in `.vscode/tasks.json`
    //   "preLaunchTask": "ui:build"
    }
  ]
}
```
`tasks.json`:
```json
{
  // See https://go.microsoft.com/fwlink/?LinkId=733558
  // for the documentation about the tasks.json format
  "version": "2.0.0",
  "tasks": [
    {
      "label": "ui:dev",
      "type": "shell",
      // `dev` keeps running in the background
      // ideally you should also configure a `problemMatcher`
      // see https://code.visualstudio.com/docs/editor/tasks#_can-a-background-task-be-used-as-a-prelaunchtask-in-launchjson
      "problemMatcher": "$rustc",
      "isBackground": true,
      // change this to your `beforeDevCommand`:
      "command": "npm",
      "args": ["run", "dev"]
    },
    {
      "label": "ui:build",
      "type": "shell",
      // change this to your `beforeBuildCommand`:
      "command": "npm run",
      "args": ["build"]
    }
  ]
}
```