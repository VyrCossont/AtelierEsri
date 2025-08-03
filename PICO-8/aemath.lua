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

function V2:__tostring()
 return "V2("..self.x..", "..self.y..")"
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

--{{{ triangle

-- triangle
T2 = mkclass()

-- takes 3 position vectors
function T2:init(a, b, c)
 self.a = a
 self.b = b
 self.c = c
end

function T2:__tostring()
 return "T2("..self.a..", "..self.b..", "..self.c..")"
end

function T2:contains(v)
 for pair in all({{self.a, self.b}, {self.b, self.c}, {self.c, self.a}}) do
  if not Halfplane(unpack(pair)):contains(v) then
   return false
  end
 end
 return true
end

Halfplane = mkclass()

function Halfplane:init(p, q)
 self.p = p
 self.a = p.y - q.y
 self.b = q.x - p.x
 self.c = -p.x
end

function Halfplane:contains(v)
 v = v - self.p
 return v.x * self.a + v.y * self.b + self.c <= 0
end

--}}}
