/** Manually managed resources (as opposed to game assets). */
/* coding: macintosh */

#include "Dialogs.r"
#include "MacWindows.r"
#include "Menus.r"

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
			"Fatal error: ^0\rLocation: ^1"
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

/*
 * Menus are called out as specifically not purgeable here:
 * https://preterhuman.net/macstuff/insidemac/MoreToolbox/MoreToolbox-13.html#HEADING13-0
 */

resource 'MBAR' (menuBarMBARResourceID, "menu bar", preload) {
	{	/* array MenuArray: 2 elements */
		/* [1] */
		appleMenuMENUResourceID,
		/* [2] */
		fileMenuMENUResourceID
	}
};

resource 'MENU' (appleMenuMENUResourceID, "Apple menu", preload) {
	appleMenuMENUResourceID,
	textMenuProc,
	allEnabled,
	enabled,
	apple,
	{	/* array: 1 elements */
		/* [1] */
		"About Atelier Esri…", noIcon, noKey, noMark, plain
	}
};

resource 'MENU' (fileMenuMENUResourceID, "file menu", preload) {
	fileMenuMENUResourceID,
	textMenuProc,
	0x7FFFFFFB,
	enabled,
	"File",
	{	/* array: 4 elements */
		/* [1] */
		"Open…", noIcon, "O", noMark, plain,
		/* [2] */
		"Save", noIcon, "S", noMark, plain,
		/* [3] */
		"-", noIcon, noKey, noMark, plain,
		/* [4] */
		"Quit", noIcon, "Q", noMark, plain
	}
};
