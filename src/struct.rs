// 「構造体」
// struct はより複雑なデータ型を作る方法の一つ。
// 例えば２次元空間の座標に関する計算を行う時、x と y の両方の値が必要となる
let origin_x = 0;
let origin_y = 0;
// struct ではこれらを一つのデータ型にまとめられる
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let origin = Point { x: 0, y: 0 }; // origin: Point

  println!("The origin is at ({}, {})", origin.x, origin.y);
}

// struct キーワードを使って構造体を作る
// 構造体は慣習により、初めが大文字のキャメルケースで記述する(o: PointInSpace, x: Point_In_Space)
// 通常の型のように let で struct のインスタンスを作成することができるが、key: value スタイルで
// それぞれのフィールドに値をセットする。順序は問わない。

// 構造体のフィールドにはドット表記でアクセスできる
// Rustの他の束縛のように、struct が持つ値はイミュータブルがデフォルト(mut 使ってミュータブルにする)
let mut point = Point { x: 0, y: 0 };
point.x = 5;
println!("The point is at ({}, {})", point.x, point.y); // The point is at (5, 0)

// Rustは言語レベルでフィールドのミュータビリティに対応していないため以下のようには書けない
struct Point {
  mut x: i32,
  y: i32,
}
// ミュータビリティは束縛に付与できる属性であり、構造体自体に付与できる属性ではない
// もしフィールドレベルのミュータビリティを使うのであれば、以下のような奇妙な方法もある
let mut point = Point { x: 0, y: 0 };

point.x = 5;

let point = point; // ここで新しい束縛により変更不可に

point.y = 6; // これはエラーになる


// アップデート構文
// struct の初期化時には、値の一部を他の構造体からコピーしたいことを示す .. を含めることができる
struct Point3d {
  x: i32,
  y: i32,
  z: i32,
}
let mut point = Point3d { x: 0, y: 0, z: 0 };
point = Point3d { y: 1, .. point };


// タプル構造体
// 一般的なタプルと struct のハイブリッドのようなデータ型
// 特徴としてタプル構造体自体には名前があるが、フィールドには名前がない
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 上記二つは同じ値を持つもの同士であっても等しくない
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// ほとんどの場合、タプル構造体よりも struct を使った方がいい
// 例えば Color と Point を struct に書き直すと以下のようになる
struct Color {
  red: i32,
  blue: i32,
  green: i32,
}
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

// タプル構造体が非常に便利なパターンは、要素が１つだけの場合
// 通常の要素の値と区別でき、独自の意味を持つ新しい型を作れる
// 通称「newtype」パターン
struct Inches(i32);

let length = Inches(10);

let Inches(integer_length) = length;
println!("length is {} inches", integer_length);

// 全くメンバを持たない struct を定義できる
struct Electron;
let x = Electron;
// このような構造体は「unit-like」であると言う
// 空のタプルである unit と呼ばれる () とよく似ているから
// これは滅多に役に立たないが、他の機能と組み合わせると便利な場合がある

