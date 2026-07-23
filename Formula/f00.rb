# Homebrew formula for f00 (pure assembly multicall coreutils suite).
#
# Install:
#   brew install theesfeld/tap/f00
#
# Official installer (recommended):
#   curl -fsSL https://f00.sh/install.sh | bash
#   curl -fsSL https://f00.sh/install.sh | F00_VERSION=v0.15.0 bash
#
# Linux x86-64 freestanding static binary. macOS bottles TBD (Darwin layer).

class F00 < Formula
  desc "Pure assembly GNU coreutils replacement suite (multicall, freestanding)"
  homepage "https://f00.sh"
  version "0.15.0"
  license "MIT"

  on_linux do
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v0.15.0/f00-0.15.0-linux-x86_64.tar.gz"
      sha256 "4e9420e48412e9765d30132e8dcb6829087343fcddd94bd436b6d6dfcb685b99"
    end
  end

  def install
    bin.install "f00"
    # multicall links for common tools
    %w[
      ls cat head tail wc true false yes id date uname
      basename dirname pwd echo env sort uniq cut tr
      cp mv rm mkdir md5sum sha256sum nproc whoami tty
      df du stat realpath
    ].each do |u|
      bin.install_symlink "f00" => "f00-#{u}"
    end
    man1.install Dir["man/man1/*.1"] if Dir.exist?("man/man1")
  end

  test do
    assert_match "f00", shell_output("#{bin}/f00-ls --version")
  end
end
