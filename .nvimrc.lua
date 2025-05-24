local dap = require("dap")

dap.adapters.codelldb = {
  type = "server",
  port = "${port}",
  executable = {
    command = vim.fn.stdpath("data") .. "/mason/packages/codelldb/extension/adapter/codelldb",
    args = { "--port", "${port}" },
  },
}

dap.configurations.rust = {
  {
    name = "Debug Executable",
    type = "codelldb",
    request = "launch",
    program = function()
      return vim.fn.input("Path to executable: ", vim.fn.getcwd() .. "/target/debug/", "file")
    end,
    cwd = "${workspaceFolder}",
    stopOnEntry = false,
    args = {},
    runInTerminal = false,
  },
  {
    name = "Debug Test",
    type = "codelldb",
    request = "launch",
    program = function()
      -- TODO: You can also automate this by searching deps/*test* can be achieved with `cargo test --no-run`
      return vim.fn.getcwd() .. "/target/debug/deps/interpreter-75efa5b489cc1842"
    end,
    args = { "--exact", "parser::parser_tests::parse_call_expression_with_literals" }, -- optional: filter to a single test function
    cwd = "${workspaceFolder}",
    stopOnEntry = false,
    runInTerminal = false,
    -- IMPORTANT: This is required to ensure that values are displayed correctly in the debugger
    sourceLanguages = { "rust" },
  },
}
