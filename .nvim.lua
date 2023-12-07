local dap = require('dap')

local vscode_lldb_path = os.getenv'VSCODE_LLDB_PATH'
if vscode_lldb_path == nil then
    print('VSCODE_LLDB_PATH not set')
    return
end

dap.configurations.rust = {
    {
        name = "Run Rust program",
        type = "codelldb",
        sourceLanguages = { "rust" },
        request = "launch",
        program = function()
            return '${workspaceFolder}/target/debug/05'
        end,
        cwd = function ()
            return '${workspaceFolder}'
        end,
        stopAtEntry = false,
        setupCommands = {
            {
                text = '-enable-pretty-printing',
                description =  'enable pretty printing',
                ignoreFailures = false
            },
        },
    },
}

dap.adapters.codelldb = {
  type = 'server',
  port = "${port}",
  executable = {
    command = vscode_lldb_path .. '/share/vscode/extensions/vadimcn.vscode-lldb/adapter/codelldb',
    args = {"--port", "${port}"},
  }
}
