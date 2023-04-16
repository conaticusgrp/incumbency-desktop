with (import <nixpkgs> {});
mkShell {
  nativeBuildInputs = [
    cairo
    gdk-pixbuf
    glib
    gtk3
    gobject-introspection
    libsoup
    webkitgtk
  ];

  buildInputs = [
    pkg-config
  ];

  packages = [
    cargo
    cargo-tauri
  ];
}
