/// A ViperusValue encapsulates the data bound to its type.
///
/// The implementation of the `From<T>` gets us the `Into<T>` for
/// free. Type conversion is supported in a bidirectional way: For any
/// defined `ViperusValue` enumeration member, we can convert their values.
///
/// # Example
/// ```rust
/// use viperus::ViperusValue;
///
/// let x:i32 = ViperusValue::I32(42).into();
/// ```
///
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
pub enum ViperusValue {
    BOOL(bool),
    I32(i32),
    Empty,
    Str(String),
    Usize(usize),
}

impl From<bool> for ViperusValue {
    fn from(b: bool) -> ViperusValue {
        ViperusValue::BOOL(b)
    }
}

impl From<&bool> for ViperusValue {
    fn from(b: &bool) -> ViperusValue {
        ViperusValue::BOOL(*b)
    }
}

// impl<'a> From<bool> for &'a ViperusValue {
//     fn from(b: bool) -> &'a ViperusValue {
//         ViperusValue::BOOL(b)
//     }
// }

impl From<i32> for ViperusValue {
    fn from(i: i32) -> ViperusValue {
        ViperusValue::I32(i)
    }
}

// impl<'a> From<i32> for &'a ViperusValue {
//     fn from(i: i32) -> &'a ViperusValue {
//         *ViperusValue::I32(i)
//     }
// }

impl From<String> for ViperusValue {
    fn from(src: String) -> ViperusValue {
        ViperusValue::Str(src)
    }
}

impl<'a> From<&'a String> for ViperusValue {
    fn from(src: &'a String) -> ViperusValue {
        ViperusValue::Str(src.clone())
    }
}

impl From<&str> for ViperusValue {
    fn from(src: &str) -> ViperusValue {
        ViperusValue::Str(src.to_owned())
    }
}


// Don't we get them for free??

impl Into<bool> for &ViperusValue {
    fn into(self) -> bool {
        match self {
            ViperusValue::BOOL(i) => *i,
            ViperusValue::Str(s) => s.parse().expect("not a bool"),
            _ => panic!("not a bool"),
        }
    }
}

impl Into<bool> for ViperusValue {
    fn into(self) -> bool {
        match self {
            ViperusValue::BOOL(i) => i,
            ViperusValue::Str(s) => s.parse().expect("not a bool"),

            _ => panic!("not a bool {:?}", self),
        }
    }
}

impl Into<i32> for ViperusValue {
    fn into(self) -> i32 {
        match self {
            ViperusValue::I32(i) => i,

            ViperusValue::Str(s) => s.parse().expect("not an i32"),
            _ => panic!("not an i32"),
        }
    }
}

impl Into<i32> for &ViperusValue {
    fn into(self) -> i32 {
        match self {
            ViperusValue::I32(i) => *i,
            ViperusValue::Str(s) => s.parse().expect("not an i32"),
            _ => panic!("not an i32"),
        }
    }
}

impl<'a> Into<&'a str> for &'a ViperusValue {
    fn into(self) -> &'a str {
         match self {
        ViperusValue::Str(i) => i,
            _ => panic!("not an str"),
         }
     }
 }

impl<'a> Into<String> for &'a ViperusValue {
    fn into(self) -> String {
         match self {
        ViperusValue::Str(i) => i.clone(),
            _ => panic!("not an str"),
         }
     }
 }

impl Into<String> for ViperusValue {
    fn into(self) -> String {
         match self {
        ViperusValue::Str(i) => i,
            _ => panic!("not a string"),
         }
    }
}
