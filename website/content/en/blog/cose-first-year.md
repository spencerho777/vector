---
title: Celebrating COSE's First Year - Our Contributions to Vector
short: COSE Team - One Year of Open Source Contributions
description: Celebrating one year of the COSE team and highlighting our contributions to the Vector open source community
authors: [ "pront" ]
date: "2025-11-04"
badges:
  type: announcement
  domains: [ "dev", "community", "opentelemetry" ]
tags: [ "community", "open source", "cose", "contributions", "opentelemetry" ]
---

_One year ago, in October 2024, the COSE (Community Open Source Engineering) team was formed with a mission to strengthen Vector's open
source foundation and improve the developer experience. Today, we're celebrating our first year by highlighting the contributions we've made
to the Vector community._

## Our Journey

Since October 2024, the COSE team has committed over **550 changes** to Vector across **7 major releases** (0.43.0 through 0.50.0), focusing
on three key areas:

- **OpenTelemetry Integration**: Making Vector a first-class citizen in the OTLP ecosystem
- **CI/CD Optimization**: Dramatically reducing build and test times for faster contributor feedback
- **Developer Experience**: Making it easier for contributors and users to work with Vector

## Major Contributions

### OpenTelemetry (OTLP) Support - Our Flagship Achievement

One of our proudest achievements this year has been bringing first-class OpenTelemetry support to Vector. As the observability landscape
increasingly standardizes around OpenTelemetry, we recognized that Vector needed to speak OTLP natively. We started by adding native OTLP
decoding capabilities to the `opentelemetry` source in v0.50.0, which means you can now build seamless OTEL → Vector → OTEL pipelines
without writing custom transforms or worrying about data format conversions. This was complemented by introducing the `otlp` encoder for the
`opentelemetry` sink, creating a complete bidirectional integration.

But we didn't stop at logs. In v0.47.0, we added full metrics ingestion support to the `opentelemetry` source, expanding Vector's
capabilities as a versatile component in OpenTelemetry architectures. Along the way, we enhanced the protobuf codecs with better OTLP
compatibility options and fixed subtle issues like HTTP decompression for OTLP payloads. The result is that Vector now integrates seamlessly
into modern OpenTelemetry-based observability stacks, whether you're using it as a collector, aggregator, or transformation layer.

Read more in our [OTLP Support highlight]({{< ref "/highlights/2025-09-23-otlp-support.md" >}}).

### CI/CD Transformation

When we started looking at Vector's CI/CD pipeline, we found that contributors were often waiting 30+ minutes for test results on their pull
requests. This wasn't just frustrating—it was a real barrier to contribution. When you're excited about a fix or feature, having to wait
half an hour to see if tests pass kills momentum and makes iteration painful. We knew we had to fix this.

We took a systematic approach to optimization. First, we tackled the test runner images, implementing smart reuse so that PR checks could
start running immediately instead of rebuilding containers. Then we added intelligent caching for the vdev development tool, benefiting both
local development and CI runs. We parallelized tests that were running sequentially—miscellaneous tests, E2E tests, and integration tests
all now run concurrently, dramatically cutting total CI time. We also implemented smart test detection, so documentation-only changes don't
trigger the full test suite. Finally, we reorganized the integration test files themselves, making it easier for contributors to find and
run relevant tests locally.

The result? **Up to 60% faster CI runs**. When you submit a pull request now, you get test results in minutes. You can iterate quickly, fix
issues while the code is still fresh in your mind, and get your contributions merged faster. For maintainers, this means we can review and
merge pull requests more efficiently, reducing the backlog and keeping the project moving forward at a healthy pace. These optimizations
have transformed the contribution experience, making Vector development more enjoyable and accessible for everyone.

### Developer Experience Improvements

Beyond CI/CD, we focused on making every aspect of Vector development smoother. We noticed that contributors were often running style checks
across the entire codebase, even when they'd only changed a few files—this could take several minutes. We optimized the style checker to
only look at modified files, achieving a **10x speedup** for local development. We also added color-aware test output, making it easier to
scan test results at a glance. For our macOS contributors, we fixed several issues that were preventing local testing, removing a
significant barrier for developers on Apple hardware.

