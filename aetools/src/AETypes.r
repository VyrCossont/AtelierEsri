/** Atelier Esri custom resource types. */

/* ResEdit template resource. (Not actually ours, but not in Apple RIncludes.) */
type 'TMPL' {
    array fields {
        pstring;    /* label */
        string[4];  /* type */
    };
};

/* Template for region lists. */
resource 'TMPL' (128, "RGN#") {
	{	/* array fields: 5 elements */
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
		"Frame",
		"RECT",
		/* [5] */
		"*****",
		"LSTE"
	}
};

/* Region list: locations of individual sprites within a sprite sheet. */
type 'RGN#' {
    unsigned integer = $$CountOf(regions);
    array regions {
        pstring;    /* name */
        rect;       /* frame */
    };
};
