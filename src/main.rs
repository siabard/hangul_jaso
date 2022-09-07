/// 한글 자소를 분리해서
/// 초성, 중성, 종성, 8x4x4 체계에서의 각 벌을 구하는 프로그램
use hangul_jaso::*;

fn main() {
    println!("{:#x}", utf8_to_ucs2(&"가").unwrap());
}
