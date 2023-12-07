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
            return '${workspaceFolder}/target/debug/03'
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

    -- On windows you may have to uncomment this:
    -- detached = false,
  }
}

-- compile and run, pass in input if it exists
vim.keymap.set('n', '<Leader>r', function ()
    local input = vim.fn.expand('%:r') .. '.in'
    local exists = vim.fn.filereadable(input)
    if exists == 1 then
        vim.cmd('!gcc ' .. ' "%" && ./a.out < "' .. input .. '"')
    else
        vim.cmd('!gcc ' .. ' "%" && ./a.out')
    end
end)

-- compile and run in an interactive shell
vim.keymap.set('n', '<Leader>t', function ()
    local cmd
    if string.find(vim.fn.expand('%'), 'nhf') then
        cmd = '!gcc ' .. ' ' .. vim.fn.expand('%:p:h') .. '/*.c'
    else
        cmd = '!gcc ' .. ' "' .. vim.fn.expand('%') .. '"'
    end
    vim.cmd(cmd)

    local dir = vim.fn.expand('%:p:h')

    vim.cmd(':te (cd ' .. dir .. ' && ../a.out)')
end)

