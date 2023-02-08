
<a name="0.3.0"></a>
## 0.3.0 Initial Release (2023-02-07)

## Features

* Add iptables rule comment metrics ([#8](https://github.com/kbknapp/iptables_exporter/pulls/8))

<a name="0.2.0"></a>
## 0.2.0 Initial Release (2022-11-30)

### Features

* Add support for iptables-legacy
  * add new --scrape-target/-t flag ([ffe390e](https://github.com/kbknapp/iptables_exporter/commit/ffe390e ))
* add support for multiple scrape targets ([68bcd25](https://github.com/kbknapp/iptables_exporter/commit/68bcd25 ), [16197d2](https://github.com/kbknapp/iptables_exporter/commit/16197d2))

### CI/Maintenance 

* update MSRV ([33f97f7](https://github.com/kbknapp/iptables_exporter/commit/33f97f7 ))
* add rustfmt and run cargo fmt ([14851d7](https://github.com/kbknapp/iptables_exporter/commit/14851d7 ))
* clippy fixes ([2859393](https://github.com/kbknapp/iptables_exporter/commit/2859393 ))
* update clap syntax ([25edebd](https://github.com/kbknapp/iptables_exporter/commit/25edebd ))
* update deps ([2a3fc2d](https://github.com/kbknapp/iptables_exporter/commit/2a3fc2d ))

<a name="0.1.1"></a>
## 0.1.1 (2021-05-11)

### Fixes

* fix typo in metric name (scape->scrape) ([7c72305](https://github.com/kbknapp/iptables_exporter/commit/7c72305))

### Docs

* fixes typos in readme ([70975fe](https://github.com/kbknapp/iptables_exporter/commit/70975fe ))
* adds releases page to installation ([b23b649](https://github.com/kbknapp/iptables_exporter/commit/b23b649 ))

### CI/Maintenance 

* Uses cross compilation to musl based libc in CI releases ([c6ebfd1](https://github.com/kbknapp/iptables_exporter/commit/c6ebfd1 ))
* adds a step to remove previous nightly release in CI ([3788778 ](https://github.com/kbknapp/iptables_exporter/commit/3788778 ))

<a name="0.1.0"></a>
## 0.1.0 Initial Release (2021-04-30)

* Initial Release
