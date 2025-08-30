rm -rf target/release
cargo build --release

BUNDLE_DIR="target/release/bundle"
BUNDLE_APP_DIR="${BUNDLE_DIR}/osx/Digital-Vigilance.app/Contents"

mkdir $BUNDLE_DIR
mkdir $BUNDLE_DIR/osx
mkdir $BUNDLE_DIR/osx/Digital-Vigilance.app
mkdir ${BUNDLE_APP_DIR}
mkdir ${BUNDLE_APP_DIR}/MacOS
mkdir ${BUNDLE_APP_DIR}/Resources

cp target/release/digital_vigilance ${BUNDLE_APP_DIR}/MacOS/digital_vigilance
cp resources/Info.plist ${BUNDLE_APP_DIR}/Info.plist
cp resources/icons/Digital-Vigilance.icns ${BUNDLE_APP_DIR}/Resources/Digital-Vigilance.icns