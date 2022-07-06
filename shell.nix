with import <nixpkgs> { };

pkgs.mkShell {
  nativeBuildInputs = [ rustc cargo gcc ];
  buildInputs = [ rustfmt clippy ] ++ (
    lib.optional stdenv.isDarwin [
      libiconv
      darwin.apple_sdk.frameworks.Security
      darwin.apple_sdk.frameworks.CoreFoundation
      darwin.apple_sdk.frameworks.Cocoa
      darwin.apple_sdk.frameworks.MetalKit
      darwin.apple_sdk.frameworks.AVFoundation
      darwin.apple_sdk.frameworks.CoreText
      darwin.apple_sdk.frameworks.CoreGraphics
      darwin.apple_sdk.frameworks.Metal
    ]
  );

  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

   shellHook = (
    if stdenv.isDarwin then
      ''export NIX_LDFLAGS="-F${darwin.apple_sdk.frameworks.Security}/Library/Frameworks -framework Security -F${darwin.apple_sdk.frameworks.Metal}/Library/Frameworks -framework Metal -F${darwin.apple_sdk.frameworks.Carbon}/Library/Frameworks -framework Carbon -F${darwin.apple_sdk.frameworks.CoreText}/Library/Frameworks -framework CoreText -F${darwin.apple_sdk.frameworks.CoreGraphics}/Library/Frameworks -framework CoreGraphics -F${darwin.apple_sdk.frameworks.AVFoundation}/Library/Frameworks -framework AVFoundation -F${darwin.apple_sdk.frameworks.Cocoa}/Library/Frameworks -framework Cocoa -F${darwin.apple_sdk.frameworks.MetalKit}/Library/Frameworks -framework MetalKit -F${darwin.apple_sdk.frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";''
    else
      ""
  );
}