/** Manually managed resources (as opposed to game assets). */

#include "Dialogs.r"
#include "MacWindows.r"

#include "AppResources.h"

resource 'ALRT' (errorALRTResourceID, "exception alert", purgeable) {
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

resource 'DITL' (errorDITLResourceID, "exception alert", purgeable) {
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
			"Fatal exception: ^0\rLocation: ^1"
		}
	}
};

resource 'WIND' (gameWINDResourceID, "game window", purgeable) {
    {43, 6, 203, 166},
    noGrowDocProc,
    visible,
    noGoAway,
    0x0, /* refCon */
    "Atelier Esri",
    noAutoCenter
};
