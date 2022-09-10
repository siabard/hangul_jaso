/// 초성 ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ
/// 중성 ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ
/// 종성 ㄱㄲ(ㄱㅅ)ㄴ(ㄴㅈ)(ㄴㅎ)ㄷㄹ(ㄹㄱ)(ㄹㅁ)(ㄹㅂ)(ㄹㅅ)(ㄹㅌ)(ㄹㅍ)(ㄹㅎ)ㅁㅂ(ㅂㅅ)ㅅㅆㅇㅈㅊㅋㅌㅍㅎ

#[derive(Default, Debug, Copy, Clone)]
pub struct Jaso {
    cho: u8,
    mid: u8,
    jong: u8,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Bul {
    cho: u8,
    mid: u8,
    jong: u8,
}

pub const NUM_OF_JONG: u16 = 28;
pub const NUM_OF_MID: u16 = 21;

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

/// 초성, 중성, 조성의 값을 가져오기
/// 유니코드에서 완성형 한글은 ac00(가)~d7a3(힣)까지 초성, 중성, 종성을 순서대로 조합해 배열한 것이다.
/// 완성형 한글 코드 = (((초성번호 * 중성개수) + 중성번호) * 종성개수) + 종성번호 + ac00
/// 중성갯수 : 21개
/// 종성갯수 : 28개

pub fn build_jaso(code: u16) -> Result<Jaso, String> {
    // MLB (가장 좌측 비트가 1인지 검사)
    if (code & 0b1000_0000_0000_0000) == 0b1000_0000_0000_0000 {
        let hancode = code - 0xac00;
        let jong = hancode % NUM_OF_JONG;
        let mid = ((hancode - jong) / NUM_OF_JONG) % NUM_OF_MID;
        let cho = (hancode - jong) / NUM_OF_JONG / NUM_OF_MID;

        Ok(Jaso {
            cho: cho as u8,
            mid: mid as u8,
            jong: jong as u8,
        })
    } else {
        Err("Not Korean".to_string())
    }
}

/// 8x4x4 폰트 세트에서 초성,중성,종성의 벌을 가져오기
///
///    초성
///    초성 1벌: 받침없는 'ㅏㅐㅑㅒㅓㅔㅕㅖㅣ' 와 결합
///    초성 2벌: 받침없는 'ㅗㅛㅡ'
///    초성 3벌: 받침없는 'ㅜㅠ'
///    초성 4벌: 받침없는 'ㅘㅙㅚㅢ'
///    초성 5벌: 받침없는 'ㅝㅞㅟ'
///    초성 6벌: 받침있는 'ㅏㅐㅑㅒㅓㅔㅕㅖㅣ' 와 결합
///    초성 7벌: 받침있는 'ㅗㅛㅜㅠㅡ'
///    초성 8벌: 받침있는 'ㅘㅙㅚㅢㅝㅞㅟ'
///
///    중성
///    중성 1벌: 받침없는 'ㄱㅋ' 와 결합
///    중성 2벌: 받침없는 'ㄱㅋ' 이외의 자음
///    중성 3벌: 받침있는 'ㄱㅋ' 와 결합
///    중성 4벌: 받침있는 'ㄱㅋ' 이외의 자음
///
///    종성
///    종성 1벌: 중성 'ㅏㅑㅘ' 와 결합
///    종성 2벌: 중성 'ㅓㅕㅚㅝㅟㅢㅣ'
///    종성 3벌: 중성 'ㅐㅒㅔㅖㅙㅞ'
///    종성 4벌: 중성 'ㅗㅛㅜㅠㅡ'
pub fn build_bul(jaso: &Jaso) -> Bul {
    // 종성에 따라 초성, 중성의 벌이 정해진다.
    let cho: u8;
    let mid: u8;
    let jong: u8;
    if jaso.jong == 0 {
        cho = match jaso.mid {
            0..=7 | 20 => 1,  // ㅏㅐㅑㅒㅓㅔㅕㅖㅣ
            8 | 12 | 18 => 2, // ㅗㅛㅡ
            13 | 17 => 3,     // ㅜㅠ
            9..=11 | 19 => 4, // ㅘㅙㅚㅢ
            14..=16 => 5,     // ㅝㅞㅟ
            _ => 0,
        };

        mid = match jaso.cho {
            0..=1 => 1,  // ㄱㅋ
            2..=18 => 2, // ㄱㅋ 이외
            _ => 0,
        };
        jong = 0;
    } else {
        cho = match jaso.mid {
            0..=7 | 20 => 6,            // ㅏㅐㅑㅒㅓㅔㅕㅖㅣ
            8 | 12 | 13 | 17 | 18 => 7, // ㅗㅛㅜㅠㅡ
            9..=11 | 14..=16 | 19 => 8, // ㅘㅙㅚㅢㅝㅞㅟ
            _ => 0,
        };

        mid = match jaso.cho {
            0..=1 => 3,  // ㄱㅋ
            2..=18 => 4, // ㄱㅋ 이외
            _ => 0,
        };
        jong = match jaso.mid {
            0 | 2 | 9 => 1,                      // ㅏㅑㅘ
            4 | 6 | 11 | 14 | 16 | 19 | 20 => 2, // ㅓㅕㅚㅝㅟㅢㅣ
            1 | 3 | 5 | 7 | 10 | 15 => 3,        // ㅐㅒㅔㅖㅙㅞ
            8 | 12 | 13 | 17 | 18 => 4,          // ㅗㅛㅜㅠㅡ
            _ => 0,
        };
    }

    Bul { cho, mid, jong }
}

impl From<u16> for Jaso {
    fn from(code: u16) -> Self {
        build_jaso(code).unwrap()
    }
}

impl From<&Jaso> for Bul {
    fn from(jaso: &Jaso) -> Self {
        build_bul(jaso)
    }
}
