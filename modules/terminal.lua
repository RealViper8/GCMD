local module = {}

require("modules")

module.ansi_code = false
module.clear_on_start = true
module.clear_on_bootstrapper_quit = true

function module.exit()
    os.exit(0)
end

function module.clear()
    if module.ansi_code == false then
        if os.getenv("OS") == "Windows_NT" then
            os.execute("cls")
        else
            os.execute("clear")
        end
    else
        print("\x1b[2J\x1b[0;0H")
    end
end

local function exists(file)
    local ok, err, code = os.rename(file, file)
    if not ok then
        if code == 13 then
            return true
        end
    end
    return ok, err
end

function module.isdir(path)
    return exists(path.."/")
end

function module.read_users(filename)
    local users = {}

    local file = io.open(filename, "r")
    if not file then
        print("Failed to open file:", filename)
        return nil
    end

    if file then
        for line in file:lines() do
            local username, password = line:match("([^,]+),(%d+)")

            if username and password then
                table.insert(users, {username = username, password = password})
            else
                print("Invalid data format in line:", line)
            end
        end
    end

    file:close()
    return users
end

return module