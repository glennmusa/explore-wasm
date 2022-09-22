# explore-wasm

A repository to explore WebAssembly modules.

## Development Tools

This repository specifies a Visual Studio Code [devcontainer.json](.devcontainer/devcontainer.json) and [Docker image](.devcontainer/Dockerfile) that gives you all the prerequisites and preview versions of software you need to begin building and running WebAssembly binaries in dotnet, Rust, and Go with the [wasmtime runtime](https://github.com/bytecodealliance/wasmtime).

It's very easy to [open this repository in a GitHub CodeSpace](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace#creating-a-codespace), otherwise [follow these instructions to open this repository in a DevContainer locally](https://code.visualstudio.com/docs/remote/containers).

## What's in this repository

|                                            | Contents                                                                               |
| -------------------------------------------| :------------------------------------------------------------------------------------- |
| [dotnet](./src/dotnet)                     | a WebAssembly module using preview features to demonstrate `.wasm` binaries from .NET  |
| [rust](./src/rust)                         | a WebAssembly module that copies files to demonstrate sandboxing and filesystem access |
| [webassembly-text](./src/webassembly-text) | a WebAssembly text format module to demonstrate converting from from `.wat` to `.wasm` |

## Context

This repository is largely inspired by this video from Matt Butcher from Fermyon that highlights how WebAssembly modules are an attractive path to developing and deploying fast, cross-platform, and secure microservices:

[![WebAssembly and Containers on YouTube](img/wasm-modules-small.png)](https://www.youtube.com/watch?v=OGcm3rHg630)
