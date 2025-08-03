-- reusable math functions

function lerp(a, b, t)
 return (1 - t) * a + t * b
end

--{{{ vector

-- 2D vector
V2 = mkclass()

function V2:init(x, y)
 self.x = x
 self.y = y
end

function V2:__eq(v)
 return self.x == v.x and self.y == v.y
end

function V2:__add(v)
 return V2(self.x + v.x, self.y + v.y)
end

function V2:__sub(v)
 return V2(self.x - v.x, self.y - v.y)
end

-- element-wise and scalar multiplication
function V2:__mul(v)
 if instanceof(v, V2) then
  return V2(self.x * v.x, self.y * v.y)
 end
 -- assume it's a scalar
 return V2(self.x * v, self.y * v)
end

function V2:__div(v)
 if instanceof(v, V2) then
  return V2(self.x / v.x, self.y / v.y)
 end
 return V2(self.x / v, self.y / v)
end

function V2:dot(v)
 return self.x * v.x + self.y * v.y
end

function V2:mag()
 return sqrt(self:dot(self))
end

-- normalize vector
function V2:norm()
 if self.x == 0 and self.y == 0 then
  -- not defined
  return nil
 end
 return self / self:mag()
end

-- 2D cross product: return magnitude of cross vector
function V2:cross(v)
 return self.x * v.y - self.y * v.x
end

--}}}
