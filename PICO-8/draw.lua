-- slow but precise filled arc
-- assumes rmin < rmax and thetamin < thetamax
function arcfill(cx, cy, rmin, rmax, thetamin, thetamax, thetaoffset, color)
    local xmin = cx - rmax
    local xmax = cx + rmax
    local ymin = cy - rmax
    local ymax = cy + rmax
    for y = ymin, ymax do
        for x = xmin, xmax do
            local dx = x - cx
            local dy = y - cy
            local r = sqrt(dx * dx + dy * dy)
            local theta = atan2(dx, dy) - thetaoffset
            theta = theta % 1
            if rmin <= r and r < rmax and thetamin <= theta and theta < thetamax then
                pset(x, y, color)
            end
        end
    end
end

-- return center (x, y) of arc
function arccenter(cx, cy, rmin, rmax, thetamin, thetamax, thetaoffset)
    local r = lerp(rmin, rmax, 0.5)
    local theta = lerp(thetamin, thetamax, 0.5) + thetaoffset
    theta = theta % 1
    return cx + r * cos(theta), cy + r * sin(theta)
end

-- draw item sprite given big (2x2) sprite number
function item_spr(n, x, y, flip_x, flip_y)
    local s = flr(n / 8) * 2 + (n % 8) * 32
    spr(s, x, y, 2, 2, flip_x, flip_y)
end

-- draw item scaled sprite given big (2x2) sprite number
function item_sspr(n, x, y, w, h, flip_x, flip_y)
    local sprite_x = flr(n / 8) * 16
    local sprite_y = (n % 8) * 16
    sspr(sprite_x, sprite_y, 16, 16, x, y, w, h, flip_x, flip_y)
end
