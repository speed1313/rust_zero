use std::iter::Iterator;

/// リストを表す型
#[derive(Debug, Clone)]
enum List<T> {
    Node { data: T, next: Box<List<T>> },
    Nil,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    /// リストを消費して、そのリストの先頭にdataを追加したリストを返す
    fn cons(self, data: T) -> List<T> {
        List::Node {
            data,
            next: Box::new(self),
        }
    }
    fn set(self, data: T, num: i32) -> List<T>
    where T: std::marker::Copy
    {
        let mut tmp =  List::Nil;
        for _ in 0..num{
            tmp = tmp.cons(data);
        }
        tmp
    }

    /// 不変イテレータを返す
    fn iter<'a>(&'a self) -> ListIter<'a, T> {
        ListIter { elm: self }
    }
}

/// 不変イテレータを表す型
struct ListIter<'a, T> {
    elm: &'a List<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    /// 次の要素を指す
    fn next(&mut self) -> Option<Self::Item> {
        match self.elm {
            List::Node { data, next } => {
                self.elm = next;
                Some(data)
            }
            List::Nil => None,
        }
    }
}

fn main() {
    // [2, 1, 0]というリストを生成
    let list = List::new().cons(0).cons(1).cons(2);
    // change list contents to 4 using pointetr

    // forで表示
    for  mut x in list.iter() {
        println!("{x}");
    }

    println!();

    // イテレータで表示
    let mut it = list.iter();
    println!("{:?}", it.next().unwrap());
    println!("{:?}", it.next().unwrap());
    println!("{:?}", it.next().unwrap());

     println!();

    let list = List::new().set(4, 3);
    for  mut x in list.iter() {
        println!("{x}");
    }
    println!("{}",list.iter().sum::<i32>());

}
