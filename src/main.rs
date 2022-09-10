/// 한글 자소를 분리해서
/// 초성, 중성, 종성, 8x4x4 체계에서의 각 벌을 구하는 프로그램
use hangul_jaso::*;

fn print_jaso_bul(t: &dyn ToString) {
    let code = utf8_to_ucs2(t).unwrap();
    let jaso = build_jaso(code).unwrap();
    let bul = build_bul(jaso);

    println!("{:#0x} {:?} {:?}", code, jaso, bul)
}

fn main() {
    print_jaso_bul(&"가");
    print_jaso_bul(&"각");
    print_jaso_bul(&"고");
    print_jaso_bul(&"구");
    print_jaso_bul(&"덇");
}
