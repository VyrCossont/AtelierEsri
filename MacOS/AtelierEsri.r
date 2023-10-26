#include "Dialogs.r"

resource 'ALRT' (128, purgeable) {
	{50, 30, 170, 410},
	128,
	{	/* array: 4 elements */
		/* [1] */
		OK, visible, sound1,
		/* [2] */
		OK, visible, sound1,
		/* [3] */
		OK, visible, sound1,
		/* [4] */
		OK, visible, sound1
	},
	alertPositionMainScreen
};

resource 'DITL' (128, purgeable) {
	{	/* array DITLarray: 2 elements */
		/* [1] */
		{80, 290, 100, 358},
		Button {
			enabled,
			"OK"
		},
		/* [2] */
		{16, 64, 66, 354},
		StaticText {
			disabled,
			"Hello World, this is Retro68!"
		}
	}
};
