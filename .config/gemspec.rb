# See: https://guides.rubygems.org/specification-reference/

Gem::Specification.new do |gem|
  gem.version            = File.read("VERSION").chomp
  gem.date               = File.mtime("VERSION").strftime("%Y-%m-%d")

  gem.name               = "asimov-serpapi-module"
  gem.homepage           = "https://github.com/asimov-modules"
  gem.license            = "Unlicense"
  gem.summary            = "ASIMOV module for data import powered by the SerpApi search data platform."
  gem.description        = gem.summary
  gem.metadata           = {
    "homepage_uri"      => gem.homepage,
    "source_code_uri"   => "https://github.com/asimov-modules/asimov-serpapi-module",
    "changelog_uri"     => "https://github.com/asimov-modules/asimov-serpapi-module/blob/master/CHANGES.md",
    "bug_tracker_uri"   => "https://github.com/asimov-modules/asimov-serpapi-module/issues",
    "documentation_uri" => "https://github.com/asimov-modules/asimov-serpapi-module/blob/master/README.md",
  }

  gem.author             = "ASIMOV Protocol"
  gem.email              = "support@asimov.so"

  gem.platform           = ENV["RUBY_PLATFORM"] || Gem::Platform::CURRENT
  gem.bindir             = %q(bin)
  gem.executables        = %w(asimov-serpapi-fetcher asimov-serpapi-importer)
  gem.files              = %w(AUTHORS CHANGES.md README.md UNLICENSE VERSION) + Dir.glob("lib/**/*.rb") +
                           gem.executables.map { |name| Dir.glob("lib/#{gem.name}/#{name}{,.exe}") }.flatten

  gem.required_ruby_version = ">= 3.0"
end
