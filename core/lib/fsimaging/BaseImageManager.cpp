#include "BaseImageManager.h"
#include <QDir>
#include <QStandardPaths>
#include <QFileInfo>
#include <QIcon>

BaseImageManager::BaseImageManager(QObject* parent) 
    : QObject(parent) {
    loadBaseImages();
}

// Load base images from system and user directories
void BaseImageManager::loadBaseImages() {
    // System-wide base images directory
    QString systemBasePath = QStandardPaths::writableLocation(QStandardPaths::AppDataLocation) 
                          + "/base-images/";
    QDir systemDir(systemBasePath);
    systemDir.mkpath(".");  // Create if doesn't exist
    
    // User-specific base images
    QString userBasePath = QDir::homePath() + "/.flatimage/base-images/";
    QDir userDir(userBasePath);
    userDir.mkpath(".");
    
    // Add Ruan's predefined base images
    m_baseImages["Alpine"] = {
        {"path", systemDir.filePath("alpine.flatimage")},
        {"type", ALPINE},
        {"description", "Lightweight Alpine Linux base image"},
        {"author", "Ruan Formigoni"}
    };
    
    m_baseImages["Arch"] = {
        {"path", systemDir.filePath("arch.flatimage")},
        {"type", ARCH},
        {"description", "Arch Linux base image with rolling updates"},
        {"author", "Ruan Formigoni"}
    };
    
    m_baseImages["Blueprint"] = {
        {"path", systemDir.filePath("blueprint.flatimage")},
        {"type", BLUEPRINT},
        {"description", "Ruan's specialized game development base image"},
        {"author", "Ruan Formigoni"}
    };
    
    // Add user-defined base images
    for (const QString& file : userDir.entryList({"*.flatimage"}, QDir::Files)) {
        QString name = QFileInfo(file).baseName();
        m_baseImages[name] = {
            {"path", userDir.filePath(file)},
            {"type", CUSTOM},
            {"description", "User-defined custom base image"},
            {"author", "User"}
        };
    }
}

// Get path for a specific base image type
QString BaseImageManager::getBaseImagePath(BaseImageType type) const {
    for (const auto& image : m_baseImages) {
        if (image["type"].toInt() == type) {
            return image["path"].toString();
        }
    }
    return "";
}

// Get all available base images with metadata
QMap<QString, QVariantMap> BaseImageManager::availableBaseImages() const {
    return m_baseImages;
}

// Get icon for base image
QIcon BaseImageManager::getBaseImageIcon(const QString& imageName) const {
    if (imageName == "Alpine") return QIcon(":/icons/alpine.png");
    if (imageName == "Arch") return QIcon(":/icons/arch.png");
    if (imageName == "Blueprint") return QIcon(":/icons/blueprint.png");
    return QIcon(":/icons/custom.png");
}
