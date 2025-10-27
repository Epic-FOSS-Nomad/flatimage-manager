#pragma once

#include "../../api/PluginInterface.h"
#include <QObject>

class PackageConverter : public PluginInterface {
    Q_OBJECT
    Q_INTERFACES(PluginInterface)
public:
    explicit PackageConverter(QObject* parent = nullptr);
    
    QString name() const override { return "Package Converter"; }
    QString description() const override { return "Convert Linux packages to FlatImages"; }
    QWidget* createWidget(QWidget* parent) override;
    bool isEnabled() const override { return true; }
    
    bool convertPackage(const QString& packagePath, const QString& outputPath);
    
private:
    bool convertDebPackage(const QString& debPath, const QString& outputPath);
    bool convertRpmPackage(const QString& rpmPath, const QString& outputPath);
};
