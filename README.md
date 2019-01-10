# Rustdice

Rustdice is a simple and flexible dice implementation to simulate tabletop dicewhich accept, parse, and roll for any standard string indicating quanity, die, and modifier. 

In the simplest case simply call the Dice.roll with:

```rust
 let dice = Rustdice::new("3d6+1");
 let result = dice.roll();
```

## Options

A number of options are supported to allow common use cases supported by different game systems using the `DiceSetOption` struct.

__best__

Take the best _n_ dice from the dice rolled

```rust
  let dice_options = DiceSetOptions::new(best: 3);
  let dice = Rustdice::new("4D6", dice_options);
  let result = dice.roll();
```

__worst__

Take the worst _n_ dice from the dice rolled

```rust
  let dice_options = DiceSetOptions::new(worst: 3);
  let dice = Rustdice::new("4d10", dice_options);
  let result = dice.roll();
```

__reroll__

Reroll any dice with a value of _n_

```rust
  let dice_options = DiceSetOptions::new(reroll: 1);
  let dice = Rustdice::new("1d20", dice_options);
  let result = dice.roll();
```

__explode__

Any dice with this value get an accumulating reroll found in a few game systems.

```rust
  let dice_options = DiceSetOptions::new(explode: 10);
  let dice = Rustdice::new("3D10", dice_options);
  let result = dice.roll();
```

## Why 

Some people may find this library useful for their Rust projects and they're welcome to use it as such.

My larger purpose for this project is to experiment with Ruby-to-Rust bridging.  The intention is to replicate the functionality provided by [Rubydice](https://github.com/atavistock/rubydice) and then validate and to measure the perfomance benefits or bottlenecks from bridging between ruby and rust.
