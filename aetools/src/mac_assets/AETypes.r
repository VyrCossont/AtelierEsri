/** Atelier Esri custom resource types. */

/* ResEdit template resource. (Not actually ours, but not in Apple RIncludes.) */
type 'TMPL' {
    array fields {
        pstring;    /* label */
        string[4];  /* type */
    };
};

/* Region list: locations of individual sprites within a sprite sheet. */
type 'RGN#' {
    unsigned integer = $$CountOf(regions);
    array regions {
        pstring;    /* name */
        align word;
        rect;       /* frame */
    };
};

/* Template for region lists. */
resource 'TMPL' (128, "RGN#") {
	{	/* array fields: 6 elements */
		/* [1] */
		"NumRegions",
		"OCNT",
		/* [2] */
		"*****",
		"LSTC",
		/* [3] */
		"Name",
		"PSTR",
		/* [4] */
		"*****",
		"AWRD",
		/* [5] */
		"Frame",
		"RECT",
		/* [6] */
		"*****",
		"LSTE"
	}
};

/* 9-patch list: locations of 9-patch regions within a sprite sheet. */
type '9PC#' {
    unsigned integer = $$CountOf(patches);
    array patches {
        pstring;    /* name */
        align word;
        rect;       /* frame */
        rect;       /* center: coordinates relative to frame origin */
    };
};

/* Template for region lists. */
resource 'TMPL' (129, "9PC#") {
	{	/* array fields: 7 elements */
		/* [1] */
		"NumRegions",
		"OCNT",
		/* [2] */
		"*****",
		"LSTC",
		/* [3] */
		"Name",
		"PSTR",
		/* [4] */
		"*****",
		"AWRD",
		/* [5] */
		"Frame",
		"RECT",
		/* [6] */
		"Center",
		"RECT",
		/* [7] */
		"*****",
		"LSTE"
	}
};

/* Tileset. */
type 'TSX ' {
    integer;    /* tile width */
    integer;    /* tile height */
    integer;    /* image width (in pixels) */
    integer;    /* image height (in pixels) */
    integer;    /* image PICT resource ID */
    integer;    /* mask PICT resource ID. 0 indicates no mask. */
};

/* Template for tilesets. */
resource 'TMPL' (130, "TSX ") {
	{	/* array fields: 8 elements */
		/* [1] */
		"Name",
		"PSTR",
		/* [2] */
		"*****",
		"AWRD",
		/* [3] */
		"Tile width",
		"DWRD",
		/* [4] */
		"Tile height",
		"DWRD",
		/* [5] */
		"Image width (in pixels)",
		"DWRD",
		/* [6] */
		"Image height (in pixels)",
		"DWRD",
		/* [7] */
		"Image PICT resource ID",
		"DWRD",
		/* [8] */
		"Mask PICT resource ID (0 indicates no mask)",
		"DWRD",
	}
};

/* Tilemap. */
type 'TMX ' {
    unsigned integer = $$CountOf(tileset_resource_ids);
    array tileset_resource_ids {
        integer;    /* TSX tileset resource ID */
    };

    unsigned integer = $$CountOf(tile_layers);
    array tile_layers {
        pstring;    /* name */
        align word;
        integer;    /* width (in tiles) */
        integer;    /* height (in tiles) */

        unsigned integer = $$CountOf(tiles);
        array tiles {
            unsigned bitstring[1];  /* flags: flip h */
            unsigned bitstring[1];  /* flags: flip v */
            unsigned bitstring[1];  /* flags: flip d */
            align byte;
            unsigned byte;          /* tileset ordinal (index + 1; 0 indicates an empty tile position) */
            unsigned integer;       /* tile ID within tileset */
        };
    };

    unsigned integer = $$CountOf(region_groups);
    array region_groups {
        pstring;    /* name */
        align word;
        integer;    /* RGN# region list resource ID */
    };
};

/* Template for tilemaps. */
resource 'TMPL' (131, "TMX ") {
    /* TODO */
    {}
};
