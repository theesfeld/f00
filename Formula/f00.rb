# Homebrew formula stub for f00.
# Usage (once taps exist):
#   brew install theesfeld/tap/f00
# or:
#   brew install --formula ./Formula/f00.rb
#
# Prefer the official installer until a tap is published:
#   curl -fsSL https://f00.sh/install.sh | bash

class F00 < Formula
  desc "Modern, friendly directory lister (ls rewrite in Rust)"
  homepage "https://f00.sh"
  version "0.4.0"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-apple-darwin.tar.gz"
      # sha256: fill from release SHA256SUMS when bottling
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-apple-darwin.tar.gz"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-unknown-linux-gnu.tar.gz"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-unknown-linux-gnu.tar.gz"
    end
  end

  def install
    bin.install "f00"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/f00 --version")
  end
end
