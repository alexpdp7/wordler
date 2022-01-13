I have not verified these results!

```
$ cargo build --release
```

```
$ time ./target/release/best_starter 
tares

real	0m47.555s
user	0m47.493s
sys	0m0.008s
```

```
$ time target/release/starter_qualities >qualities

real	0m45.962s
user	0m45.694s
sys	0m0.100s
$ sort -n qualities | head
447346	tares
465416	tales
470502	rates
487182	earls
493648	lanes
499466	reals
500748	roles
501708	earns
502226	tears
510038	cares
$ sort -n qualities | tail
7128676	jiffy
7226974	oxbow
7241368	kudzu
7467302	civic
7497460	jinni
7996338	puppy
8004916	yuppy
8347890	mummy
8353778	yummy
8876660	fuzzy
```
