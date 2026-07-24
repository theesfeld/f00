# Homebrew formula for f00 (pure assembly multicall coreutils suite).
#
# Install:
#   brew install theesfeld/tap/f00
#
# Official installer:
#   curl -fsSL https://f00.sh/install.sh | bash

class F00 < Formula
  desc "f00tils — pure assembly coreutils replacement (multicall, freestanding)"
  homepage "https://f00.sh"
  version "0.15.1"
  license "MIT"

  on_linux do
    on_intel do
      url "https://github.com/theesfeld/f00/releases/download/v#{version}/f00-0.15.1-linux-x86_64.tar.gz"
      sha256 "cd170d12234b6abb26b2bcebc0d0e8064360a9806d7f69a93cacdee16f6d6958"
    end
  end

  def install
    bin.install "f00"
    utils = %w[
      ls cat true false yes nproc tty whoami basename dirname
      head tail wc tee seq echo pwd sleep
      env printenv realpath readlink pathchk mktemp link unlink sync truncate
      mkdir rmdir chmod touch logname hostid
      cut tr sort uniq rev tac nl fold expand unexpand paste join comm fmt od
      split csplit shuf tsort pr ptx factor numfmt expr
      cp mv rm ln chown chgrp stat df du install mkfifo mknod shred dd dir vdir
      id groups uname arch date users who pinky uptime hostname
      nice nohup timeout kill test printf
      md5sum sha1sum sha256sum sha224sum sha384sum sha512sum b2sum cksum sum
      base64 basenc base32 dircolors chroot stty stdbuf runcon chcon
    ]
    utils.each do |u|
      bin.install_symlink "f00" => "f00-#{u}"
    end
    man1.install Dir["man/man1/*.1"] if Dir.exist?("man/man1")
  end

  test do
    assert_match "f00", shell_output("#{bin}/f00-ls --version")
    system bin/"f00-true"
  end
end
