# Android Development (Non-IDE) Cheat Sheet

Since you are using Arch Linux and wanting to avoid Android Studio, here is your workflow.

## 1. Initial Setup (One-time)
If you are on a new machine, run these to get the tools:
```bash
# Install core tools
yay -S android-sdk android-sdk-platform-tools android-sdk-build-tools android-ndk android-emulator jdk17-openjdk

# Setup your JAVA version
sudo archlinux-java set java-17-openjdk
```

## 2. Your Environment (.zshrc)
Make sure these are at the bottom of your `~/.zshrc`:
```bash
export ANDROID_HOME=/opt/android-sdk
export ANDROID_NDK_HOME=/opt/android-ndk
export ANDROID_AVD_HOME=$HOME/.config/.android/avd
export PATH=$PATH:$ANDROID_HOME/platform-tools:$ANDROID_HOME/tools/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/emulator

# Pro-tip: Symlink your AVDs so the emulator always finds them
mkdir -p ~/.android && ln -s ~/.config/.android/avd ~/.android/avd
```

## 3. Creating the Emulator (One-time for 'pixel')
```bash
# Download the OS image
sdkmanager "system-images;android-34;google_apis;x86_64"

# Create the virtual phone
avdmanager create avd -n pixel -k "system-images;android-34;google_apis;x86_64"
```

## 4. Daily Workflow

### Step A: Start the Emulator
You need to open the phone *before* running Dioxus. If you see a black screen, use the reliable software rendering command:

```bash
# Most compatible way to start the emulator (fixes black screen)
emulator -avd pixel -wipe-data -gpu swiftshader_indirect -no-snapshot
```
*Note: The `-gpu swiftshader_indirect` tells the emulator to handle graphics via software rather than your GPU, which avoids driver bugs.*

### Step B: Build and Run
In a second terminal, run your app:
```bash
# Build only
dx build --platform android

# Build and auto-deploy to the open emulator
dx serve --platform android
```

---

## 💡 Pro-Tips
- **Speed:** Use a physical Android phone with a USB cable; it's 10x faster than an emulator.
- **iOS:** To build for iOS, you MUST eventually move your code to a Mac. Dioxus/Tauri/Flutter cannot compile iOS binaries on Linux.
