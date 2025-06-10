class Tide < Formula
  desc "Concurrent HTTP load testing tool in Rust for web app performance evaluation"
  homepage "https://github.com/ao/tide"
  url "https://github.com/ao/tide/archive/refs/tags/v0.4.0.tar.gz"
  sha256 "c9d5a8befd5b0050a8ab13bb47a8f9508ff0dfd403b132dc49548bce8d307dc9"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tide 0.4.0", shell_output("#{bin}/tide --version")
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