Documentation is where users often start their Vector journey, so we invested heavily in creating comprehensive guides. We built a debugging
guide that walks you through troubleshooting Vector configurations and deployments—no more guessing why your pipeline isn't working. We
created a config autocompletion guide showing how to set up your IDE for Vector configs, bringing IntelliSense-style features to your YAML
files. We reorganized the AWS guides to be clearer and more comprehensive, and enhanced the protobuf codec documentation. We also made
numerous improvements to VRL function documentation, adding more examples and clearer explanations.

Finally, we automated most of the release preparation steps. This might sound like an internal concern, but it directly impacts you:
automated releases mean fewer human errors, more consistent release quality, and faster turnaround when critical fixes need to go out. It
also frees up maintainer time to focus on reviewing contributions and building features rather than wrestling with release checklists.

### Features & Enhancements

On the feature side, we focused on making Vector more robust and easier to operate in production. We enabled internal logs rate limiting by
default in v0.50.0—a small change that can save you from log storms that could overwhelm your systems. We also made configuration reloads
more reliable by fixing watch-config file event handling and adding support for reloading external VRL scripts on SIGHUP. In v0.46.0, we
added TLS cert/key watching for HTTP sinks, so certificate rotations happen automatically without requiring Vector restarts.

Performance and reliability also saw important improvements. We fixed issues where utilization metrics could be lost during configuration
reloads and where negative utilization values could appear when messages arrived late. We addressed CPU usage metric accuracy problems in
the host_metrics source and prevented TCP metric panics. These might seem like small fixes, but they're the kind of issues that wake you up
at 3am or cause confusion when debugging production problems.

We also made Vector quieter when it should be. The AWS S3 source was generating excessive info-level logs that cluttered log aggregators—we
moved those to debug level where they belong. These quality-of-life improvements make Vector a better citizen in your infrastructure.

### Community Highlights

One of our core missions has been supporting the Vector community, and we're proud of the contributions we helped shepherd through the
review and release process. These represent some of the most exciting additions to Vector this year.

@esensar and @Quad9DNS brought us the memory enrichment table in v0.45.0—a powerful new component that can act as both sink and source,
enabling caching and key-value store use cases that were previously difficult. They also contributed the websocket server sink with message
buffering and ACK support, perfect for real-time streaming scenarios. Their work on custom authorization strategies gave users fine-grained
control over HTTP server source security, while their tag cardinality limits per metric and per-metric-set expiration features help prevent
cardinality explosions in production.

@jorgehermo9 delivered the comprehensive Postgres sink in v0.46.0, supporting logs, metrics, and traces—a significant addition for
PostgreSQL users. They also added filtering to `vector top`, making it easier to monitor large Vector deployments. @sainad2222 contributed
the Keep sink in v0.45.0, and @zvirblis brought us the window transform in v0.47.0 for time-based event aggregation.

These contributions showcase the strength of Vector's open source community. Behind each of these features are real use cases and real users
solving real problems.

## Community Impact

Our work has benefited the entire Vector community:

- **60% Faster CI**: Contributors now get test results much faster, improving iteration speed and making contributing more enjoyable
- **Easier Local Development**: macOS users can now run the full test suite locally, lowering the barrier to contribution
- **Better Observability Integration**: OTLP support makes Vector a natural fit in OpenTelemetry ecosystems, simplifying your observability
  pipelines
- **Improved Stability**: Bug fixes and better error handling reduce production issues and improve reliability
- **Enhanced Documentation**: Comprehensive guides help users get more value from Vector and troubleshoot issues faster

## By the Numbers

Over the past year, the COSE team has:

- **550+** commits to Vector
- **7** major releases supported (0.43.0 through 0.50.0)
- **100+** pull requests merged
- **159** unique community contributors
- **60%** reduction in CI run times for many workflows
- **10x** faster local style checks

## VRL Enhancements

We've also steadily expanded VRL (Vector Remap Language) with numerous new functions and improvements. Across three VRL releases, we added over a dozen new functions including CBOR parsing, character set encoding, LZ4 compression, and various utility functions. We also introduced performance improvements like SIMD-accelerated string conversions and a faster user agent parser. While these are incremental improvements rather than major features, they collectively make VRL more capable and ergonomic for everyday data transformation tasks.

