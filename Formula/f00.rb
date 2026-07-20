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
  version "0.11.0"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-apple-darwin.tar.gz"
      sha256 "71efaa2a2b8b28a262a3a4b7a3d06921b9f549d6d6e4681dd844cbd3a97f408e"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-apple-darwin.tar.gz"
      sha256 "5d5888bb6c253aacaafd19776e186d34437ef15c5ef8b8dba906956aeea7321a"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "a3a8630236196a61d409e259910d96a50eff9e01e11b4eacee8053b7de892cfb"
    end
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "7645b256021778840b0ed1e10da180f6ce46ddd46032e384055a56a05191d7b0"
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
