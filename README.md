# Incumbency Desktop

![Incumbency Screenshot](/screenshots/incumbency.png)

## About

Incumbency is an economy simulation engine made by the YouTuber [conaticus](https://youtube.com/@conaticus) and his team. In this game you can change taxes, rules and budgets to keep the economy stable.

### Download

You can either build with instructions from the README or download the executable [here](https://github.com/conaticusgrp/incumbency-desktop/releases).

## Honourable mentions

### Lemon Foxmere

The slick UX Design for this app was created completely by [Lemon Foxmere](https://github.com/TheLemonOrange).

### Exedice

The fundamentals of the desktop UI was created in svelte by [Exedice](https://github.com/Ex-ce-pt).

### Z88

[Z88](https://github.com/maksymixs) helped out with the wireframe for the desktop UI and came up with some game design ideas for incumbency.

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

-   Create a `.vscode` folder

Inside the folder:

-   Create a `launch.json`
-   Create a `tasks.json`

`launch.json`:

```json
{
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
            "preLaunchTask": "ui:dev",
            "cwd": "${workspaceFolder}/src-tauri"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Tauri Production Debug",
            "cargo": {
                "args": [
                    "build",
                    "--release",
                    "--manifest-path=./src-tauri/Cargo.toml"
                ]
            }
        }
    ]
}
```

`tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "ui:dev",
            "type": "shell",
            "problemMatcher": "$rustc",
            "isBackground": true,
            "command": "npm",
            "args": ["run", "dev"]
        },
        {
            "label": "ui:build",
            "type": "shell",
            "command": "npm run",
            "args": ["build"]
        }
    ]
}
```
