-- reusable math functions

function lerp(a, b, t)
 -- written backwards so that V2:__mul works
 return a * (1 - t) + b * t
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

-- https://en.wikipedia.org/wiki/Rotation_matrix
function V2:rotate(theta)
 local s = sin(theta)
 local c = cos(theta)
 return V2(
  self.x * c - self.y * s,
  self.x * s + self.y * c
 )
end

--}}}

--{{{ rectangle

-- rectangle
R2 = mkclass()

-- takes position and size
-- size components assumed to be non-negative
function R2:init(pos, size)
 self.pos = pos
 self.size = size
end

function R2:__tostring()
 return "R2("..tostr(self.pos)..", "..tostr(self.size)..")"
end

-- more negative corner
function R2:v1()
 return self.pos
end

-- more positive corner
function R2:v2()
 return self.pos + self.size
end

function R2:contains(v)
 local v1 = self:v1()
 local v2 = self:v2()
 return v.x >= v1.x
  and v.y >= v1.y
  and v.x <= v2.x
  and v.y <= v2.y
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
 return "T2("..tostr(self.a)..", "..tostr(self.b)..", "..tostr(self.c)..")"
end

function T2:contains(v)
 for pair in all({{self.a, self.b}, {self.b, self.c}, {self.c, self.a}}) do
  if not Halfplane(unpack(pair)):contains(v) then
   return false
  end
 end
 return true
end

-- returns rectangle containing the triangle
function T2:aabb()
 local xmin = min(self.a.x, min(self.b.x, self.c.x))
 local xmax = max(self.a.x, max(self.b.x, self.c.x))
 local ymin = min(self.a.y, min(self.b.y, self.c.y))
 local ymax = max(self.a.y, max(self.b.y, self.c.y))
 return R2(V2(xmin, ymin), V2(xmax - xmin, ymax - ymin))
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
