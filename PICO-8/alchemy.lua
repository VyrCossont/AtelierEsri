-- alchemy data model

match_material = 'match_material'
match_item = 'match_item'

-- item properties

-- flat damage addition
damage = 'damage'
-- projectiles go thru one additional enemy per pierce level
pierce = 'pierce'
-- increases effect radius
radius = 'radius'
-- harvest increases material drop chance
harvest = 'harvest'
-- deployables with longer duration last longer
duration = 'duration'
-- quality multiplies pretty much every good stat somewhere
quality = 'quality'
-- burn and other elemental effect types add chance to proc that effect
burn = 'burn'

-- material types
material_type_ingot = 'ingot'
material_type_ore = 'ore'
material_type_crystal = 'crystal'
material_type_rock = 'rock'
material_type_deployable = 'deployable'

-- blitz ore material
blitz_ore = {
 id = 'blitz_ore',
 type = material_type_ore,
 big_sprite_index = 11, -- elerium
}

-- copper ingot material
copper_ingot = {
 id = 'copper_ingot',
 type = material_type_ingot,
 big_sprite_index = 3, -- copper bar
 nodes = nil, -- TODO
}

-- granite material
granite = {
 id = 'granite',
 type = material_type_rock,
 big_sprite_index = 31, -- lump?
}

-- slot IDs for standard recipe shape
ring_1 = 1
ring_1_c = 1

ring_2 = 2
ring_2_e = 1
ring_2_w = 2

ring_3 = 3
ring_3_ne = 1
ring_3_nw = 2
ring_3_sw = 3
ring_3_se = 4

ring_4 = 4
ring_4_ne = 1
ring_4_n = 2
ring_4_nw = 3
ring_4_sw = 4
ring_4_s = 5
ring_4_se = 6

Ring = mkclass {
 thetaoffset = 0,
}

function Ring:num_slots()
 return #self.links
end

RecipeShape = mkclass()

-- a recipe shape is a list of rings
function RecipeShape:init(rings)
 self.rings = rings
end

function RecipeShape:num_rings()
 return #self.rings
end

-- polar coords each ring slot falls into
-- returns rmin, rmax, thetamin, thetamax, thetaoffset
function RecipeShape:slot_bounds(ring_index, slot_index)
 if ring_index == ring_1 and slot_index == ring_1_c then
  -- ring 1 is special, only rmax is really relevant
  return 0, 0.5, 0, 1, 0
 end

 local rmin = 0.5 + (ring_index - 2)
 local rmax = rmin + 1
 local ring = self.rings[ring_index]
 local dtheta = 1 / ring:num_slots()
 local thetaoffset = ring.thetaoffset
 local thetamin = (slot_index - 1) * dtheta
 local thetamax = thetamin + dtheta
 return
  rmin,
  rmax,
  thetamin,
  thetamax,
  thetaoffset
end

-- find the slot for a given ring and angle
function RecipeShape:slot_index(ring_index, theta)
 local ring = self.rings[ring_index]
 theta = theta - ring.thetaoffset
 theta = theta % 1
 return 1 + flr(theta * ring:num_slots())
end

recipe_shape_standard = RecipeShape {
 [ring_1] = Ring {
  links = {
   [ring_1_c] = { ring_2_e, ring_2_w },
  },
 },
 [ring_2] = Ring {
  -- ring 2 is rotated halfway to look nicer
  thetaoffset = -0.25,
  links = {
   [ring_2_e] = { ring_3_ne, ring_3_se },
   [ring_2_w] = { ring_3_nw, ring_3_sw },
  }
 },
 [ring_3] = Ring {
  links = {
   [ring_3_ne] = { ring_4_ne, ring_4_n },
   [ring_3_nw] = { ring_4_n, ring_4_nw },
   [ring_3_sw] = { ring_4_sw, ring_4_s },
   [ring_3_se] = { ring_4_s, ring_4_se },
  },
 },
 [ring_4] = Ring {
  -- we need the keys to provide the list of slots,
  -- but ring 4 is the last so the links for each slot are empty
  links = {
   [ring_4_ne] = {},
   [ring_4_n] = {},
   [ring_4_nw] = {},
   [ring_4_sw] = {},
   [ring_4_s] = {},
   [ring_4_se] = {},
  }
 }
}

