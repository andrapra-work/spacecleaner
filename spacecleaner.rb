class Spacecleaner < Formula
  desc "Fast storage cleanup tool for macOS and Linux"
  homepage "https://github.com/andrapra-work/spacecleaner"
  url "https://github.com/andrapra-work/spacecleaner/archive/v0.1.0.tar.gz"
  sha256 "..." # This would be calculated from the actual release
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "spacecleaner", shell_output("#{bin}/spacecleaner --version")
  end

  def caveats
    <<~EOS
      🧹 SpaceCleaner is ready to use!
      
      Quick Start:
        spacecleaner              # Interactive mode
        spacecleaner scan         # Check storage usage
        spacecleaner quick        # Quick safe cleanup
        spacecleaner --dry-run    # Preview mode
      
      🛡️  Always run with --dry-run first to preview changes!
    EOS
  end
end