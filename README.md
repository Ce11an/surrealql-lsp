# SurrealQL LSP

A Language Server Protocol (LSP) implementation for SurrealDB's query language, SurrealQL.

__Work in Progress!__

## About
SurrealQL LSP is built with Rust and leverages several powerful libraries:
- [tower-lsp](https://github.com/ebkalderon/tower-lsp)
- [lsp-textdocument](https://github.com/GiveMe-A-Name/lsp-textdocument)
- [tree-sitter-surrealql](https://github.com/Ce11an/tree-sitter-surrealql)

## Installation
Development is ongoing, so installation guides are currently limited to Neovim or
Visual Studio Code. Either way, clone and change directories to the root of the repository.

### Visual Studio Code
Open Visual Studio Code in the root of the repository.

Ensure that you have installed the required node modules:

```posh
cd editors/code && npm install && cd ../..
```

Once installed, you can launch the extension by pressing `F5`. This will open a new
instance of Visual Studio Code. In the new instance, navigate to the `examples/test.surql`
file. Start typing SurrealQL, you should see completions!

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

### Hover
Hover over SurrealQL keyworks for documentation - ongoing!

![Screenshot 2024-07-13 at 20 46 24](https://github.com/user-attachments/assets/fa99a451-0c48-4243-8c02-e455322da938)

## Contributions
We welcome contributions! If you find this project interesting and want to help, please consider contributing.
