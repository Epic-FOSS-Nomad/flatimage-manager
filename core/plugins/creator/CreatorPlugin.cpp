#include "CreatorPlugin.h"
#include <QPushButton>
#include <QCheckBox>
#include <QFormLayout>
#include <QFileDialog>
#include "../../lib/fsimaging/BaseImageManager.h"

CreatorPlugin::CreatorPlugin(QObject* parent) 
    : PluginInterface(parent) {}

QWidget* CreatorPlugin::createWidget(QWidget* parent) {
    auto widget = new QWidget(parent);
    auto layout = new QFormLayout(widget);
    
    // Source selection
    auto sourceBtn = new QPushButton("Browse...", widget);
    connect(sourceBtn, &QPushButton::clicked, [this] {
        auto path = QFileDialog::getExistingDirectory(nullptr, "Select Source");
        if (!path.isEmpty()) m_sourcePath = path;
    });
    layout->addRow("Source Directory:", sourceBtn);
    
    // Base image selection
    auto baseLabel = new QLabel("Base Image:", widget);
    auto baseCombo = new QComboBox(widget);
    BaseImageManager baseManager;
    
    // Add "None" option
    baseCombo->addItem("None - Create from scratch", "");
    
    // Add available base images
    for (const auto& name : baseManager.availableBaseImages().keys()) {
        baseCombo->addItem(
            baseManager.getBaseImageIcon(name),
            name,
            baseManager.availableBaseImages()[name]["path"]
        );
    }
    
    // Connect selection change
    connect(baseCombo, QOverload<int>::of(&QComboBox::currentIndexChanged),
        [this, baseCombo](int index) {
            m_baseImagePath = baseCombo->itemData(index).toString();
        });
    
    auto baseLayout = new QHBoxLayout();
    baseLayout->addWidget(baseLabel);
    baseLayout->addWidget(baseCombo);
    layout->addRow(baseLayout);
    
    // Emulation toggle
    auto emulationToggle = new QCheckBox("Enable Emulation", widget);
    emulationToggle->setChecked(true);
    connect(emulationToggle, &QCheckBox::toggled, 
        [this](bool checked) { m_emulationEnabled = checked; });
    layout->addRow(emulationToggle);
    
    // Conversion button
    auto convertBtn = new QPushButton("Create FlatImage", widget);
    connect(convertBtn, &QPushButton::clicked, this, &CreatorPlugin::startConversion);
    layout->addRow(convertBtn);
    
    return widget;
}

void CreatorPlugin::startConversion() {
    if (m_sourcePath.isEmpty()) {
        emit errorOccurred("No source directory selected");
        return;
    }
    
    emit operationStarted(tr("Creating FlatImage from %1").arg(m_sourcePath));
    
    // Build conversion command
    QStringList args;
    args << "create" << m_sourcePath;
    if (m_emulationEnabled) args << "--emulate";
    
    // Add base image if selected
    if (!m_baseImagePath.isEmpty()) {
        args << "--base" << m_baseImagePath;
    }
    
    // Execute via core imaging service
    emit executeCommand("flatimage-cli", args);
    
    // Simulate progress
    for (int i = 0; i <= 100; i += 10) {
        QThread::msleep(100);
        emit progressChanged(i);
    }
    
    // After successful creation
    emit layerCreated(QFileInfo(m_sourcePath).fileName());
}
