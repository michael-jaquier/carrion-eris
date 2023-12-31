package.path = "/opt/homebrew/Cellar/luarocks/3.9.2/share/lua/5.4/?.lua;/opt/homebrew/share/lua/5.4/?.lua;/opt/homebrew/share/lua/5.4/?/init.lua;/opt/homebrew/lib/lua/5.4/?.lua;/opt/homebrew/lib/lua/5.4/?/init.lua;./?.lua;./?/init.lua;/Users/michael.jaquier/.luarocks/share/lua/5.4/?.lua;/Users/michael.jaquier/.luarocks/share/lua/5.4/?/init.lua"
local json = require('dkjson')

count = 0
function print_map()
    io.write("Took ")
    io.write(count)
    io.write(" iterations")
    io.write("\n")
for y = 1, map_size.y do
    for x = 1, map_size.x do
        local tile = map[x][y][1].tile_set
     
        if tile == "Water" then
            io.write("W ")
        elseif tile == "Coast" then
            io.write("C ")
        elseif tile == "Grass" then
            io.write("G ")
        elseif tile == "Mountain" then
            io.write("M ")
        elseif tile == "Swamp" then
            io.write("S ")
        elseif tile == "Plains" then
            io.write("P ")
        elseif tile == "Cave" then
            io.write("Q ")
        else
            io.write("V ")
        end
    end
    io.write("\n")
    end
    io.write("\n")
end
-- 1. Data
map_size = {x = 20, y = 20, z = 1} -- Replace with your actual map size
map = {}

tile_set = {"Water", "Coast", "Grass", "Mountain", "Swamp", "Plains", "Cave"}

constraints = {
    Cave = {"Cave, Plains"},
    Plains = {"Plains", "Grass", "Mountain", "Swamp"},
    Swamp = {"Swamp", "Grass", "Water", "Plains"},
    Water = {"Water", "Coast"},
    Coast = {"Water", "Grass", "Coast"},
    Grass = {"Coast", "Mountain", "Grass", "Swamp"},
    Mountain = {"Mountain", "Grass", "Plains"}
}
weights = {
    Cave = 0.01,
    Plains = 0.3,
    Swamp = 0.03,
    Water = 0.03,
    Coast = 0.03,
    Grass = 0.3,
    Mountain = 0.08

}

local constraint_count = 0
for _ in pairs(constraints) do
    constraint_count = constraint_count + 1
end
local weights_count = 0
for _ in pairs(weights) do
    weights_count = weights_count + 1
end

assert(#tile_set == constraint_count, "Number of tiles and constraints must match")
assert(#tile_set == weights_count, "Number of tiles and weights must match")

for x = 1, map_size.x do
    map[x] = {}
    for y = 1, map_size.y do
        map[x][y] = {}
        for z = 1, map_size.z do
            map[x][y][z] = { tile_set = tile_set, collapsed = false }
        
        end
    end
end



local function get_lowest_entropy()
    local min_entropy = math.huge
    local min_x, min_y, min_z
    for x = 1, map_size.x do
        for y = 1, map_size.y do
            for z = 1, map_size.z do
                if map[x][y][z].collapsed == true then
                    goto continue
                end
                local entropy = #map[x][y][z].tile_set
                if entropy < min_entropy then
                    min_entropy = entropy
                    min_x, min_y, min_z = x, y, z
                end
                ::continue::
        end
    end
end
    return min_x, min_y, min_z
end

local function contains(list, element)
    for _, value in ipairs(list) do
        if value == element then
            return true
        end
    end
    return false
end
                                        
local function update_neighbor_entropy (x,y,z) 

    -- Update all neighbors of x,y,z entropy with the new entropy value
    local directions = {
        {x = -1, y = 0, z = 0}, -- west
        {x = 1, y = 0, z = 0}, -- east
        {x = 0, y = -1, z = 0}, -- south
        {x = 0, y = 1, z = 0}, -- north
        {x = 0, y = 0, z = -1}, -- down
        {x = 0, y = 0, z = 1} -- up
    }

    for _, direction in ipairs(directions) do
        local nx, ny, nz = x + direction.x, y + direction.y, z + direction.z
        if nx >= 1 and nx <= map_size.x and ny >= 1 and ny <= map_size.y and nz >= 1 and nz <= map_size.z then
            if map[nx][ny][nz].collapsed then
               goto continue 
            end
            local possible_tiles = map[nx][ny][nz].tile_set
            
            local neighbor_constraints = constraints[map[x][y][z].tile_set]
            local new_possible_tiles = {}

            for _, tile in ipairs(possible_tiles) do
                if contains(neighbor_constraints, tile) then
                    table.insert(new_possible_tiles, tile)
                end
            end

            map[nx][ny][nz].tile_set = new_possible_tiles
            if #new_possible_tiles == 1 then
                map[nx][ny][nz].collapsed = true
            end
        end
        ::continue::
    end
end

local function weighted_random_choice(possible_tiles)
    local total_weight = 0
    for _, tile in ipairs(possible_tiles) do
        total_weight = total_weight + weights[tile]
    end
    local choice = math.random() * total_weight
    for _, tile in ipairs(possible_tiles) do
        if weights[tile] >= choice then
            return tile
        end
        choice = choice - weights[tile]
    end
end

local function procedural_generation()
    while true do
        count = count + 1
        local x, y, z = get_lowest_entropy()
        if not x then break end
        local possible_tiles = map[x][y][z].tile_set
        local tile = weighted_random_choice(possible_tiles)
        map[x][y][z].tile_set = tile
        map[x][y][z].collapsed = true
        update_neighbor_entropy(x, y, z)
    end
end

------------------------------------
-- 3. Finishing

procedural_generation()
local function write_map_to_file(file_path)
    local file = io.open(file_path, "w")
    if not file then
        print("Could not open file: " .. file_path)
        return
    end

    local tiles = {}
    for y = 1, map_size.y do
        for x = 1, map_size.x do
            for z = 1, map_size.z do 
                local tile = map[x][y][z].tile_set
                local tile_string = tostring(tile) 
                local descriptor = tile_string .. " at location " .. "(".. x .. "," .. y .. "," .. z .. ")"
                table.insert(tiles, {x = x, y = y, z = z, descriptor = descriptor, tile = tile})
        end
    end
end
    local json_string = json.encode(tiles, { indent = true })
    file:write(json_string)
    file:close()
end

-- Usage:
write_map_to_file("map.json")