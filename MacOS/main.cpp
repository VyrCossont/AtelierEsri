#include <Dialogs.h>
#include <Fonts.h>
#include <MacWindows.h>
#include <Menus.h>
#include <QuickDraw.h>
#include <TextEdit.h>

const SInt16 kAlertID = 128;

class App
{
public:
    App();
    void Run();

private:
    void Initialize();
};

App::App()
{
    Initialize();
}

void App::Initialize()
{
#if !TARGET_API_MAC_CARBON
    InitGraf(&qd.thePort);
    InitFonts();
    InitWindows();
    InitMenus();
    TEInit();
    InitDialogs(nil);
#endif
    InitCursor();
}

void App::Run()
{
    NoteAlert(kAlertID, nullptr);
}

int main()
{
    App app;

    app.Run();

    return 0;
}
