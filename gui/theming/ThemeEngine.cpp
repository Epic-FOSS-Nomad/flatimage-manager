#include "ThemeEngine.h"
#include <QApplication>
#include <QStyleFactory>
#include <QFile>
#include <QSettings>

ThemeEngine::ThemeEngine(QObject* parent) : QObject(parent) {}

void ThemeEngine::applyTheme(ThemeStyle style) {
    switch(style) {
        case ModernDark:
            applyModernDarkTheme();
            break;
        case TraditionalLight:
            applyTraditionalLightTheme();
            break;
        case Custom:
            applyCustomTheme();
            break;
    }
    adaptToDesktopEnvironment();
    
    // Fallback if stylesheet loading fails
    if (qApp->styleSheet().isEmpty()) {
        qApp->setStyle(QStyleFactory::create("Fusion"));
    }
}

void ThemeEngine::applyModernDarkTheme() {
    QFile styleFile(":/styles/modern-dark.qss");
    if (styleFile.open(QIODevice::ReadOnly)) {
        QString styleSheet = styleFile.readAll();
        qApp->setStyleSheet(styleSheet);
    }
    qApp->setStyle(QStyleFactory::create("Fusion"));
    
    QPalette darkPalette;
    darkPalette.setColor(QPalette::Window, QColor(53,53,53));
    darkPalette.setColor(QPalette::WindowText, Qt::white);
    // ... additional palette configuration
    qApp->setPalette(darkPalette);
}

void ThemeEngine::adaptToDesktopEnvironment() {
    QString desktop = qgetenv("XDG_CURRENT_DESKTOP").toLower();
    
    if (desktop.contains("gnome")) {
        // GNOME-specific adjustments
    } else if (desktop.contains("kde")) {
        // KDE-specific adjustments
    } else if (desktop.contains("xfce")) {
        // XFCE-specific adjustments
        setWindowRounding(4);
        setAnimationDuration(150);
    } else {
        // Default adjustments
    }
}
