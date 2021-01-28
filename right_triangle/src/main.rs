fn main() {
    let tri = RightTriangle {
        base: 3.0,
        prependicular: 4.0,
    };
    printval(&tri);

    let rec = Reactangle {
        width: 3.0,
        height: 4.0,
    };
    printval(&rec);
}

// トレイト宣言
trait GeoCalculator {
    fn area(&self) -> f64;
    fn length(&self) -> f64;
}

#[derive(Debug)]
// 直角三角形の構造体
struct RightTriangle {
    base: f64,          // 底辺の長さ
    prependicular: f64, // 高さ
}

// トレイトGeoCalculatorが要求するメソッドのRightTriangleに対する実装
impl GeoCalculator for RightTriangle {
    // 面積を求める
    fn area(&self) -> f64 {
        (self.base * self.prependicular) * 0.5
    }
    // 周の長さを求める
    fn length(&self) -> f64 {
        // 斜辺の長さは三平方の定理で求める
        self.base + self.prependicular 
            + (self.base.powi(2) + self.prependicular.powi(2)).sqrt()
    }
}

// 長方形の構造体
#[derive(Debug)]
struct Reactangle {
    width: f64,     // 幅
    height: f64,    // 高さ
}

// トレイトGeoCalculatorが要求するメソッドのReactangleに対する実装
impl GeoCalculator for Reactangle {
    // 面積を求める
    fn area(&self) -> f64 {
        self.width * self.height
    }
    // 周の長さを求める
    fn length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

// fn printval<T: GeoCalculator>(poly: &T) {
fn printval(poly: &dyn GeoCalculator) {
    //println!("poly is {:#?}", poly);
    println!("{}", poly.area());
    println!("{}", poly.length());
}
