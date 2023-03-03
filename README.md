<div align="center">
  <img src="https://user-images.githubusercontent.com/23109089/222835775-3ebb4754-db2d-4787-9ef6-7ca15f2fd6a2.svg" alt="LXDE project logo" width=150 href="http://www.lxde.org/"/>
</div>

<div align="center">
  <p>LXTask is a lightweight and desktop-independent task manager derived from xfce4-taskmanager with all dependencies on xfce removed, new features, and some improvement of the user interface.</p>
</div>

<div align="center">
  <img src="https://user-images.githubusercontent.com/23109089/222985903-85717835-e146-4bb2-9243-2abd3daa5640.png" width="682" />
</div>

## Dependencies

It requires make, automake, intltool and GTK+ 2.6 or newer. Furthermore, you will also need the Rust toolchain to build the project.

To install Rust toolchain, you can access [this page](https://www.rust-lang.org/tools/install), or run the comamnd bellow.
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build and Installation

To build LXTask, follow these steps:
```sh
./autogen.sh
```

Run the configure script to configure the build options:
```sh
./configure
```

Run the make command to build the LXTask:
```sh
make
```

Optionally, you can run the make check command to run the test suite:
```sh
make check
```

Finally, run the sudo make install command to install LXTask on your system:
```sh
sudo make install
```

Other Information

    LXDE uses the Openbox window manager by default, but it can also be used with other window managers.
    LXDE includes a range of lightweight applications, such as the PCManFM file manager and the LXPanel taskbar.
    For more information about LXDE, see the project website at http://lxde.org/.
