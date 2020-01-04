fn main() {
  // 何らかの繰り返しを行う処理に対して、
  // Rustは３種類のアプローチを提供している

  // loop
  // 無限ループであり、何らかの終了状態に到達するまでずっとループする
  loop {
    println!("Loop forever!");
  }

  // while
  // 何回ループするか明らかではない時に使う
  let mut x = 5; // mut x: i32
  let mut done = false; // mut done: bool

  while !done {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
      done = true;
    }
  }
  // 無限ループを行う手として
  // while true { }
  // があるが、これは安全性や効率の観点から推奨されていない
  // 無限ループをするなら常に loop を使うべき

  // for
  // 特定の回数だけループする時に使う
  for x in 0..10 {
    println!("{}", x); // x: i32
  }
  // イテレータの各要素を順番に処理して、要素がなくなれば終了する
  // また 0..10 は 0 から 9 までを表示する


  // 列挙
  // ループ内で何回目の繰り返しかを知る必要がある場合、
  // .enumerate() 関数が使える
  // 以下の例では i にインデックスが, j に数値が入る
  for (i, j) in (5..10).enumrate() {
    println!("i = {} and j = {}", i, j); // e.g. i = 0 and j = 5
  }

  // 反復の早期終了
  // break と continue というキーワードがある
  // breakは反復からの脱出、continueは次の反復への移行となる
  let mut x = 5;
  loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
  }

  // ループラベル
  // 入れ子ループがあり、break や continueがどのループに対応するか
  // 指定する必要がある場合 '[name]: をループに付け、
  // break, continueの後に付ければそのループを対象にできる
  // 以下の例ではx, yが両方奇数の時だけ表示を行う
  'outer: for x in 0..10 {
    'inner: for y in 0..10 {
      if x % 2 == 0 { continue 'outer; } // x のループを継続
      if y % 2 == 0 { continue 'inner; } // y のループを継続
      println!("x: {}, y: {}", x, y);
    }
  }
}
