#![cfg_attr(not(feature = "std"), no_std)]

// #[test]
// fn test_array_dec() {
//     let bytes = [1_u8, 0, 0, 0];
//     let mut decoder = bytes.iter();
//     let val = decoder.get_le_u16().unwrap();
//     assert_eq!(val, 1);
//     let val = decoder.get_le_u16().unwrap();
//     assert_eq!(val, 0);
// }

#[test]
fn test_slice_dec() {
    let bytes = [1_u8, 0, 0, 0];
    let decoder = &mut bytes.as_ref();
    let val = decoder.get_le_u16();
    assert_eq!(val, 1);
    let val = decoder.get_le_u16().unwrap();
    assert_eq!(val, 0);
}

// #[test]
// fn test_vec_dec() {
//     let bytes = vec![1_u8, 0, 0, 0];
//     let mut decoder = bytes.iter();
//     let val = decoder.get_le_u16().unwrap();
//     assert_eq!(val, 1);
//     let val = decoder.get_le_u16().unwrap();
//     assert_eq!(val, 0);
// }

pub trait Decoder {
    type E;

    fn get_u8(&mut self) -> Result<u8, Self::E>;

    fn take(&mut self, dst: &mut [u8]) -> Result<(), Self::E> {
        for b in dst {
            *b = self.get_u8()?;
        }
        Ok(())
    }

    fn take_2(&mut self) -> Result<[u8; 2], Self::E> {
        let mut buf = [0; 2];
        self.take(&mut buf)?;
        Ok(buf)
    }

    fn take_4(&mut self) -> Result<[u8; 4], Self::E> {
        let mut buf = [0; 4];
        self.take(&mut buf)?;
        Ok(buf)
    }

    fn take_8(&mut self) -> Result<[u8; 8], Self::E> {
        let mut buf = [0; 8];
        self.take(&mut buf)?;
        Ok(buf)
    }

    fn take_16(&mut self) -> Result<[u8; 16], Self::E> {
        let mut buf = [0; 16];
        self.take(&mut buf)?;
        Ok(buf)
    }

    fn get_i8(&mut self) -> Result<i8, Self::E> {
        self.get_u8().map(|b| b as i8)
    }

    fn get_le_i16(&mut self) -> Result<i16, Self::E> {
        self.take_2().map(i16::from_le_bytes)
    }

    fn get_le_u16(&mut self) -> Result<u16, Self::E> {
        self.take_2().map(u16::from_le_bytes)
    }

    fn get_le_i32(&mut self) -> Result<i32, Self::E> {
        self.take_4().map(i32::from_le_bytes)
    }

    fn get_le_u32(&mut self) -> Result<u32, Self::E> {
        self.take_4().map(u32::from_le_bytes)
    }

    fn get_le_i64(&mut self) -> Result<i64, Self::E> {
        self.take_8().map(i64::from_le_bytes)
    }

    fn get_le_u64(&mut self) -> Result<u64, Self::E> {
        self.take_8().map(u64::from_le_bytes)
    }

    fn get_le_i128(&mut self) -> Result<i128, Self::E> {
        self.take_16().map(i128::from_le_bytes)
    }

    fn get_le_u128(&mut self) -> Result<u128, Self::E> {
        self.take_16().map(u128::from_le_bytes)
    }

    fn get_le_f32(&mut self) -> Result<f32, Self::E> {
        self.take_4().map(f32::from_le_bytes)
    }

    fn get_le_f64(&mut self) -> Result<f64, Self::E> {
        self.take_8().map(f64::from_le_bytes)
    }

    fn get_be_i16(&mut self) -> Result<i16, Self::E> {
        self.take_2().map(i16::from_be_bytes)
    }

    fn get_be_u16(&mut self) -> Result<u16, Self::E> {
        self.take_2().map(u16::from_be_bytes)
    }

    fn get_be_i32(&mut self) -> Result<i32, Self::E> {
        self.take_4().map(i32::from_be_bytes)
    }

    fn get_be_u32(&mut self) -> Result<u32, Self::E> {
        self.take_4().map(u32::from_be_bytes)
    }

    fn get_be_i64(&mut self) -> Result<i64, Self::E> {
        self.take_8().map(i64::from_be_bytes)
    }

    fn get_be_u64(&mut self) -> Result<u64, Self::E> {
        self.take_8().map(u64::from_be_bytes)
    }

    fn get_be_i128(&mut self) -> Result<i128, Self::E> {
        self.take_16().map(i128::from_be_bytes)
    }

    fn get_be_u128(&mut self) -> Result<u128, Self::E> {
        self.take_16().map(u128::from_be_bytes)
    }

    fn get_be_f32(&mut self) -> Result<f32, Self::E> {
        self.take_4().map(f32::from_be_bytes)
    }

    fn get_be_f64(&mut self) -> Result<f64, Self::E> {
        self.take_8().map(f64::from_be_bytes)
    }
}

// impl<'a, I> Decoder for I
// where
//     I: Iterator<Item = &'a u8>,
// {
//     type E = ();
//     fn get_u8(&mut self) -> Result<u8, Self::E> {
//         self.next().map(|&b| b).ok_or(())
//     }
// }

impl Decoder for &[u8] {
    type E = ();
    fn get_u8(&mut self) -> Result<u8, Self::E> {
        if self.is_empty() {
            return Err(());
        }
        let b = self[0];
        *self = &self[1..];
        Ok(b)
    }
}
