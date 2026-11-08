#!/usr/bin/env bash
set -euo pipefail

APP_NAME="granco-planner"
PRODUCT_NAME="Granco Planner"
VERSION="1.0.0"
ARCH_DEB="amd64"
ARCH_RPM="x86_64"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BIN_PATH="${ROOT_DIR}/target/release/${APP_NAME}"
RELEASE_DIR="${ROOT_DIR}/release"

if [[ ! -f "${BIN_PATH}" ]]; then
    echo "Building release binary..."
    (cd "${ROOT_DIR}" && cargo build --release)
fi

mkdir -p "${RELEASE_DIR}"

# ---------------------------------------------------------
# 1. Build Debian / Ubuntu (.deb) Installer
# ---------------------------------------------------------
DEB_STAGE="$(mktemp -d)"
chmod 0755 "${DEB_STAGE}"
mkdir -p "${DEB_STAGE}/DEBIAN"
chmod 0755 "${DEB_STAGE}/DEBIAN"
mkdir -p "${DEB_STAGE}/usr/bin"
mkdir -p "${DEB_STAGE}/usr/share/applications"
mkdir -p "${DEB_STAGE}/usr/share/icons/hicolor/512x512/apps"
chmod -R 0755 "${DEB_STAGE}/usr"

cp "${BIN_PATH}" "${DEB_STAGE}/usr/bin/${APP_NAME}"
chmod 0755 "${DEB_STAGE}/usr/bin/${APP_NAME}"
cp "${ROOT_DIR}/packaging/linux/granco-planner.desktop" "${DEB_STAGE}/usr/share/applications/${APP_NAME}.desktop"
cp "${ROOT_DIR}/assets/icon.png" "${DEB_STAGE}/usr/share/icons/hicolor/512x512/apps/${APP_NAME}.png"

INSTALLED_SIZE=$(du -sk "${DEB_STAGE}/usr" | cut -f1)

cat > "${DEB_STAGE}/DEBIAN/control" <<EOF
Package: ${APP_NAME}
Version: ${VERSION}
Section: sound
Priority: optional
Architecture: ${ARCH_DEB}
Installed-Size: ${INSTALLED_SIZE}
Depends: libc6, libgl1, libx11-6, libxcb1, libxkbcommon0
Maintainer: Oscar Coello <oacoello@users.noreply.github.com>
Homepage: https://github.com/oacoello/worship-planner
Description: Granco Planner - Native Rust Worship Service & Setlist Planner
 Plan worship setlists, sermon series, musical keys, multi-arrangements,
 technical run sheets, and song rotation analytics.
EOF

DEB_OUT="${RELEASE_DIR}/${APP_NAME}_${VERSION}_${ARCH_DEB}.deb"
dpkg-deb --root-owner-group --build "${DEB_STAGE}" "${DEB_OUT}"
rm -rf "${DEB_STAGE}"
echo "Built Debian installer: ${DEB_OUT}"

# ---------------------------------------------------------
# 2. Build RPM (.rpm) Installer (if rpmbuild is available)
# ---------------------------------------------------------
if command -v rpmbuild >/dev/null 2>&1; then
    RPM_TOP="$(mktemp -d)"
    mkdir -p "${RPM_TOP}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS,BUILDROOT}

    cat > "${RPM_TOP}/SPECS/${APP_NAME}.spec" <<EOF
Name:           ${APP_NAME}
Version:        ${VERSION}
Release:        1
Summary:        Granco Planner - Native Rust Worship Service & Setlist Planner
License:        MIT
URL:            https://github.com/oacoello/worship-planner
BuildArch:      ${ARCH_RPM}
AutoReqProv:    no

%description
Granco Planner is a native Rust desktop application for planning worship services,
sermon series, musical setlists, song arrangements, and song rotation analytics.

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/icons/hicolor/512x512/apps
install -m 0755 "${BIN_PATH}" %{buildroot}/usr/bin/${APP_NAME}
install -m 0644 "${ROOT_DIR}/packaging/linux/granco-planner.desktop" %{buildroot}/usr/share/applications/${APP_NAME}.desktop
install -m 0644 "${ROOT_DIR}/assets/icon.png" %{buildroot}/usr/share/icons/hicolor/512x512/apps/${APP_NAME}.png

