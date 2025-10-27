#pragma once

#include <QObject>
#include <QApplication>

class ThemeEngine : public QObject {
    Q_OBJECT
public:
    enum ThemeStyle {
        ModernDark,
        TraditionalLight,
        Custom
    };
    
    explicit ThemeEngine(QObject* parent = nullptr);
    
    void applyTheme(ThemeStyle style);
    
private:
    void applyModernDarkTheme();
    void applyTraditionalLightTheme();
    void applyCustomTheme();
    void adaptToDesktopEnvironment();
};
