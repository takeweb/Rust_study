// 直角三角形の構造体
struct RightTriangle {
    base: f64,
    perpendicular : f64,
}

// RightTriangleという構造体に対する実装
impl RightTriangle {
    // 面積を求める
    fn area(&self) -> f64 {
        (self.base * self.perpendicular) * 0.5
    }

    // 周の長さを求める
    fn length(&self) -> f64 {
        // 斜辺の長さは三平方の定理で算出
        self.base + self.perpendicular
            + (self.base.powi(2) + self.perpendicular.powi(2)).sqrt()
    }
}

// 長方形の構造体
struct Rectangle { 
    width: f64,
    height: f64,
}

// Rectangleという構造体に対する実装
impl Rectangle {
    // 面積を求める
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // しゅうの
}

fn main() {
    println!("Hello, world!");
}
