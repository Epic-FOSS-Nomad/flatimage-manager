#!/bin/bash
set -e  # Exit on error

# Build the project
mkdir build
cd build
cmake .. -DCMAKE_PREFIX_PATH=/path/to/qt6 -DCMAKE_BUILD_TYPE=Release
cmake --build .

# Create package directory
mkdir -p flatimage-manager-package/usr/bin
mkdir -p flatimage-manager-package/usr/lib/flatimage-manager/plugins
mkdir -p flatimage-manager-package/usr/share/flatimage-manager/resources
mkdir -p flatimage-manager-package/usr/share/icons/hicolor/scalable/apps/

# Copy binaries
cp FlatImageManager flatimage-manager-package/usr/bin/
cp ../lib/*.so flatimage-manager-package/usr/lib/flatimage-manager/plugins/
cp -r ../resources/* flatimage-manager-package/usr/share/flatimage-manager/resources/
cp ../resources/icons/* flatimage-manager-package/usr/share/icons/hicolor/scalable/apps/

# Create desktop entry
cat > flatimage-manager-package/usr/share/applications/flatimage-manager.desktop <<EOF
[Desktop Entry]
Name=FlatImage Manager
Comment=Manage FlatImage packages
Exec=/usr/bin/FlatImageManager
Icon=flatimage-manager
Terminal=false
Type=Application
Categories=Utility;
EOF

BUILD_DATE=$(date +%Y-%m-%dT%H:%M:%S%z)
VERSION=$(date +%Y.%m.%d)
sed -i "s/^Version: .*/Version: $VERSION/" flatimage-manager-package/DEBIAN/control

# Check dependencies
if ! command -v flatimage-cli &> /dev/null; then
    echo "Error: flatimage-cli not found. Install it first."
    exit 1
fi

# Create the FlatImage
flatimage-cli create flatimage-manager-package -o FlatImageManager.flatimage

# Cleanup
echo "Cleaning up..."
rm -rf flatimage-manager-package

echo "Packaging complete: FlatImageManager.flatimage"
