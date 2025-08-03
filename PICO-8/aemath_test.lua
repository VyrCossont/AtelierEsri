luaunit = require('luaunit')

require 'pico8_compat'
require 'oop'
require 'aemath'

--{{{ vector

test_aemath_vector = {}

function test_aemath_vector:test_add()
 luaunit.assertEquals(
  V2(2, 3) + V2(5, 7),
  V2(7, 10)
 )
end

function test_aemath_vector:test_sub()
 luaunit.assertEquals(
  V2(2, 3) - V2(5, 7),
  V2(-3, -4)
 )
end

function test_aemath_vector:test_mul()
 luaunit.assertEquals(
  V2(2, 3) * V2(5, 7),
  V2(10, 21)
 )
 luaunit.assertEquals(
  V2(2, 3) * 5,
  V2(10, 15)
 )
 end

function test_aemath_vector:test_div()
 luaunit.assertEquals(
  V2(2, 3) / V2(5, 7),
  V2(2 / 5, 3 / 7)
 )
 luaunit.assertEquals(
  V2(2, 3) / 5,
  V2(2 / 5, 3 / 5)
 )
end

function test_aemath_vector:test_norm()
 luaunit.assertEquals(
  V2(0, 0):norm(),
  nil
 )
 luaunit.assertEquals(
  V2(2, 0):norm(),
  V2(1, 0)
 )
end

function test_aemath_vector:test_cross()
 -- parallel
 luaunit.assertEquals(
  V2(1, 0):cross(V2(1, 0)),
  0
 )
 -- antiparallel
 luaunit.assertEquals(
  V2(1, 0):cross(V2(-1, 0)),
  0
 )
 -- perpendicular
 luaunit.assertEquals(
  V2(1, 0):cross(V2(0, 1)),
  1
 )
 -- perpendicular the other way
 luaunit.assertEquals(
  V2(1, 0):cross(V2(0, -1)),
  -1
 )
end

--}}}

os.exit(luaunit.LuaUnit.run())
