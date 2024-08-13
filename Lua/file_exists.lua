#!/usr/bin/lua
function file_exists(name)
        local file=io.open(name,"r")
        if file~=nil then
                io.close(file)
                return true
        else
                return false
        end
end
