local dap = require('dap')

dap.configurations.rust = {
    {
        name = "Run Rust program",
        type = "cppdbg",
        request = "launch",
        program = function()
            return '${workspaceFolder}/target/debug/01'
        end,
        cwd = function ()
            -- if string.find(vim.fn.expand('%'), 'nhf') then
            --     return '${workspaceFolder}/nhf'
            -- end
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

dap.adapters.cppdbg = {
    id = 'cppdbg',
    type = 'executable',
    command = os.getenv'OPEN_DEBUG_PATH',
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

