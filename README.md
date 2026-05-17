# Baka Recorder
Baka recorder is a daemon that constantly records the screen using gpu-screen-recorder, and upon receiving the save signal saves 15 second clips in the records directory

# Installation
```git clone https://github.com/mexicanrage/BakaRecorder.git && cd BakaRecorder && sudo ./install.sh```

# Instructions
1. Run the command "BakaRecorder" in the terminal to initialize the daemon

2. Send commands to the daemon socket at "/tmp/bakarecorder/daemon.sock" with your preferred tool and use the daemon commands, "SAVE" to save and "EXIT" to close it. (Example using socat):
```echo -n "SAVE" | socat - UNIX-CONNECT:/tmp/bakarecorder/daemon.sock```

3. It is advisable to assign that command to the keybind of your preference depending on the DE or WM you have and put the daemon in the startup to have everything working.
