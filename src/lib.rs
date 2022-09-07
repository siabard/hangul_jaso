/// UTF8로 표시된 (1~4바이트) 글자를 16비트(2바이트) UCS2 값으로 전환하기
pub fn utf8_to_ucs2(s: &dyn ToString) -> Result<u16, String> {
    let str = s.to_string();
    let raw_bytes = str.as_bytes();
    let len = raw_bytes.len();
    let i = 0;

    let result: u16;

    if raw_bytes[i] & 0b1000_0000 == 0b0000_0000 {
        // 해당하는 값은 ASCII 코드 (0~127) 이므로 바로 반환한다.
        result = u16::from(raw_bytes[i]);
    } else if raw_bytes[i] & 0b1110_0000 == 0b1100_0000 {
        // 2 바이트 글자인 경우
        if i + 1 > len {
            // 위 패턴에 맞춰볼 때 이 글자는 2바이트 글자인데, 다음바이트가 없다면
            // 이것은 오류이다.
            return Err("NOT 2 Byte".to_string());
        }

        let a = u16::from(raw_bytes[i] & 0b0001_1111);
        let b = u16::from(raw_bytes[i + 1] & 0b0011_1111);
        result = a << 6 | b;
    } else if raw_bytes[i] & 0b1111_0000 == 0b1110_0000 {
        // 3 바이트 글자인 경우
        if i + 2 >= len || i + 1 >= len {
            return Err("Not 3 Byte".to_string());
        }

        let a = u16::from(raw_bytes[i] & 0b0000_1111);
        let b = u16::from(raw_bytes[i + 1] & 0b0011_1111);
        let c = u16::from(raw_bytes[i + 2] & 0b0011_1111);

        result = a << 12 | b << 6 | c;
    } else if raw_bytes[i] & 0b1111_0000 == 0b1111_0000 {
        // 4 바이트 글자인 경우
        return Err("range is over".to_string());
    } else {
        return Err("Not UTF8".to_string());
    }

    Ok(result)
}
