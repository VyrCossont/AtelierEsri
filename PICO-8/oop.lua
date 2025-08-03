-- impose some structure on the table soup that is Lua

-- https://www.lua.org/pil/16.1.html
function mkclass(defaults)
 local cls = defaults or {}
 -- instances of the class inherit properties from the class
 cls.__index = cls
 -- the class can be called to construct class instances
 local mt = {}
 function mt:__call(...)
  local obj
  if cls.init ~= nil then
   -- make a new table so we can call init on it later
   obj = {}
  else
   -- assume we were given a table with fields, or nothing
   obj = ... or {}
  end
  setmetatable(obj, cls)
  if obj.init ~= nil then
   obj:init(...)
  end
  return obj
 end
 setmetatable(cls, mt)
 return cls
end

function instanceof(obj, cls)
 return getmetatable(obj) == cls
end
