class Tide < Formula
  desc "Concurrent HTTP load testing tool in Rust for web app performance evaluation"
  homepage "https://github.com/ao/tide"
  url "https://github.com/ao/tide/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "5962df6852d5d1b78ace3248f26d5737fee131dda80f2f8b53bbdf1319e85b76"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tide 0.2.0", shell_output("#{bin}/tide --version")
  end

  def caveats
    <<~EOS
      Tide requires accessibility permissions on macOS.
      After installation, go to:
      System Preferences > Security & Privacy > Privacy > Accessibility
      and add Tide to the list of allowed applications.
    EOS
  end
end