# Nahoda - Cryptographic Random Number Generator

[Czech version (Česká verze)](./readme.md)

## About the Project

This program implements a highly unpredictable random number generator named "Nahoda" (meaning "randomness" in Czech). The generator utilizes multiple entropy sources and complex cryptographic techniques to ensure high-quality random numbers.

## Usage

The program requires `min` and `max` parameters and an optional `count` parameter:

```
nahoda.exe <min> <max> [count]
```

where:

- `<min>` - minimum value for generated numbers (required)
- `<max>` - maximum value for generated numbers (required)
- `[count]` - number of values to generate (optional, default: 1, maximum: 100)

### Usage Examples:

```
nahoda.exe 1 100
```

Generates one random number in the range 1-100.

```
nahoda.exe 1 6 10
```

Generates 10 random numbers in the range 1-6 (dice roll simulation).

## Technical Details

### Entropy Sources

The generator collects entropy from multiple sources to ensure maximum unpredictability:

1. **System time** - with nanosecond precision
2. **Memory addresses** - using variable addresses provides an unpredictable data source
3. **Process ID** - identifier of the current process
4. **Thread information** - address of the current thread object
5. **Environment variables** - values of system environment variables
6. **Stack and heap addresses** - memory locations provide additional entropy

### Mixing Algorithm

The generator uses the following techniques for mixing entropy:

1. Prime multipliers (linear congruential generator)
2. Bit operations (XOR, rotations, shifts)
3. Hashing for better uniform distribution
4. Dynamic state updates when generating each number
5. Secret buffer used for non-linear operations

### Implementation

The main `NahodaGenerator` class contains:

- `state` (128 bits) - stores the current state of the generator
- `counter` - counts generated numbers, ensures different results
- `tajemstvi` (secret) - buffer of random values for complex mixing

#### Main Methods:

- `new()` - initializes the generator by collecting entropy from various sources
- `dalsi_cislo()` - generates the next random 64-bit number
- `cislo_v_rozsahu(min, max)` - generates a number in the specified range [min, max]

## Security

The generator is designed to provide highly unpredictable numbers. Thanks to the number of entropy sources and complex mixing algorithm, it's unrealistic to predict the next generated number. The generator is therefore suitable for cryptographic purposes, gaming applications, and simulations.

## Compilation and Running

To compile the project, use the following commands:

```
cargo build --release
```

To run:

```
cargo run --release -- <min> <max> [count]
```

or:

```
target\release\nahoda.exe <min> <max> [count]
```
