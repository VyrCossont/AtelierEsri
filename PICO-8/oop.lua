-- impose some structure on the table soup that is Lua

-- https://www.lua.org/pil/16.1.html
function mkclass(defaults)
 local cls = defaults or {}
 -- class instances inherit defaults from the class table
 cls.__index = cls
 -- the class can be called to construct class instances
 local mt = {}
 function mt:__call(obj)
  setmetatable(obj, cls)
  return obj
 end
 setmetatable(cls, mt)
 return cls
end
