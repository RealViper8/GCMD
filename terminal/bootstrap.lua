local user = {}

local terminal = require("modules.terminal")

local debug = true
terminal.ansi_code = true

if not debug and terminal.clear_on_start then
    terminal.clear()
end

local data = io.open("data.txt","r")

if data == nil then
    print()
    print("\x1b[0;32m--- GCMD-Bootstrapper ---")

    local username
    ::username::
    while true do
        io.write("\x1b[0;36mUsername: ")
        local input = io.read()

        if string.match(input, " ") then
            print("\x1b[0;31mUsername shouldnt contain spaces\x1b[0m")
            goto username
        elseif input:match("^%w%w%w%w.*$")then
            username = input
            break
        else
            print("\x1b[0;31mUsername should be 4 characters long.\x1b[0m")
        end
    end

    local password
    ::password::
    while true do
        io.write("\x1b[0;32mPassword: ")
        local input = io.read()

        if input == username or string.find(string.lower(input), string.lower(username)) then print("\x1b[0;31mPassword shouldnt be the same as username\x1b[0m"); goto password
        elseif string.sub(input, -1) == ' ' then
            input = string.sub(input, 1, -2)
        end

        if string.match(input, " ") then
            print("\x1b[0;31mPassword shouldnt contain spaces\x1b[0m")
            goto password
        elseif input:match("^%w%w%w%w%w%w%w%w.*$")then
            password = input
            break
        else
            print("\x1b[0;31mPassword should be 8 characters long and should contain up-lowercase, and one digit.\x1b[0m")
        end
    end

    table.insert(user, encode(username))
    table.insert(user, hash(password))

    io.open("data.txt", "w"):write(table.concat(user, ",")):close()

    print("\x1b[0;32m--- GCMD-Bootstrapper done ---\x1b[0m")
else
    local user_text = io.open("data.txt", "r")
    local users = {}

    local function split(inputstr, sep)
        if sep == nil then
            sep = "%s"
        end
        local t = {}
        for str in string.gmatch(inputstr, "([^" .. sep .. "]+)") do
            table.insert(t, str)
        end
        return t
    end

    if user_text then
        for line in user_text:lines() do
            local userdata = split(line, ",")
            table.insert(users, userdata[1])
            table.insert(users, userdata[2])
        end
    end

    local username
    repeat
        io.write("\x1b[0;32mUsername: ")
        username = io.read()
        if encode(username) ~= users[1] then print("\x1b[0;31mUsername doesnt match.\x1b[0m") end
    until encode(username) == users[1]

    local password
    repeat
        io.write("\x1b[0;36mPassword: ")
        password = io.read()
        if hash(password) ~= users[2] then print("\x1b[0;31mPassword doesnt match.\x1b[0m") end
    until verify(password, users[2])

    io.close(user_text)
    io.close(data)
end

if terminal.clear_on_bootstrapper_quit then
    terminal.clear()
end

--print("Name: " .. user[1])
--print("Password-hash: " .. user[2])