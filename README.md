# SurrealQL LSP

A Language Server Protocol (LSP) implementation for SurrealDB's query language, SurrealQL.

__Work in Progress!__

## About
SurrealQL LSP is built with Rust and leverages several powerful libraries:
- [tower-lsp](https://github.com/ebkalderon/tower-lsp)
- [lsp-textdocument](https://github.com/GiveMe-A-Name/lsp-textdocument)
- [tree-sitter-surrealql](https://github.com/Ce11an/tree-sitter-surrealql)

## Installation
Development is ongoing, so installation guides are currently limited to Neovim. To get started:

1. Clone this repository.
2. Follow the instructions below to set up the LSP in Neovim.

### Neovim Setup
Add the following Lua script to your Neovim configuration:

```lua
local M = {}

local find_rust_bin = function()
  return '<path-to-repo>/surrealql-lsp/target/debug/surrealql-lsp-server'
end

M.start = function()
  vim.lsp.set_log_level 'debug'
  require('vim.lsp.log').set_format_func(vim.inspect)

  local client = vim.lsp.start {
    name = 'surrealql-lsp-server',
    cmd = { find_rust_bin() },
  }

  if not client then
    vim.notify('Failed to start surrealql-lsp-server', vim.log.levels.ERROR)
    return
  end

  vim.lsp.buf_attach_client(0, client)
end

local group = vim.api.nvim_create_namespace 'surrealql-lsp-server'

M.setup = function()
  vim.api.nvim_clear_autocmds { group = group }

  vim.api.nvim_create_autocmd('FileType', {
    group = group,
    pattern = 'surql',
    callback = M.start,
  })
end

return M
```

Inside the root of the repository, run:

```sh
cargo build
```

Navigate to your `test.surql` file and run `:LspInfo` in Neovim to ensure the LSP is attached.

## Features

### Incremental Parsing
Utilises [tree-sitter](https://github.com/tree-sitter/tree-sitter) for efficient incremental parsing.

### Code Completion
Provides code completion support to streamline your development workflow - ongoing!

![Screenshot 2024-07-02 at 22 32 16](https://github.com/Ce11an/surrealql-lsp/assets/60790416/6e39965f-4e8c-41ff-bc16-125d43b4db65)

## Contributions
We welcome contributions! If you find this project interesting and want to help, please consider contributing.
