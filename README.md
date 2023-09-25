<h1 align="center">📝 Metro Exodus Lost Save Fix</h1>
<h5 align="center">Simple Save File FIX for Metro Exodus, written in Rust.</h5>
<h2>💥 Read before Start:</h2>

```sh
- Turn off your Game
- Disable Steam Cloud for Metro Exodus: {
  1. Open Steam.
  2. Right-click on Metro Exodus.
  3. Click "Properties."
  4. In the 'General' tab,
  navigate to the 'STEAM CLOUD' subtab and proceed to disable
  the option named 'Preserve saved games in Steam Cloud for Metro Exodus.'
}
```
<h2>💜 How to Use the Program?</h2>
➜ After launching the program, you need to enter the path to the folder containing the game saves.
<br>- How to get Path?<br>
1. Open windows "Run" Dialog BOX (WIN + R)<br>
2. Enter following text:<br><br>

```sh
%userprofile%
```
3. Now, proceed to the 'Saved Games' folder (note that 'saved games' may vary depending on your system's language)
4. Now, navigate to the 'Metro Exodus' folder, and then enter the folder with a large number of digits (which represents your unique ID)
5. Copy the path to this folder (a sample path is provided in the image below)

<image src="https://media.discordapp.net/attachments/1029109218625736795/1155832339881013318/image.png">

Alternatively, you can accomplish this without manually searching for folders by using the command (not recommended):<br><br>

```sh
%userprofile%\Saved Games\Metro Exodus
```
<h2>🎊 What if ...?</h2>
➜ I don't have a folder with my ID<br>
- Try to reference the parent folder<br>
➜ The application doesn't want to start for me<br>
- Turn off your antivirus program<br>
- Run the application with administrator privileges<br>

<h2>🔰 How to Build the Program?</h2>
In you don't want to build the project, you can find it already built <a href="https://github.com/rubuscode/Metro-Exodus-Save-FIX/releases/download/1.0/save_fix.exe">here</a>.<br>
➜ RUST Installation<br>
1. Download Installer for <a href="https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe">x64</a>, or <a href="https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe">x86</a> bits<br>
2. Complete the installation following the steps<br>
3. ReStart your computer<br>
➜ Building the project<br>
1. Download and extract the project<br>
2. Go to the main folder and open the terminal (CMD) in that path<br>
3. Enter the following command:<br><br>

```sh
cargo build
```
✅ After entering this command, an executable (EXE) file should appear in the 'target/debug' folder, ready to run.

<h2>🎃 Author note:</h2>
➜ This program has been entirely developed by me, although it was inspired by the following project: <a href="https://github.com/JayW24/MetroExodus_SaveFix">CLICK</a><br>
✨If you liked the program, I encourage you to leave a star.
