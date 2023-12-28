/** Manually managed resources (as opposed to game assets). */
/* coding: macintosh */

#include "Controls.r"
#include "Dialogs.r"
#include "Icons.r"
#include "MacWindows.r"
#include "Menus.r"
#include "Processes.r"

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

resource 'WIND' (synthesisWINDResourceID, "synthesis window", preload, purgeable) {
    {43, 6, 203, 166},
    zoomDocProc,
    visible,
    goAway,
    0x0, /* refCon */
    "Synthesis",
    noAutoCenter
};

resource 'CNTL' (synthesisHScrollBarCNTLResourceID, "synthesis window horizontal scroll bar", preload, purgeable) {
    {145, -1, 161, 146},
    0, /* initial setting */
    visible,
    0, /* max setting */
    0, /* min setting */
    scrollBarProc,
    0x0, /* refCon */
    "" /* title */
};

resource 'CNTL' (synthesisVScrollBarCNTLResourceID, "synthesis window vertical scroll bar", preload, purgeable) {
    {-1, 145, 146, 161},
    0, /* initial setting */
    visible,
    0, /* max setting */
    0, /* min setting */
    scrollBarProc,
    0x0, /* refCon */
    "" /* title */
};

resource 'CNTL' (synthesisCompleteButtonCNTLResourceID, "synthesis complete button", preload, purgeable) {
    {13, 88, 33, 147},
    0, /* initial setting */
    visible,
    1, /* max setting */
    0, /* min setting */
    pushButProc,
    0x0, /* refCon */
    "Complete" /* title */
};

resource 'CNTL' (synthesisCancelButtonCNTLResourceID, "synthesis cancel button", preload, purgeable) {
    {46, 88, 66, 147},
    0, /* initial setting */
    visible,
    1, /* max setting */
    0, /* min setting */
    pushButProc,
    0x0, /* refCon */
    "Cancel" /* title */
};

resource 'CNTL' (synthesisUndoButtonCNTLResourceID, "synthesis undo button", preload, purgeable) {
    {46, 88, 66, 147},
    0, /* initial setting */
    invisible,
    1, /* max setting */
    0, /* min setting */
    pushButProc,
    0x0, /* refCon */
    "Undo" /* title */
};

resource 'WIND' (inventoryWINDResourceID, "inventory window", preload, purgeable) {
    {43, 206, 203, 366},
    zoomDocProc,
    visible,
    goAway,
    0x0, /* refCon */
    "Container",
    noAutoCenter
};

resource 'CNTL' (inventoryVScrollBarCNTLResourceID, "inventory window vertical scroll bar", preload, purgeable) {
    {-1, 145, 146, 161},
    0, /* initial setting */
    visible,
    0, /* max setting */
    0, /* min setting */
    scrollBarProc,
    0x0, /* refCon */
    "" /* title */
};

resource 'WIND' (titleScreenWINDResourceID, "title screen window", preload, purgeable) {
    {120, 240, 280, 400},
    altDBoxProc,
    visible,
    noGoAway,
    0x0, /* refCon */
    "", /* title */
    centerMainScreen
};

resource 'WIND' (atelierInteriorWINDResourceID, "atelier interior window", preload, purgeable) {
    {43, 6, 343, 406},
    noGrowDocProc,
    visible,
    noGoAway,
    0x0, /* refCon */
    "Atelier Esri", /* title */
    centerMainScreen
};

resource 'CNTL' (atelierInteriorSynthesizeButtonCNTLResourceID, "atelier interior synthesize button", preload, purgeable) {
    {197, 165, 217, 245},
    0, /* initial setting */
    visible,
    1, /* max setting */
    0, /* min setting */
    pushButProc,
    0x0, /* refCon */
    "Synthesize" /* title */
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

/* Require 10 MB of heap for now. Main sprite sheet currently uses a silly amount of memory. */
resource 'SIZE' (-1) {
	reserved,
	ignoreSuspendResumeEvents,
	reserved,
	cannotBackground,
	notMultiFinderAware,
	backgroundAndForeground,
	dontGetFrontClicks,
	ignoreChildDiedEvents,
	is32BitCompatible,
	notHighLevelEventAware,
	onlyLocalHLEvents,
	notStationeryAware,
	dontUseTextEditServices,
	notDisplayManagerAware,
	reserved,
	reserved,
	10485760, /* 10 MB */
	10485760  /* 10 MB */
};

resource 'ICON' (666) {
	$"0000 0000 0000 0000 0000 0000 0000 0000"
	$"0000 0000 0000 0000 0000 0000 0003 F000"
	$"0004 0800 0FFB F7FE 100A 1403 200E 1C05"
	$"4000 0009 7FFF FFF1 8000 0011 8000 0011"
	$"8000 0011 8000 0011 8403 0711 9F06 8C91"
	$"B486 0C11 B406 0C11 9F0F 1E11 8586 0C11"
	$"A586 0C11 9F16 0C51 840C 1F91 8000 0011"
	$"8000 0012 8000 0014 8000 0018 7FFF FFF0"
};

resource 'DITL' (666, purgeable) {
	{	/* array DITLarray: 5 elements */
		/* [1] */
		{83, 228, 103, 287},
		Button {
			enabled,
			"Buy"
		},
		/* [2] */
		{83, 156, 103, 215},
		Button {
			enabled,
			"Cancel"
		},
		/* [3] */
		{35, 78, 70, 287},
		StaticText {
			disabled,
			"Do you want to buy 500 Gems for $4.99?"
		},
		/* [4] */
		{13, 23, 45, 55},
		Icon {
			disabled,
			666
		},
		/* [5] */
		{13, 78, 28, 287},
		StaticText {
			disabled,
			"Confirm Your In-App Purchase"
		}
	}
};

resource 'ALRT' (666, purgeable) {
	{40, 40, 156, 340},
	666,
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

resource 'CNTL' (666, preload, purgeable) {
    {234, 23, 254, 143},
    0, /* initial setting */
    visible,
    1, /* max setting */
    0, /* min setting */
    pushButProc,
    0x0, /* refCon */
    "Purchase Gems" /* title */
};

resource 'CNTL' (667, preload, purgeable) {
    {267, 23, 287, 143},
    0, /* initial setting */
    visible,
    1, /* max setting */
    0, /* min setting */
    pushButProc,
    0x0, /* refCon */
    "Restore Purchases" /* title */
};