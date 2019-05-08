# redis-bigint (TESTING)

Adds bigint support to redis using the redis module system.

## Getting Started

Install [Redis v4.0+](https://redis.io)

Install [rustup](https://rustup.rs)

```bash 
git clone https://github.com/ahouts/redis-bigint
cd redis-bigint
cargo build --release
# change the ".so" to the file extension for shared libraries on your OS
redis-server --loadmodule ./target/release/libredis_bigint.so
```

Documentation about loading redis modules can be found 
[here](https://redis.io/topics/modules-intro).

## Usage

#### BIGINT.SET

Set a key to the desired value. The value is a string which will be interpreted as
a base `radix` integer.

`BIGINT.SET <key> <value> [radix: 10]`

```redis
> BIGINT.SET a 10
(nil)
> BIGINT.SET b 123456789abcdef 16
(nil)
```

#### BIGINT.GET

Get the value of a bigint. The return value is a base `radix` string.

`BIGINT.GET <key> [radix: 10]`

```redis
> BIGINT.SET a 12345
(nil)
> BIGINT.GET a
12345
> BIGINT.GET a 2
11000000111001
```

#### BIGINT.ADD

Add the value of two bigints together, storing the result in the first bigint.

`BIGINT.ADD <target> <other>`

```redis
> BIGINT.SET a 1234
(nil)
> BIGINT.SET b 4321
(nil)
> BIGINT.ADD a b
(nil)
> BIGINT.GET a
5555
```

#### BIGINT.ADDINT

Add a value (in base `radix`) to a bigint. 
The value of `int` must fit in a 64 bit signed integer.

`BIGINT.ADDINT <target> <int> [radix: 10]`

```redis
> BIGINT.SET a 1234
(nil)
> BIGINT.ADDINT a 4321
(nil)
> BIGINT.GET a
5555
```
