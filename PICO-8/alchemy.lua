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

-- blitz ore material
blitz_ore = {
    id = 'blitz_ore',
}

-- copper ingot material
copper_ingot = {
    id = 'copper_ingot',
    nodes = nil, -- TODO
}

-- slot IDs
ring_1_c  = 1

ring_2_e  = 1
ring_2_w  = 2

ring_3_ne = 1
ring_3_nw = 2
ring_3_sw = 3
ring_3_se = 4

ring_4_ne = 1
ring_4_n  = 2
ring_4_nw = 3
ring_4_sw = 4
ring_4_s  = 5
ring_4_se = 6

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

-- static pylon recipe
pylon = {
    id = 'pylon',
    nodes = {
        -- required
        {
            {
                condition = {match_item, blitz_ore.id},
                -- ring 1 has the implicit effect of setting the item id
            }
        },
        -- major variant
        {
            -- range
            {
                condition = {match_material, material_type_ingot},
                effect = {radius, 10},
            },
            -- damage
            {
                condition = {match_item, blitz_ore.id},
                effect = {damage, 10},
            },
        },
        -- minor variant or bonus
        {
            -- duration
            {
                condition = {match_material, material_type_rock},
                effect = {duration, 10},
            },
        },
        -- polar shared nodes should be quality
        -- other ones are harder to reach and should have more powerful effects
    },
}

function draw_alchemy_diagram()
    local cx = 64
    local cy = 64
    local r = 16
    local color = 0
    local ring_max = 4

    local sector_centers = {}

    -- draw ring 1 (the required center node)
    circfill(cx, cy, r / 2, color)
    color = color + 1
    sector_centers[1] = {{cx, cy}}

    -- draw rings 2-4
    for ring_index = 2, ring_max do
        sector_centers[ring_index] = {}
        local num_slots = 2 * (ring_index - 1)
        local dtheta = 1 / num_slots
        local rmin = (r / 2) + r * (ring_index - 2)
        local rmax = rmin + r
        local thetaoffset = 0
        -- ring 2 is rotated halfway to look nicer
        if ring_index == 2 then
            thetaoffset = thetaoffset - 0.25
        end
        for sector_index = 1, num_slots do
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
            sector_centers[ring_index][sector_index] = {tx, ty}
        end
    end

    -- draw inter-node lines and collect nodes
    -- todo: light up only paths that can be selected
    -- start with the root node
    local node_centers = {{{cx, cy}}}
    for ring_index_2 = 2, ring_max do
        node_centers[ring_index_2] = {}
        local ring_index_1 = ring_index_2 - 1
        for sector_links in all(ring_links[ring_index_1]) do
            local sector_index_1, sector_indexes = unpack(sector_links)
            for sector_index_2 in all(sector_indexes) do
                local x1, y1 = unpack(sector_centers[ring_index_1][sector_index_1])
                local x2, y2 = unpack(sector_centers[ring_index_2][sector_index_2])
                -- draw lines between ring centers
                line(x1, y1, x2, y2, 0)
                -- add the node at the end of the line
                node_centers[ring_index_2][sector_index_2] = {x2, y2}
            end
        end
    end

    -- draw nodes
    -- todo: light up only filled nodes
    local shape = 0
    for ring_nodes in all(node_centers) do
        for sector_nodes in all(ring_nodes) do
            local x, y = unpack(sector_nodes)
            circfill(x, y, r * 0.35, 0)
            item_sspr(shape, x - 3, y - 3, 8, 8)
            shape = shape + 1
        end
    end
end

function _draw()
    cls()

    draw_alchemy_diagram()
end

function _update60()

end