# Lifetime Overrestriction
Bart Massey 2024

This repo contains some examples and demos of "lifetime
overrestriction". Compile each example to see that it works,
then with `--features=appfail` to see it fail because of
lifetime overrestriction, then with
`--features=appfail,libfix` to see it succeed again once the
overrestriction is gone.

## Examples

* `thing`: Needlessly tying struct lifetimes.
* `static_hello`: Implicit lifetime overrestriction.
* `close`: Closure lifetime overrestriction (work in progress)

## References

* Closures and higher-ranked lifetimes:
  * <https://stackoverflow.com/questions/31403723/how-to-declare-a-higher-ranked-lifetime-for-a-closure-argument>
  * <https://rust-lang.github.io/rfcs/3216-closure-lifetime-binder.html>


* Set difference issue: <https://github.com/rust-lang/rust/issues/73788>