For a detailed list of all VRL changes, see the [VRL changelog appendix](#appendix-vrl-releases) below.

## Looking Ahead

As we enter our second year, we remain committed to:

- **Continuing OpenTelemetry Integration**: Adding trace support and improving OTLP compatibility
- **Optimizing the Developer Experience**: Making contributions even easier and more rewarding
- **Enhancing Testing Capabilities**: Adding more comprehensive test coverage and tooling
- **Improving Documentation**: Making Vector more accessible to new users and contributors
- **Supporting the Community**: Helping community contributions through the review and release process

## Thank You

We want to thank the Vector community for your support, feedback, and contributions. The open source community is what makes Vector great,
and we're honored to contribute to its success.

Special thanks to our COSE team members: **Pavlos Rontidis**, **Thomas**, and all contributors who've helped improve Vector over the past
year, including the amazing community contributors from **Quad9DNS**, **@esensar**, **@jorgehermo9**, and many others.

Here's to another year of building great open source software together! 🚀

---

## Appendix: Release Links

For technical users interested in the detailed changelog for each release we supported:

- [v0.43.0 Release Notes]({{< ref "/releases/0.43.0" >}}) - November 2024
- [v0.44.0 Release Notes]({{< ref "/releases/0.44.0" >}}) - January 2025
- [v0.45.0 Release Notes]({{< ref "/releases/0.45.0" >}}) - February 2025
- [v0.46.0 Release Notes]({{< ref "/releases/0.46.0" >}}) - April 2025
- [v0.47.0 Release Notes]({{< ref "/releases/0.47.0" >}}) - April 2025
- [v0.48.0 Release Notes]({{< ref "/releases/0.48.0" >}})
- [v0.49.0 Release Notes]({{< ref "/releases/0.49.0" >}})
- [v0.50.0 Release Notes]({{< ref "/releases/0.50.0" >}})

## Appendix: VRL Releases

Detailed VRL changes by version:

### v0.21.0 (in Vector release 0.44.0)

**New functions:**

- `crc` - Calculate CRC checksums
- `parse_cbor` - Parse CBOR-encoded data
- `zip` - Combine multiple arrays element-wise
- `decode_charset`, `encode_charset` - Character set encoding/decoding
- `object_from_array` - Build objects from arrays
- `parse_bytes` - Parse byte strings with units

**Enhancements:**

- Enhanced `parse_duration` with multi-unit support (e.g., `1h2m3s`)
- Added `timezone` option to `parse_timestamp` for better timezone handling
- Added `main` log format support to `parse_nginx_log`

### v0.22.0 (in Vector release 0.45.0)

**New functions:**

- `to_syslog_facility_code` - Convert syslog facility keyword to code
- `shannon_entropy` - Calculate Shannon entropy for strings

**Enhancements:**

- Enhanced `ip_cidr_contains` to accept arrays for checking multiple CIDRs
- SIMD-accelerated string conversions for better performance

**Fixes:**

- Fixed decimals parsing in `parse_duration`

### v0.23.0 (in Vector release 0.46.0)

**New functions:**

- `encode_lz4`, `decode_lz4` - LZ4 compression support

**Enhancements:**

- Improved `encode_proto` with automatic type conversion for integer, float, and boolean values to string proto fields
- Faster `parse_user_agent` using the ua-parser library (significant performance improvement)
- Enhanced `snakecase` with excluded_boundaries support for more control over transformations

**Breaking changes:**

- `ip_cidr_contains` now validates CIDR arguments at compile time for constants (better error detection)

---

_Want to contribute to Vector? Check out our:_

- _[Contribution Guide](https://github.com/vectordotdev/vector/blob/master/CONTRIBUTING.md)_
- _[Debugging Guide]({{< ref "/guides/developer/debugging" >}})_
- _[Config Autocompletion Guide]({{< ref "/guides/developer/config-autocompletion" >}})_
- _[Discord Community](https://discord.gg/dX3bdkF)_
