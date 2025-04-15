[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/lrand-select.svg
[l1]: https://crates.io/crates/lrand-select

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: LICENSE

[s3]: https://docs.rs/lrand-select/badge.svg
[l3]: https://docs.rs/lrand-select/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

The RandomSelector selects among weighted choices, without bias.

```
use rand_select::RandomSelector;
let selector = RandomSelector::default()
   .with(1.0, 'A')
   .with(1.5, 'B')
   .with_none(3.0);
let l = selector.select();
// l has half a chance to be None, and is 50% more likely to be 'B' than 'A'
```

If you set a value and call neither `with_none` nor `with_none_up_to`, the selector will always return a value.

If you have already normalized weight, `with_none_up_to` is a convenient way to set the total weight of the selector:

```
use rand_select::RandomSelector;
let selector = RandomSelector::default()
   .with(0.1, 'A')
   .with(0.2, 'B')
   .with_none_up_to(1.0);
```
The RandomSelector is designed for reuse, and can use the RNG of your choice.
