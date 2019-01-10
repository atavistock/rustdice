# Rustdice

Rustdice is a simple and flexible dice implementation to simulate tabletop dicewhich accept, parse, and roll for any standard string indicating quanity, die, and modifier. 

In the simplest case simply call the Dice.roll with:

```rust
 let dice = Rustdice::new("3d6+1");
 let result = dice.roll;
```

## Options

A number of options are supported to allow common use cases supported by different game systems using the `DiceSetOption` struct.

_best_

Take the best n dice from the dice rolled

```rust
  let dice_options = DiceSetOptions::new(best: 3);
  let dice = Rustdice::new("4D6", dice_options);
  let result = dice.roll();
```

_worst_

Take the worst n dice from the dice rolled

```rust
  let dice_options = DiceSetOptions::new(worst: 3);
  let dice = Rustdice::new("4d10", dice_options);
  let result = dice.roll();
```

_reroll_

Reroll any dice with a value of 1

```rust
  let dice_options = DiceSetOptions::new(reroll: 1);
  let dice = Rustdice::new("1d20", dice_options);
  let result = dice.roll();
```

_explode_

Any dice with this value get an accumulating reroll

```rust
  let dice_options = DiceSetOptions::new(explode: 10);
  let dice = Rustdice::new("3D10", dice_options);
  let result = dice.roll();
```

## Why 

Some people may find this library useful for their Rust projects and they're welcome to use it as such.

Thoguh one reason I am undertaking this project is to experiment with Ruby/Rust bridging.  The intention is to replicate the functionality provided by [Rubydice](https://github.com/atavistock/rubydice) and then validate the ruby/rust proof of concept and to measure the impact of bridging between ruby and rust.
