#pragma once
#include <QObject>
#include <QWidget>

class PluginInterface {
public:
    virtual ~PluginInterface() = default;
    
    virtual QString name() const = 0;
    virtual QString description() const = 0;
    virtual QWidget* createWidget(QWidget* parent) = 0;
    virtual bool isEnabled() const = 0;
    
signals:
    virtual void operationStarted(const QString& message) = 0;
    virtual void errorOccurred(const QString& message) = 0;
    virtual void layerCreated(const QString& layerName) = 0;
    virtual void layerRemoved(const QString& layerName) = 0;
    virtual void progressChanged(int percent) = 0;
};

#define PluginInterface_iid "com.flatimage.PluginInterface"
Q_DECLARE_INTERFACE(PluginInterface, PluginInterface_iid)
