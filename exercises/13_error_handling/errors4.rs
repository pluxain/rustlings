use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.
        // Note: first draft
        // if value > 0 {
        //     Ok(Self(value as u64))
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Err(CreationError::Negative)
        // }

        // Note: using match
        match value.cmp(&0) {
            Ordering::Greater => Ok(Self(value as u64)),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Less => Err(CreationError::Negative),
        }
    }
}

fn main() -> Result<(), CreationError> {
    // You can optionally experiment here.
    let v = PositiveNonzeroInteger::new(10)?;
    println!("Value is {v:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
