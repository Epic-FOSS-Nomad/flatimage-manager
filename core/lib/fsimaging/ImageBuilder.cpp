#include "ImageBuilder.h"
#include <QProcess>
#include <QDir>
#include <sys/mount.h>

ImageBuilder::ImageBuilder(QObject* parent) : QObject(parent) {}

void ImageBuilder::createImage(const QString& sourcePath, 
                               const QString& outputPath,
                               bool enableEmulation,
                               LayerType layerType,
                               const QString& baseImagePath) {
    qInfo() << "Creating FlatImage from" << sourcePath;
    try {
        // Create base image
        QProcess createProcess;
        QStringList createArgs;
        createArgs << "create" << sourcePath << "-o" << outputPath;
        if (enableEmulation) createArgs << "--emulate";
        
        // Add base image if specified
        if (!baseImagePath.isEmpty()) {
            createArgs << "--base" << baseImagePath;
        }
        
        createProcess.start("flatimage-cli", createArgs);
        createProcess.waitForFinished();
        
        if (createProcess.exitCode() != 0) {
            emit errorOccurred("Image creation failed: " + createProcess.errorString());
            return;
        }
        
        // Setup layers
        switch(layerType) {
            case TEMPORARY:
                addTemporaryLayer(outputPath);
                break;
            case PERMANENT:
                addPersistentLayer(outputPath);
                break;
            case MULTI_USER:
                addMultiUserLayers(outputPath);
                break;
        }
        
        emit imageCreated(outputPath);
    } catch (const std::exception& e) {
        qCritical() << "Image creation failed:" << e.what();
        emit errorOccurred(e.what());
    }
}

void ImageBuilder::addTemporaryLayer(const QString& imagePath) {
    QDir tempDir(QDir::tempPath() + "/flatimage-temp");
    tempDir.mkpath(".");
    
    // Mount with OverlayFS
    mountOverlay(imagePath, 
                 tempDir.filePath("lower"),
                 tempDir.filePath("upper"),
                 tempDir.filePath("work"));
}

void ImageBuilder::addPersistentLayer(const QString& imagePath) {
    QDir persistDir(QDir::homePath() + "/.flatimage/persistent");
    persistDir.mkpath(".");
    
    QString layerName = QFileInfo(imagePath).baseName() + "_persist";
    QString layerPath = persistDir.filePath(layerName);
    
    mountOverlay(imagePath, 
                 persistDir.filePath("lower"),
                 layerPath,
                 persistDir.filePath("work"));
}

void ImageBuilder::addMultiUserLayers(const QString& imagePath) {
    QDir baseDir(QDir::homePath() + "/.flatimage/layers");
    baseDir.mkpath(".");
    
    QString username = qgetenv("USER");
    if (username.isEmpty()) username = qgetenv("USERNAME");
    
    QString userLayer = baseDir.filePath(username + "_layer");
    QDir(userLayer).mkpath(".");
    
    mountOverlay(imagePath, 
                 baseDir.filePath("lower"),
                 userLayer,
                 baseDir.filePath("work_" + username));
}

QVector<ImageBuilder::UserLayer> ImageBuilder::getUserLayers() const {
    QVector<UserLayer> layers;
    QDir layerDir(QDir::homePath() + "/.flatimage/layers");
    
    for (const auto& dir : layerDir.entryList(QDir::Dirs | QDir::NoDotAndDotDot)) {
        if (dir.startsWith("work_")) continue;
        
        UserLayer layer;
        layer.username = dir.replace("_layer", "");
        layer.layerPath = layerDir.filePath(dir);
        layer.isActive = (qgetenv("USER") == layer.username);
        layers.append(layer);
    }
    
    return layers;
}

void ImageBuilder::mountOverlay(const QString& baseImage,
                                const QString& lowerDir,
                                const QString& upperDir,
                                const QString& workDir) {
    #ifdef Q_OS_LINUX
    // Create directories
    QDir().mkpath(lowerDir);
    QDir().mkpath(upperDir);
    QDir().mkpath(workDir);
    
    // Mount base image to lower directory
    QProcess mountProc;
    mountProc.start("mount", {"--bind", baseImage, lowerDir});
    mountProc.waitForFinished();
    
    if (mountProc.exitCode() != 0) {
        emit errorOccurred("Mount failed: " + mountProc.errorString());
        return;
    }
    
    // Mount OverlayFS
    QString options = "lowerdir=" + lowerDir + ",upperdir=" + upperDir + ",workdir=" + workDir;
    mountProc.start("mount", {"-t", "overlay", "overlay", "-o", options, baseImage});
    mountProc.waitForFinished();
    
    if (mountProc.exitCode() != 0) {
        emit errorOccurred("OverlayFS mount failed: " + mountProc.errorString());
    }
    #else
    emit errorOccurred("Layered filesystem is only supported on Linux");
    #endif
}

void ImageBuilder::unmountAll() {
    #ifdef Q_OS_LINUX
    QProcess proc;
    proc.start("umount", {"-l", "-R", "/mnt/flatimage"});
    proc.waitForFinished();
    #endif
}
