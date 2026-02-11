class Lob < Formula
  desc "Fast Rust-powered data pipeline CLI tool"
  homepage "https://github.com/olirice/lob"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/olirice/lob/releases/download/v#{version}/lob-#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_ARM64_SHA256"
    else
      url "https://github.com/olirice/lob/releases/download/v#{version}/lob-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_X86_64_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/olirice/lob/releases/download/v#{version}/lob-#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_LINUX_ARM64_SHA256"
    else
      url "https://github.com/olirice/lob/releases/download/v#{version}/lob-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_LINUX_X86_64_SHA256"
    end
  end

  def install
    bin.install "lob"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/lob --version")
  end
end
