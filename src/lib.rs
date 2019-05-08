use num_bigint::BigInt;
use num_traits::Num;
use redismodule::native_types::RedisType;
use redismodule::{
    redis_command, redis_module, Context, NextArg, RedisError, RedisResult, RedisValue,
};
use std::convert::TryFrom;
use std::ops::Deref;

static BIG_INT: RedisType = RedisType::new("___BIGINT");

fn set(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;
    let value = args.next_string()?;

    // provided number radix, assume base 10
    let radix = args.next_i64().unwrap_or(10);

    // convert our radix into a u32. fails on >u32::MAX or negative numbers
    let radix = match u32::try_from(radix) {
        Ok(r) => r,
        Err(_) => {
            return Err("invalid radix provided, must fit in an unsigned 32 bit integer".into())
        }
    };

    let b = match BigInt::from_str_radix(&value, radix) {
        Ok(b) => b,
        Err(e) => {
            return Err(RedisError::String(format!(
                "error while parsing input: {}",
                e
            )))
        }
    };

    let key = ctx.open_key_writable(&key);

    match key.get_value::<BigInt>(&BIG_INT)? {
        Some(value) => {
            *value = b;
        }
        None => {
            key.set_value(&BIG_INT, b)?;
        }
    }

    Ok(RedisValue::None)
}

fn get(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key = args.next_string()?;

    // provided number radix, assume base 10
    let radix = args.next_i64().unwrap_or(10);

    // convert our radix into a u32. fails on >u32::MAX or negative numbers
    let radix = match u32::try_from(radix) {
        Ok(r) => r,
        Err(_) => {
            return Err("invalid radix provided, must fit in an unsigned 32 bit integer".into())
        }
    };

    let key = ctx.open_key_writable(&key);

    match key.get_value::<BigInt>(&BIG_INT)? {
        Some(v) => Ok(RedisValue::SimpleString(v.to_str_radix(radix))),
        None => Ok(RedisValue::None),
    }
}

fn add(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_target = args.next_string()?;
    let key_other = args.next_string()?;

    if key_target == key_other {
        return Err("adding a value to itself is not allowed".into());
    }

    let key_target = ctx.open_key_writable(&key_target);

    let target = match key_target.get_value::<BigInt>(&BIG_INT)? {
        Some(v) => v,
        None => return Err("target key does not exist".into()),
    };

    let key_other = ctx.open_key_writable(&key_other);

    let other = match key_other.get_value::<BigInt>(&BIG_INT)? {
        Some(v) => v,
        None => return Err("target key does not exist".into()),
    };

    *target += other.deref();

    Ok(RedisValue::None)
}

fn addint(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_target = args.next_string()?;
    let other = args.next_i64()?;

    let key_target = ctx.open_key_writable(&key_target);

    let target = match key_target.get_value::<BigInt>(&BIG_INT)? {
        Some(v) => v,
        None => return Err(RedisError::Str("target key does not exist")),
    };

    *target += other;

    Ok(RedisValue::None)
}

fn inc(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_target = args.next_string()?;

    let key_target = ctx.open_key_writable(&key_target);

    let target = match key_target.get_value::<BigInt>(&BIG_INT)? {
        Some(v) => v,
        None => return Err(RedisError::Str("target key does not exist")),
    };

    *target += 1;

    Ok(RedisValue::None)
}

fn dec(ctx: &Context, args: Vec<String>) -> RedisResult {
    let mut args = args.into_iter().skip(1);
    let key_target = args.next_string()?;

    let key_target = ctx.open_key_writable(&key_target);

    let target = match key_target.get_value::<BigInt>(&BIG_INT)? {
        Some(v) => v,
        None => return Err(RedisError::Str("target key does not exist")),
    };

    *target -= 1;

    Ok(RedisValue::None)
}

redis_module! {
    name: "redis-bigint",
    version: 1,
    data_types: [BIG_INT],
    commands: [
        ["bigint.set", set, "write"],
        ["bigint.get", get, ""],
        ["bigint.add", add, "write"],
        ["bigint.addint", addint, "write"],
        ["bigint.inc", inc, "write"],
        ["bigint.dec", dec, "write"],
    ],
}
