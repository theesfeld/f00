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
  version "0.10.4"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-apple-darwin.tar.gz"
      sha256 "f94298a6d0b4cc0de7269f0e2a0716b3aeaabb68a9099e7c8651b096e9b1f923"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-apple-darwin.tar.gz"
      sha256 "fbcd9260e86391389e10665a15fe6793648570b02a8925ecffbf64a488744714"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "0c0f6fd886da8099f086d037ddacd75c3fa51943068cd135397a13177b876cb3"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "060420f2af3efd73ef6a6acbaed940f9e52dea8f597c6dd0383f85a7e27e3c47"
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
