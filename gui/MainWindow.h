#pragma once

#include <QMainWindow>
#include <QTabWidget>
#include <QMenu>
#include <QIcon>

class MainWindow : public QMainWindow {
    Q_OBJECT
public:
    explicit MainWindow(QWidget* parent = nullptr);
    
private slots:
    void showLayerManager();
    void showAboutDialog();
    
private:
    void setupUi();
    void loadPlugins();
    
    QTabWidget* m_tabWidget;
    QMenu* m_layerMenu;
    QIcon m_appIcon;
};
