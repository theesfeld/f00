# Homebrew formula for f00.
#
# Install:
#   brew install theesfeld/tap/f00
#
# Official installer (any platform):
#   curl -fsSL https://f00.sh/install.sh | bash
#
# This file is auto-updated on release (Refs: packaging phase).

class F00 < Formula
  desc "Modern, friendly directory lister (ls rewrite in Rust)"
  homepage "https://f00.sh"
  version "0.12.0"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-apple-darwin.tar.gz"
      sha256 "8ea1fd3b7348a316b5c8b5ea1763f7c11031e006c6e1460e6c2c084f64984dc5"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-apple-darwin.tar.gz"
      sha256 "a65aa495099760a83340de996fd38478504eb7c54cce91c4ca80fdae88c81f50"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "e426d203300fcd5ecc10b7a7bc33c73f2daa81cb38d9673c4b00b5a2a62e5d7e"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "56cfe2cc177ff9f1b45303359762ef82e53c4154d5ea90c3fba13d1ba3f48b66"
    end
  end

  def install
    # Release tarball root is f00-<target-triple>/f00
    bin.install Dir["f00-*/f00"].first
  end

  test do
    assert_match "f00", shell_output("#{bin}/f00 --version")
  end
end
