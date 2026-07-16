# Homebrew formula for f00.
#
# Install (once a tap is published):
#   brew install theesfeld/tap/f00
#
# Or from a local clone:
#   brew install --formula ./Formula/f00.rb
#
# Official installer (recommended):
#   curl -fsSL https://f00.sh/install.sh | bash

class F00 < Formula
  desc "Modern, friendly directory lister (ls rewrite in Rust)"
  homepage "https://f00.sh"
  version "0.4.0"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-apple-darwin.tar.gz"
      sha256 "bfd8e3a25a3544b92b3dd6e0f199272ff6edca9bafddad63b92ba2c54cbc8f75"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-apple-darwin.tar.gz"
      sha256 "02ea4215dede1dd989d1b917d4bc0bbbedb48fa5cd1d4a8e73c9607c77041a86"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "dc9eaa2dac9d1ad8b22441cc4001ce9d03592b4ef2b1ca221a1e6d41a285e210"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "bcad6ec3553bd1be5b192a21490bf01b7736048039b4e40da32992d3e7805cd8"
    end
  end

  def install
    bin.install "f00"
  end

  test do
    assert_match "f00", shell_output("#{bin}/f00 --version")
  end
end
