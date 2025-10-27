#include "MainWindow.h"
#include <QApplication>

int main(int argc, char *argv[]) {
    QApplication app(argc, argv);
    
    // Ensure cleanup on exit
    auto cleanup = [] {
        ImageBuilder builder;
        builder.unmountAll();
    };
    QObject::connect(&app, &QApplication::aboutToQuit, cleanup);
    
    MainWindow mainWindow;
    mainWindow.show();
    
    return app.exec();
}
