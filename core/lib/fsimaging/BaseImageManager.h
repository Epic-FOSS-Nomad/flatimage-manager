#pragma once
#include <QObject>
#include <QMap>
#include <QString>
#include <QIcon>
#include <QVariantMap>

// Manages available base images for FlatImage creation
class BaseImageManager : public QObject {
    Q_OBJECT
public:
    // Base image types (Ruan's and others)
    enum BaseImageType {
        ALPINE,      // Alpine Linux base
        ARCH,        // Arch Linux base
        BLUEPRINT,   // Ruan's blueprint base
        CUSTOM       // User-defined custom base
    };
    
    explicit BaseImageManager(QObject* parent = nullptr);
    
    // Get path to a base image by type
    QString getBaseImagePath(BaseImageType type) const;
    
    // Get all available base images with metadata
    QMap<QString, QVariantMap> availableBaseImages() const;
    
    // Get icon for a base image
    QIcon getBaseImageIcon(const QString& imageName) const;
    
private:
    // Load base images from standard locations
    void loadBaseImages();
    
    // Map of image names to metadata
    QMap<QString, QVariantMap> m_baseImages;
};
