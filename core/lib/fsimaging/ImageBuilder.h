#pragma once

#include <QObject>
#include <QString>
#include <QVector>
#include <QDir>

class ImageBuilder : public QObject {
    Q_OBJECT
public:
    enum LayerType {
        TEMPORARY,
        PERMANENT,
        MULTI_USER
    };
    
    explicit ImageBuilder(QObject* parent = nullptr);
    
    void createImage(const QString& sourcePath, 
                     const QString& outputPath,
                     bool enableEmulation,
                     LayerType layerType,
                     const QString& baseImagePath = "");
    
signals:
    void imageCreated(const QString& path);
    void progressChanged(int percent);
    void errorOccurred(const QString& message);
    
    // Layer management
    void layerCreated(const QString& layerName);
    void layerRemoved(const QString& layerName);
    
private:
    void addTemporaryLayer(const QString& imagePath);
    void addPersistentLayer(const QString& imagePath);
    void addMultiUserLayers(const QString& imagePath);
    void mountOverlay(const QString& baseImage,
                      const QString& lowerDir,
                      const QString& upperDir,
                      const QString& workDir);
    void unmountAll();
    
    struct UserLayer {
        QString username;
        QString layerPath;
        bool isActive;
    };
    
    QVector<UserLayer> getUserLayers() const;
};
