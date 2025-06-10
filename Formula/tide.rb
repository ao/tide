class Tide < Formula
  desc "Concurrent HTTP load testing tool in Rust for web app performance evaluation"
  homepage "https://github.com/ao/tide"
  url "https://github.com/ao/tide/archive/refs/tags/v0.3.0.tar.gz"
  sha256 "733f7b3c305ffe39d7324ca9468ddbb2256bf143eba673e69abce1037d86f305"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tide 0.3.0")
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