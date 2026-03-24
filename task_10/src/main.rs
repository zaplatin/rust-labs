use std::io::Read;

struct RotDecoder<R: Read> {
    input: R,
    rot: u8,
}

// Реализуем трейт Read для RotDecoder
impl<R: Read> Read for RotDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // 1. Читаем данные из внутреннего input в буфер
        let bytes_read = self.input.read(buf)?;
        
        // 2. Применяем ROT13 к прочитанным байтам
        for byte in &mut buf[..bytes_read] {
            // Применяем ROT13 только к ASCII буквам
            if byte.is_ascii_alphabetic() {
                let base = if byte.is_ascii_uppercase() { b'A' } else { b'a' }; // Определяет базовое значение (смещение) для преобразования ROT13 в зависимости от регистра буквы.
                // Формула ROT13: (буква - base + 13) % 26 + base
                *byte = (*byte - base + self.rot) % 26 + base;
            }
            // Остальные символы (цифры, знаки препинания) не меняются
        }
        
        // 3. Возвращаем количество прочитанных байт
        Ok(bytes_read)
    }
}

fn main() {
    let mut rot = RotDecoder { 
        input: "Gb trg gb gur bgure fvqr!".as_bytes(), 
        rot: 13 
    };
    let mut result = String::new();
    rot.read_to_string(&mut result).unwrap();
    println!("{}", result); // Выведет: "To get to the other side!"
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn joke() {
        let mut rot = RotDecoder { 
            input: "Gb trg gb gur bgure fvqr!".as_bytes(), 
            rot: 13 
        };
        let mut result = String::new();
        rot.read_to_string(&mut result).unwrap();
        assert_eq!(&result, "To get to the other side!");
    }

    #[test]
    fn binary() {
        let input: Vec<u8> = (0..=255u8).collect();
        let mut rot = RotDecoder::<&[u8]> { 
            input: input.as_ref(), 
            rot: 13 
        };
        let mut buf = [0u8; 256];
        assert_eq!(rot.read(&mut buf).unwrap(), 256);
        for i in 0..=255 {
            if input[i] != buf[i] {
                assert!(input[i].is_ascii_alphabetic());
                assert!(buf[i].is_ascii_alphabetic());
            }
        }
    }
}