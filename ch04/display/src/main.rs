fn main() {
    use std::fmt::{Display,Debug, Formatter};

    /// 虚数を表す型
    #[derive(Debug)]
    struct ImaginaryNumber {
        real: f64,
        img: f64,
    }

    /// 虚数を表示するため、Displayトレイトを実装
    impl Display for ImaginaryNumber {
        fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "{} + {}i", self.real, self.img)
        }
    }
    /// 虚数を表示するため、Debugトレイトを実装
    /*impl Debug for ImaginaryNumber {
        fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
            write!(f, "Re:{}, Im:{}", self.real, self.img)
        }
    }*/

    let n = ImaginaryNumber {
        real: 3.0,
        img: 4.0,
    };

    println!("{n}");
    println!("{:?}",n)
}
