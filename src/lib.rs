//! utf8 문자열이나 ucs2 문자열중 한글을 자소로 분리하는 라이브러리
//!
//! 지원하는 한글 자소는 아래와 같다.
//! - 초성 ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ
//! - 중성 ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ
//! - 종성 ㄱㄲ(ㄱㅅ)ㄴ(ㄴㅈ)(ㄴㅎ)ㄷㄹ(ㄹㄱ)(ㄹㅁ)(ㄹㅂ)(ㄹㅅ)(ㄹㅌ)(ㄹㅍ)(ㄹㅎ)ㅁㅂ(ㅂㅅ)ㅅㅆㅇㅈㅊㅋㅌㅍㅎ

/// ucs-2 코드 값으로 확인되는 해당 문자열의 언어
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Languages {
    Ascii,
    Hangul,
    HangulJamo,
    Kana,
    Arrow,
    NotImplemented,
}

/// ucs2 코드로 해당 언어를 반환한다.
pub fn ucs2_language(code: u16) -> Languages {
    match code {
        0x0000..=0x007f => Languages::Ascii,
        0xac00..=0xd7a3 => Languages::Hangul,
        0x3131..=0x3163 => Languages::HangulJamo,
        0x3040..=0x30ff => Languages::Kana,
        0x2190..=0x2199 => Languages::Arrow,
        _ => Languages::NotImplemented,
    }
}

/// 한글의 초, 중, 종성 값을 보관하는 구조체
#[derive(Default, Debug, Copy, Clone)]
pub struct Jaso {
    pub cho: u8,
    pub mid: u8,
    pub jong: u8,
}

/// 각 자소가 어떤 벌 값인지 알려준다.
/// 이 라이브러리에서는 8x4x4 한글 자소 폰트 기반으로
/// 구성하는 것을 전제로 한다.
#[derive(Default, Debug, Copy, Clone)]
pub struct Bul {
    pub cho: Option<u8>,
    pub mid: Option<u8>,
    pub jong: Option<u8>,
}

/// 종성의 갯수. 현대 (2022 기준) 사용되는 종성의 수는 28개이다.
pub const NUM_OF_JONG: u16 = 28;
/// 중성의 갯수. 현대 (2022 기준) 사용되는 중성의 수는 21개이다.
pub const NUM_OF_MID: u16 = 21;

/// UTF8로 표시된 (1~4바이트) 글자를 16비트(2바이트) UCS2 값으로 전환
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
    let cho: Option<u8>;
    let mid: Option<u8>;
    let jong: Option<u8>;
    if jaso.jong == 0 {
        cho = match jaso.mid {
            0..=7 | 20 => Some(0),  // ㅏㅐㅑㅒㅓㅔㅕㅖㅣ
            8 | 12 | 18 => Some(1), // ㅗㅛㅡ
            13 | 17 => Some(2),     // ㅜㅠ
            9..=11 | 19 => Some(3), // ㅘㅙㅚㅢ
            14..=16 => Some(4),     // ㅝㅞㅟ
            _ => None,
        };

        mid = match jaso.cho {
            0..=1 => Some(0),  // ㄱㅋ
            2..=18 => Some(1), // ㄱㅋ 이외
            _ => None,
        };
        jong = None;
    } else {
        cho = match jaso.mid {
            0..=7 | 20 => Some(5),            // ㅏㅐㅑㅒㅓㅔㅕㅖㅣ
            8 | 12 | 13 | 17 | 18 => Some(6), // ㅗㅛㅜㅠㅡ
            9..=11 | 14..=16 | 19 => Some(7), // ㅘㅙㅚㅢㅝㅞㅟ
            _ => None,
        };

        mid = match jaso.cho {
            0..=1 => Some(2),  // ㄱㅋ
            2..=18 => Some(3), // ㄱㅋ 이외
            _ => None,
        };
        jong = match jaso.mid {
            0 | 2 | 9 => Some(0),                      // ㅏㅑㅘ
            4 | 6 | 11 | 14 | 16 | 19 | 20 => Some(1), // ㅓㅕㅚㅝㅟㅢㅣ
            1 | 3 | 5 | 7 | 10 | 15 => Some(2),        // ㅐㅒㅔㅖㅙㅞ
            8 | 12 | 13 | 17 | 18 => Some(3),          // ㅗㅛㅜㅠㅡ
            _ => None,
        };
    }

    Bul { cho, mid, jong }
}

/// 해당 문자열 코드로부터 자소와 벌 값을 한번에 가져오기
pub fn build_jaso_bul(t: &dyn ToString) -> (Jaso, Bul) {
    let code = utf8_to_ucs2(t).unwrap();
    let jaso = build_jaso(code).unwrap();
    let bul = build_bul(&jaso);

    (jaso, bul)
}

/// u16 값(ucs-2)을 자소로 변환하는 유틸리티
impl From<u16> for Jaso {
    fn from(code: u16) -> Self {
        build_jaso(code).unwrap()
    }
}

/// Jaso 값을 토대로 Bul을 반환하는 유틸리티
impl From<&Jaso> for Bul {
    fn from(jaso: &Jaso) -> Self {
        build_bul(jaso)
    }
}
