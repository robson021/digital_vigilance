# Digital Vigilance
Prevents from sleeping when the machine is idle. Similar app to mouse jiggler. It moves the mouse unnoticeably (back and forth by one pixel).
<br>The app is lightweight and efficient. It consumes almost no CPU.

App icon downloaded from: https://www.flaticon.com/free-icon-font/contact-lens-eyes_16772568

To use it without terminal (just tray icon in menu bar) you need to build executable app for Mac [https://github.com/burtonageo/cargo-bundle]: `cargo bundle --release`<br>
As a Mac user you may need to grant access to the app and/or terminal. Go to settings: `Privacy & Security` -> `Accessibility`
### How to disable dock icon (to leave only tray-icon in the nav bar):
- run bundle task and go to `target/release/bundle/osx/Digital-Vigilance.app/Contents/Info.plist`
- add these two properties to the file:
<br>`<key>LSBackgroundOnly</key>`<br>`<true/>`