%files
/usr/bin/${APP_NAME}
/usr/share/applications/${APP_NAME}.desktop
/usr/share/icons/hicolor/512x512/apps/${APP_NAME}.png
EOF

    rpmbuild --define "_topdir ${RPM_TOP}" --define "_tmppath /tmp" -bb "${RPM_TOP}/SPECS/${APP_NAME}.spec"
    find "${RPM_TOP}/RPMS" -name "*.rpm" -exec cp {} "${RELEASE_DIR}/" \;
    rm -rf "${RPM_TOP}"
    echo "Built RPM installer in ${RELEASE_DIR}"
fi

# ---------------------------------------------------------
# 3. Build Portable .tar.gz & Self-Extracting .sh Installer
# ---------------------------------------------------------
TAR_STAGE="$(mktemp -d)"
PKG_FOLDER="${APP_NAME}-${VERSION}-linux-x86_64"
mkdir -p "${TAR_STAGE}/${PKG_FOLDER}"
cp "${BIN_PATH}" "${TAR_STAGE}/${PKG_FOLDER}/${APP_NAME}"
cp "${ROOT_DIR}/packaging/linux/granco-planner.desktop" "${TAR_STAGE}/${PKG_FOLDER}/"
cp "${ROOT_DIR}/assets/icon.png" "${TAR_STAGE}/${PKG_FOLDER}/"

TAR_OUT="${RELEASE_DIR}/${PKG_FOLDER}.tar.gz"
tar -C "${TAR_STAGE}" -czf "${TAR_OUT}" "${PKG_FOLDER}"
rm -rf "${TAR_STAGE}"
echo "Built portable archive: ${TAR_OUT}"

SH_INSTALLER="${RELEASE_DIR}/${APP_NAME}-${VERSION}-linux-x86_64-installer.sh"
cat > "${SH_INSTALLER}" <<'EOF_HEADER'
#!/usr/bin/env bash
set -euo pipefail

echo "=================================================="
echo "  Installing Granco Planner v1.0.0 (Native Rust)  "
echo "=================================================="

PREFIX="${HOME}/.local"
BIN_DIR="${PREFIX}/bin"
APP_DIR="${PREFIX}/share/applications"
ICON_DIR="${PREFIX}/share/icons/hicolor/512x512/apps"

mkdir -p "${BIN_DIR}" "${APP_DIR}" "${ICON_DIR}"

ARCHIVE_LINE=$(awk '/^__ARCHIVE_BELOW__/ {print NR + 1; exit 0; }' "$0")
TMP_EXTRACT="$(mktemp -d)"
tail -n +"${ARCHIVE_LINE}" "$0" | tar -xz -C "${TMP_EXTRACT}"

EXTRACTED_DIR=$(find "${TMP_EXTRACT}" -mindepth 1 -maxdepth 1 -type d | head -n 1)
install -m 0755 "${EXTRACTED_DIR}/granco-planner" "${BIN_DIR}/granco-planner"
install -m 0644 "${EXTRACTED_DIR}/icon.png" "${ICON_DIR}/granco-planner.png"

sed "s|/usr/bin/granco-planner|${BIN_DIR}/granco-planner|g" \
    "${EXTRACTED_DIR}/granco-planner.desktop" > "${APP_DIR}/granco-planner.desktop"

rm -rf "${TMP_EXTRACT}"

echo "✔ Granco Planner installed to ${BIN_DIR}/granco-planner"
echo "✔ Desktop entry created at ${APP_DIR}/granco-planner.desktop"
exit 0
__ARCHIVE_BELOW__
EOF_HEADER

cat "${TAR_OUT}" >> "${SH_INSTALLER}"
chmod +x "${SH_INSTALLER}"
echo "Built self-extracting Linux installer: ${SH_INSTALLER}"
