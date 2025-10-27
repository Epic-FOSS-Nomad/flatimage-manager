#include "PackageConverter.h"
#include <QFileInfo>
#include <QDebug>
#include <QPushButton>
#include <QVBoxLayout>
#include <QFileDialog>

PackageConverter::PackageConverter(QObject* parent) 
    : PluginInterface(parent) {}

QWidget* PackageConverter::createWidget(QWidget* parent) {
    auto widget = new QWidget(parent);
    auto layout = new QVBoxLayout(widget);
    
    auto convertBtn = new QPushButton("Convert Package", widget);
    connect(convertBtn, &QPushButton::clicked, [this] {
        auto path = QFileDialog::getOpenFileName(nullptr, "Select Package");
        if (!path.isEmpty()) convertPackage(path, path + ".flatimage");
    });
    
    layout->addWidget(convertBtn);
    return widget;
}

bool PackageConverter::convertPackage(const QString& packagePath, 
                                      const QString& outputPath) {
    if (!QFile::exists(packagePath)) {
        emit errorOccurred("Package file not found: " + packagePath);
        return false;
    }
    
    QFileInfo fileInfo(packagePath);
    QString suffix = fileInfo.suffix().toLower();
    
    bool success = false;
    
    if (suffix == "deb") {
        success = convertDebPackage(packagePath, outputPath);
    } else if (suffix == "rpm") {
        success = convertRpmPackage(packagePath, outputPath);
    } else {
        emit errorOccurred("Unsupported package format: " + suffix);
        return false;
    }
    
    if (success) {
        emit layerCreated(QFileInfo(outputPath).fileName());
    }
    
    return success;
}

bool PackageConverter::convertDebPackage(const QString& debPath, 
                                         const QString& outputPath) {
    // Extract deb package
    QProcess dpkg;
    dpkg.start("dpkg-deb", {"-x", debPath, "/tmp/extract"});
    dpkg.waitForFinished();
    
    if (dpkg.exitCode() != 0) {
        emit errorOccurred("DEB extraction failed");
        return false;
    }
    
    // Create flatimage from extracted contents
    emit operationStarted("Creating FlatImage from DEB package");
    // Actual image creation would call ImageBuilder here
    return true;
}
