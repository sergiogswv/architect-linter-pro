# typed: false
# frozen_string_literal: true

class ArchitectLinterPro < Formula
  desc "Multi-language architecture linter with AI-powered auto-fix (Hexagonal, Clean, MVC)"
  homepage "https://github.com/sergiogswv/architect-linter-pro"
  version "4.3.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/sergiogswv/architect-linter-pro/releases/download/v#{version}/architect-linter-pro-macos-aarch64"
      sha256 "PLACEHOLDER_AARCH64_MACOS_SHA256" # macos-aarch64
    end
    on_intel do
      url "https://github.com/sergiogswv/architect-linter-pro/releases/download/v#{version}/architect-linter-pro-macos-x86_64"
      sha256 "PLACEHOLDER_X86_64_MACOS_SHA256" # macos-x86_64
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/sergiogswv/architect-linter-pro/releases/download/v#{version}/architect-linter-pro-linux-aarch64"
      sha256 "PLACEHOLDER_AARCH64_LINUX_SHA256" # linux-aarch64
    end
    on_intel do
      url "https://github.com/sergiogswv/architect-linter-pro/releases/download/v#{version}/architect-linter-pro-linux-x86_64"
      sha256 "PLACEHOLDER_X86_64_LINUX_SHA256" # linux-x86_64
    end
  end

  def install
    # Find the downloaded binary (name varies by platform)
    binary = Dir["architect-linter-pro*"].reject { |f| f.end_with?(".sha256") }.first
    raise "Binary not found" unless binary

    bin.install binary => "architect-linter-pro"
  end

  test do
    assert_match "architect-linter-pro", shell_output("#{bin}/architect-linter-pro --version")
  end
end
