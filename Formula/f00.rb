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
  version "0.10.5"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-apple-darwin.tar.gz"
      sha256 "13a199e9a7dbf173bbc01d80b8d4ae6d05d92e6c8fc4b2fb5279e35b2ec62fad"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-apple-darwin.tar.gz"
      sha256 "8855d7e9c3ffcc06b06da7103948d13c9fa2f600063f649d445d246ca17183b8"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "2e868d2b1d0b23c6c1fefbb782fe356c0dd1f8924bb48f44040b05a0cbadfe38"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "444a44569e2bd79e843b62ed08769d7aa8c0da572d1b5e8716d5e5ec1beaa7ac"
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
