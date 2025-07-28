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
ring_1_c = 1

ring_2_e = 1
ring_2_w = 2

ring_3_ne = 1
ring_3_nw = 2
ring_3_sw = 3
ring_3_se = 4

ring_4_ne = 1
ring_4_n = 2
ring_4_nw = 3
ring_4_sw = 4
ring_4_s = 5
ring_4_se = 6

recipe_shape_standard = {
 rings = 4,
 num_slot_ids_by_ring = { 1, 2, 4, 6 },
 -- ring 2 is rotated halfway to look nicer
 theta_offsets = { 0, -0.25, 0, 0 },
 -- each pair of rings maps a slot in the first ring
 -- to a list of slots in the second ring
 ring_links = {
  -- ring 1 to 2
  {
   {
    ring_1_c,
    {
     ring_2_e,
    },
   },
   {
    ring_1_c,
    {
     ring_2_w,
    },
   },
  },
  -- ring 2 to 3
  {
   {
    ring_2_e,
    {
     ring_3_ne,
     ring_3_se,
    },
   },
   {
    ring_2_w,
    {
     ring_3_nw,
     ring_3_sw,
    },
   },
  },
  -- ring 3 to 4
  {
   {
    ring_3_ne,
    {
     ring_4_ne,
     ring_4_n,
    },
   },
   {
    ring_3_nw,
    {
     ring_4_n,
     ring_4_nw,
    },
   },
   {
    ring_3_sw,
    {
     ring_4_sw,
     ring_4_s,
    },
   },
   {
    ring_3_se,
    {
     ring_4_s,
     ring_4_se,
    },
   }
  },
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

-- https://www.lua.org/pil/16.1.html
function mkclass()
 local cls = {}
 cls.__index = cls
 return cls
end

SynthesisState = mkclass()

-- a SynthesisState tracks progress on a recipe
-- material must have a recipe attached
function SynthesisState:new(material)
 local new = {
  material = material,
  -- list of { slot ID, item } pairs for each filled ring
  choices = {},
 }
 setmetatable(new, self)
 return new
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
function SynthesisState:ring()
 return #self.choices + 1
end

-- returns whether a given slot on the current ring will accept a given item
function SynthesisState:can_place(slot_id, item)
 local match, id = unpack(self.material.recipe.nodes[self:ring()][slot_id])
 return (match == match_item and item.material.id == id)
  or (match == match_material and item.material.type == id)
end

-- place a given item in a given slot ID on the current ring
-- assumes you already checked if it'll go there
function SynthesisState:place(slot_id, item)
 add(self.choices, { slot_id, item })
end

-- pop the choices stack
function SynthesisState:undo()
 if #self.choices > 0 then
  deli(self.choices)
 end
end

-- list of items that are in use by this synthesis
function SynthesisState:items()
 local result = {}
 for entry in all(self.choices) do
  add(entry[2])
 end
 return result
end

function draw_alchemy_diagram()
 local cx = 64
 local cy = 64
 local r = 16
 local color = 0

 local inventory = {
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

 local synstate = SynthesisState:new(pylon)
 synstate:place(ring_1_c, inventory[1])
 synstate:place(ring_2_e, inventory[2])
 synstate:place(ring_3_ne, inventory[3])

 local sector_centers = {}

 -- draw ring 1 (the required center node)
 circfill(cx, cy, r / 2, color)
 color = color + 1
 sector_centers[1] = { { cx, cy } }

 -- draw rings 2-4
 for ring_index = 2, synstate:ring() do
  sector_centers[ring_index] = {}
  local num_slots = 2 * (ring_index - 1)
  local dtheta = 1 / num_slots
  local rmin = (r / 2) + r * (ring_index - 2)
  local rmax = rmin + r
  local thetaoffset = synstate.material.recipe.shape.theta_offsets[ring_index]
  for sector_index = 1, synstate.material.recipe.shape.num_slot_ids_by_ring[ring_index] do
   local thetamin = (sector_index - 1) * dtheta
   local thetamax = thetamin + dtheta
   color = color + 1
   arcfill(
    cx,
    cy,
    rmin,
    rmax,
    thetamin,
    thetamax,
    thetaoffset,
    color
   )
   local tx, ty = arccenter(
    cx,
    cy,
    rmin,
    rmax,
    thetamin,
    thetamax,
    thetaoffset
   )
   sector_centers[ring_index][sector_index] = { tx, ty }
  end
 end

 -- draw inter-node lines and collect nodes
 -- todo: light up only paths that can be selected
 -- start with the root node
 local node_centers = { { { cx, cy } } }
 for ring_index_2 = 2, synstate:ring() do
  node_centers[ring_index_2] = {}
  local ring_index_1 = ring_index_2 - 1
  for sector_links in all(synstate.material.recipe.shape.ring_links[ring_index_1]) do
   local sector_index_1, sector_indexes = unpack(sector_links)
   for sector_index_2 in all(sector_indexes) do
    local x1, y1 = unpack(sector_centers[ring_index_1][sector_index_1])
    local x2, y2 = unpack(sector_centers[ring_index_2][sector_index_2])
    -- draw lines between ring centers
    line(x1, y1, x2, y2, 0)
    -- add the node at the end of the line
    node_centers[ring_index_2][sector_index_2] = { x2, y2 }
   end
  end
 end

 -- draw nodes
 -- todo: light up only filled nodes
 for ring_index, ring_nodes in pairs(node_centers) do
  local choice = synstate.choices[ring_index]
  if choice ~= nil then
   local slot_id, item = unpack(choice)
   for sector_id, sector_nodes in pairs(ring_nodes) do
    local x, y = unpack(sector_nodes)
    circfill(x, y, r * 0.35, 0)
    if slot_id == sector_id then
     item_sspr(item.material.big_sprite_index, x - 3, y - 3, 8, 8)
    end
   end
  end
 end
end

function _draw()
 cls()

 draw_alchemy_diagram()
end

function _update60()

end