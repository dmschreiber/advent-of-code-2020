# Advent of Code 2020

My solutions to AoC 2020 in Rust

### Results on MacBook Pro 2.7 GHz Quad-Core Intel Core i7

Inputs stored in files (need to refactor into an input directory and generalize the path). Timings are inclusive of read time from the file (as sometimes that includes parse time into structs).

```
SOLUTION FOUND 883,1137 = 1003971
SOLUTION FOUND 99,689, 1232 = 84035952
Day 1 complete in 134.698µs
```

```
Good passwords 422
Good passwords 451
Day 2 complete in 1.773804ms
```

```
234 trees found in part 1
5813773056 is the product of all
Day 3 complete in 857.002µs
```

```
valid passports with validation false is 242
valid passports with validation true is 186
Day 4 complete in 1.645584ms
```

```
Day 5
max seat_id is 878
my seat is 504
Day 5 complete in 212.734µs
```

```
Part 1 customs questions are 6930
Part 2 customs questions are 3585
Day 6 complete in 974.065µs
```

```
242 bags can contain shiny gold
Day 7 part 1 complete in 15.980153ms
shiny gold has 176035 bags
Day 7 part 2 complete in 7.386µs
```

```
Infinte loop at None Final accumulator 1930, index 310, and Some(true)
Success at Some(217) Final accumulator 1688, index 622, and None
Day 8 complete in 3.511578ms
```

```
Didn't find numbers that sum 105950735
found 105950735 with min 4117189, max 9709726 and sum 13826915
Day 9 complete in 1.590365ms
```
```
Day 10 part 1 one joltage=71, three joltage=27, product 1917 in 4.455µs
Day 10 part 2 num arrangements 113387824750592 in 50.202µs
Day 10 complete in 77.453µs
```
```
Day 11 Part 1 iteration 71 resulting in 2489 occupied seats
Day 11 Part 2 86 iteratioin resulting in 2180 occupied seats
Day 11 complete in 919.430038ms
```
```
Day 12 Part 1 manhattan is 1441
Day 12 Part 2 manhattan is 61616
Day 12 complete in 281.875µs
```
```
Day 13 part 1 Found bus 523 at timestamp 1000499 product is 2092
Day 13 part 2 Found at time 702970661767766
Day 13 complete in 36.438µs
```
```
Day 14 part 1 sum is 5055782549997
Day 14 part 2 sum is 4795970362286
Day 14 complete in 91.919287ms
```
```
Day 15 part 1 276
Day 15 part 2 31916
Day 15 complete in 5.594521443s
```
```
Day 16 part 1 Ticket scanning error rate 23115
Day 16 239727793813
Day 16 complete in 1.869946ms
```
```
Day 17 part 1 213
Day 17 part 2 1624
Day 17 complete in 792.11353ms
```
```
Day 18 part 1 650217205854
Day 18 part 2 20394514442037
Day 18 complete in 13.225364ms
```

```
Day 19 part 1 complete in 183.217939ms
Day 19 Part 2 Notes:
My matching algorithm isn't working with the modified rules (it processes, but not correctly according to the test data set)
It appears the modified rules loop upon themselves and mean they go forever if they don't satisfy the original rule.
I find that I run out of string and am still trying to match, unsure whether that's failure or success.
My matching in the OR case with different arguments on the left & right mean I needed to potentially go back to the beginning of my Basic2 upon failing the second Argument (rather than just fail)
Next steps:
- Instead of changing the two rules, implement the newly modified Rule 0 manually to account for the looping cases.
- Recognize that the parts of the modified rule themselves aren't modified.
```

```
3803x3539x1571x1439=30425930368573 in 1.618080383s
Day 20 complete in 1.621750835s
Day 20 part 2 notes:
My algorithm now seems to successfully create a grid of tiles that correctly match one another.
Next steps:
- find the right orientation for each tile given their position
- strip the border or each tile, rotate to the right orientation and emit the inner pattern
- collect up all the tiles correctly oriented inner pattern
- figure out if the assembled image is oriented correctly (flip or rotate)
- search for the sea monsters; count the # symbols and subtract the SeaMonsterCount*N(sea_monster)
```

```
Day 21 Crtical allergens ["dairy", "fish", "nuts", "peanuts", "sesame", "shellfish", "soy", "wheat"]
Day 21 part 1 2162
Day 21 part 2 - Crtitical ingredients lmzg,cxk,bsqh,bdvmx,cpbzbx,drbm,cfnt,kqprv
Day 21 complete in 932.130034ms
```