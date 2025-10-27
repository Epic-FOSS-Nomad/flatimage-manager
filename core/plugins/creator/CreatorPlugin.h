#pragma once

#include "../../api/PluginInterface.h"
#include <QObject>
#include "../../lib/fsimaging/BaseImageManager.h"

class CreatorPlugin : public PluginInterface {
    Q_OBJECT
    Q_INTERFACES(PluginInterface)
public:
    explicit CreatorPlugin(QObject* parent = nullptr);
    
    QString name() const override { return "Image Creator"; }
    QString description() const override { return "Create FlatImages from folders or scripts"; }
    QWidget* createWidget(QWidget* parent) override;
    bool isEnabled() const override { return true; }
    
public slots:
    void startConversion();
    
private:
    QString m_sourcePath;
    bool m_emulationEnabled = true;
    QString m_baseImagePath;  // Store selected base image path
};
