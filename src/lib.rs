
pub mod errors;
pub mod encoder;


#[cfg(test)]
mod tests {
    use crate::encoder::to_string;
    #[test]
    fn string_encode() {
        assert_eq!(to_string("helloworld"), Ok("10:helloworld".to_string()));
    }

    #[test]
    fn number_encode() {
        assert_eq!(to_string(10 as i8), Ok("i10e".to_string()));
        assert_eq!(to_string(10 as i16), Ok("i10e".to_string()));
        assert_eq!(to_string(10 as i32), Ok("i10e".to_string()));
        assert_eq!(to_string(10 as u8), Ok("i10e".to_string()));
        assert_eq!(to_string(10 as u16), Ok("i10e".to_string()));
        assert_eq!(to_string(10 as u32), Ok("i10e".to_string()));
    }

    #[test]
    fn vec_encode() {
        let v = vec![0, 1, 2, 3];
        assert_eq!(to_string(v), Ok("li0ei1ei2ei3ee".to_string()));
    }

    #[test]
    fn hm_encode() {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        hm.insert("a", "ok");
        hm.insert("b", "test");
        assert_eq!(to_string(hm), Ok("da2:okb4:teste".to_string()));
    }

    #[test]
    fn struct_encode() {
        use serde::Serialize;
        #[derive(Serialize)]
        struct Person {
            name: String,
            surname: String
        }

        let p = Person { name: "john".to_string(), surname: "doe".to_string() };
        assert_eq!(to_string(p), Ok("dname4:johnsurname3:doee".to_string()));
    }


}
