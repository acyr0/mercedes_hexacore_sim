# maplestory mercedes hexacore levelling order calculations

If you're just looking for results, go [here](RESULTS.md).

## quick start
As long as you have Rust installed, you can clone this repo, change the various
values in [src/config.rs](src/config.rs) to your liking, and run:
```
cargo run --release
```

If you want the calculations to be based on Sol Erda rather than Sol Erda
Fragments, you can run the following instead:
```
cargo run --release --features sol_erda
```

If you want table output that you can paste into Google Sheets, then you can
use:
```
cargo run --release --features table_output | pbcopy
```

## methodology
The logic applied here greedily chooses the next most efficient upgrade at each
step (effectively fd / cost), with lookahead. Upgrades of different skills are
assumed to be non-interactive (this is not currently the case, but is probably
close enough for our purposes), so we can independently lookahead at all
possible future levels of each individual skill.
