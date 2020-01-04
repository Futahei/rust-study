fn main() {
  let x = 5;
  
  // ifのあとの式がtrueの時ブロックが実行される
  // falseのときは実行されない
  if x == 5 {
    println!("x は 5 です!");
  }

  // falseの時にも何か実行したいならelseを
  // 場合分けが複数ある時はelse ifを使う
  if x == 5 {
    println!("x は 5 です!");
  } else if x == 6 {
    println!("x は 6 です!");
  } else {
    println!("x は 5 でも 6 でもありません :(");
  }

  // 以下のように使うこともできる
  // これはifが式であるため
  let y = if x == 5 {
    10
  } else {
    15
  }; // y: i32
  // または
  let y = if x == 5 { 10 } else { 15 };

  // elseがない時は()が入る