-- static pylon recipe
pylon = {
 id = 'pylon',
 type = material_type_deployable,
 big_sprite_index = 36, -- flower 2
 recipe = {
  shape = recipe_shape_standard,
  nodes = {
   -- required
   {
    {
     condition = { match_item, blitz_ore.id },
     -- ring 1 has the implicit effect of setting the item id
    }
   },
   -- major variant
   {
    -- range
    {
     condition = { match_material, material_type_ingot },
     effect = { radius, 10 },
    },
    -- damage
    {
     condition = { match_item, blitz_ore.id },
     effect = { damage, 10 },
    },
   },
   -- minor variant or bonus
   {
    -- duration
    {
     condition = { match_material, material_type_rock },
     effect = { duration, 10 },
    },
   },
   -- polar shared nodes should be quality
   -- other ones are harder to reach and should have more powerful effects
  },
 },
}

-- a SynthesisState tracks progress on a recipe
-- material must have a recipe attached
SynthesisState = mkclass()

function SynthesisState:init(material)
 self.material = material
 -- list of { slot ID, item } pairs for each filled ring
 self.choices = {}

 -- input state
 self.theta = 0
 self.btn_cooldown = 0

 -- draw state
 self.slot_index = 1
 self.ring_dirty = true
end

-- for now, we can finish any synthesis if we've placed the center item
function SynthesisState:can_finish()
 return #self.choices > 0
end

-- diagram's full, stop adding stuff
function SynthesisState:must_finish()
 return #self.choices >= #self.material.recipe.nodes
end

-- the current ring (the one the player is picking a slot on)
-- todo: someday we want them to be able to add multiple items to a single ring
--  but for now they can't do that
function SynthesisState:ring_index()
 return #self.choices + 1
end

-- returns whether a given slot on the current ring will accept a given item
function SynthesisState:can_place(slot_id, item)
 local match, id = unpack(self.material.recipe.nodes[self:ring_index()][slot_id])
 return (match == match_item and item.material.id == id)
  or (match == match_material and item.material.type == id)
end

-- place a given item in a given slot ID on the current ring
-- assumes you already checked if it'll go there
function SynthesisState:place(slot_id, item)
 add(self.choices, { slot_id, item })
 self.ring_dirty = true
end

-- pop the choices stack
function SynthesisState:undo()
 if #self.choices > 0 then
  deli(self.choices)
 end
 self.ring_dirty = true
end

-- track the input to update the selected slot
function SynthesisState:track(lx, ly, btn_a, btn_b)
 if self.btn_cooldown > 1 then
  if not btn_a and not btn_b then
   self.btn_cooldown = self.btn_cooldown - 1
  end
 else
  local cooldown_max = 5
  if btn_a then
   self.btn_cooldown = cooldown_max
   if self:ring_index() == 2 then
    self:place(self.slot_index, inventory[2])
   elseif self:ring_index() == 3 then
    self:place(self.slot_index, inventory[3])
   else
    print("would finish synthesis")
   end
  elseif btn_b then
   self.btn_cooldown = cooldown_max
   if self:ring_index() == 2 then
    print("would abort synthesis")
   else
    self:undo()
   end
  end
 end

 if lx ~= 0 or ly ~= 0 then
  self.theta = atan2(lx, ly)
  local slot_index = self.material.recipe.shape:slot_index(self:ring_index(), self.theta)
  if slot_index ~= self.slot_index then
   self.ring_dirty = true
   self.slot_index = slot_index
  end
 end
end

-- list of items that are in use by this synthesis
function SynthesisState:items()
 local result = {}
 for _, item in all(self.choices) do
  add(item)
 end
 return result
end

inventory = {
 {
  material = blitz_ore,
 },
 {
  material = copper_ingot,
 },
 {
  material = granite,
 }
}

synstate = SynthesisState(pylon)
synstate:place(ring_1_c, inventory[1])
--synstate:place(ring_2_e, inventory[2])
--synstate:place(ring_3_ne, inventory[3])

