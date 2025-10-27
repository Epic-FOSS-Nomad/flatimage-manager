#include "MainWindow.h"
#include "theming/ThemeEngine.h"
#include <QTabWidget>
#include <QDir>
#include <QPluginLoader>
#include <QMenu>
#include <QAction>
#include <QDialog>
#include <QVBoxLayout>
#include <QListWidget>
#include <QListWidgetItem>
#include <QStatusBar>
#include <QProgressBar>
#include <QMessageBox>

MainWindow::MainWindow(QWidget* parent) 
    : QMainWindow(parent), m_tabWidget(new QTabWidget(this)), m_layerMenu(nullptr), m_statusBar(nullptr), m_progressBar(nullptr) {
    setCentralWidget(m_tabWidget);
    setWindowTitle("FlatImage Manager");
    m_appIcon = QIcon(":/app-icon.svg");
    setWindowIcon(m_appIcon);
    resize(1024, 768);  // Optimal starting size
    
    // Apply theme
    ThemeEngine themeEngine;
    themeEngine.applyTheme(ThemeEngine::ModernDark);
    
    // Load plugins
    loadPlugins();
    
    setupUi();
    
    // Setup progress bar
    m_progressBar = new QProgressBar(this);
    m_statusBar = new QStatusBar(this);
    m_statusBar->addPermanentWidget(m_progressBar);
    setStatusBar(m_statusBar);
    m_progressBar->setVisible(false);
    
    statusBar()->showMessage("FlatImage Manager v" + QString(FLATIMAGE_MANAGER_VERSION));
}

void MainWindow::loadPlugins() {
    QDir pluginsDir(qApp->applicationDirPath() + "/plugins");
    for (auto fileName : pluginsDir.entryList(QDir::Files)) {
        QPluginLoader loader(pluginsDir.absoluteFilePath(fileName));
        if (auto plugin = qobject_cast<class PluginInterface*>(loader.instance())) {
            if (plugin->isEnabled()) {
                auto widget = plugin->createWidget(this);
                m_tabWidget->addTab(widget, plugin->name());
                
                connect(plugin, &PluginInterface::layerCreated,
                        this, [this](const QString& layerName) {
                    statusBar()->showMessage("Layer created: " + layerName, 5000);
                });
                
                connect(plugin, &PluginInterface::progressChanged,
                        this, [this](int percent) {
                    m_progressBar->setVisible(percent > 0 && percent < 100);
                    m_progressBar->setValue(percent);
                });
                
                connect(plugin, &PluginInterface::errorOccurred,
                        this, [this](const QString& msg) {
                    QMessageBox::critical(this, "Plugin Error", msg);
                });
            }
        } else {
            qWarning() << "Failed to load plugin:" << loader.errorString();
        }
    }
}

void MainWindow::setupUi() {
    // Add layer menu
    m_layerMenu = new QMenu("Layers", this);
    menuBar()->addMenu(m_layerMenu);
    
    auto layerAction = new QAction("Manage Layers", this);
    connect(layerAction, &QAction::triggered, this, &MainWindow::showLayerManager);
    m_layerMenu->addAction(layerAction);
    
    auto aboutAction = new QAction("About", this);
    connect(aboutAction, &QAction::triggered, this, &MainWindow::showAboutDialog);
    menuBar()->addAction(aboutAction);
}

void MainWindow::showLayerManager() {
    auto dialog = new QDialog(this);
    auto layout = new QVBoxLayout(dialog);
    
    auto listWidget = new QListWidget(dialog);
    auto builder = ImageBuilder();
    auto layers = builder.getUserLayers();
    
    for (const auto& layer : layers) {
        auto item = new QListWidgetItem(
            QString("%1: %2").arg(layer.username, layer.layerPath), 
            listWidget
        );
        item->setCheckState(layer.isActive ? Qt::Checked : Qt::Unchecked);
    }
    
    layout->addWidget(listWidget);
    dialog->exec();
}

void MainWindow::showAboutDialog() {
    QString aboutText = QString(
        "FlatImage Manager v%1\n"
        "Commit: %2\n"
        "Built: %3"
    ).arg(FLATIMAGE_MANAGER_VERSION, 
          FLATIMAGE_MANAGER_GIT_COMMIT,
          FLATIMAGE_MANAGER_BUILD_DATE);
    
    QMessageBox::about(this, "About", aboutText);
}

MainWindow::~MainWindow() {
    #ifdef Q_OS_LINUX
    ImageBuilder builder;
    builder.unmountAll();
    #endif
}