function draw_alchemy_diagram()
 local c = V2(64, 64)
 local r = 16

 -- selected slot
 local rmin, rmax, thetamin, thetamax, thetaoffset = synstate.material.recipe.shape:slot_bounds(synstate:ring_index(), synstate.slot_index)

 -- draw ring 1 (the required center node)
 circfill(c.x, c.y, rmin * r, 1)
 circfill(c.x, c.y, r / 2, 0)

 -- draw rings 2-4
 arcfill(
  c.x,
  c.y,
  rmin * r,
  rmax * r,
  thetamin,
  thetamax,
  thetaoffset,
  1
 )
 local slot_rel_c = V2(arccenter(
  c.x,
  c.y,
  rmin * r,
  rmax * r,
  thetamin,
  thetamax,
  thetaoffset
 )) - c
 local end_rel_slot = slot_rel_c:norm():rotate(0.25) * 80
 local end1 = c + slot_rel_c + end_rel_slot
 local end2 = c + slot_rel_c - end_rel_slot
 for i = 0, 100, 1 do
  local a = i / 100
  local v = lerp(end1, end2, a)
  circfill(v.x, v.y, 8, 2)
 end
 for i = 0, 100, 1 do
  local a = i / 100
  local v = lerp(end1, end2, a)
  circfill(v.x, v.y, 6, 0)
 end
 local item = 0
 for i = 0, 100, 10 do
  local a = i / 100
  local v = lerp(end1, end2, a)
  circfill(v.x, v.y, 5, 1)
  item_sspr(item, v.x - 3, v.y - 3 , 8, 8)
  item = item + 1
 end
end

-- todo: reintegrate this stuff
function donothing()
 -- draw inter-node lines and collect nodes
 -- todo: light up only paths that can be selected
 -- start with the root node
 local node_centers = { { { cx, cy } } }
 for ring_index_2 = 2, synstate:ring_index() do
  node_centers[ring_index_2] = {}
  local ring_index_1 = ring_index_2 - 1
  local choice_1 = synstate.choices[ring_index_1]
  local choice_2 = synstate.choices[ring_index_2]
  if choice_1 ~= nil then
   local slot_id_1 = choice_1[1]
   local slot_id_2s
   if choice_2 ~= nil then
    -- draw two connected filled nodes
    slot_id_2s = { choice_2[1] }
   else
    -- draw a filled node and empty nodes next to it
    local ring_1 = synstate.material.recipe.shape[ring_index_1]
    slot_id_2s = ring_1.links[slot_id_1]
   end
   for slot_id_2 in all(slot_id_2s) do
    local x1, y1 = unpack(sector_centers[ring_index_1][slot_id_1])
    local x2, y2 = unpack(sector_centers[ring_index_2][slot_id_2])
    -- draw lines between ring centers
    line(x1, y1, x2, y2, 0)
    -- add the node at the end of the line
    node_centers[ring_index_2][slot_id_2] = { x2, y2 }
   end
  end
 end

 -- draw nodes
 -- todo: light up only filled nodes
 for ring_index, ring_nodes in pairs(node_centers) do
  local choice = synstate.choices[ring_index]
  if choice ~= nil then
   local slot_id, item = unpack(choice)
   local x, y = unpack(ring_nodes[slot_id])
   circfill(x, y, r * 0.35, 0)
   item_sspr(item.material.big_sprite_index, x - 3, y - 3, 8, 8)
  end
 end
end

syn_theta = 0
syn_prev_ring_index = nil
syn_prev_slot_index = nil

function _draw()
 if not input_is_inited() then
  input_draw_instructions()
  return
 elseif not input_is_connected() then
  cls()
  print("player #1, connect your gamepad...")
  return
 end

 if synstate ~= nil and synstate.ring_dirty then
  cls()
  draw_alchemy_diagram()
  synstate.ring_dirty = false
 end
end

function _update60()
 if synstate ~= nil then
  local lx, ly = input_stick(pi_l)
  synstate:track(lx, ly, input_button(pi_a), input_button(pi_b))
 end
end