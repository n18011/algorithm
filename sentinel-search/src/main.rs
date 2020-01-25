// 線形探索法(番兵法)
fn main() {
    // 探索したい元の配列(ベクタ)
    let mut v = vec![4, 1, 3, 2, 5];
    // 配列ををコンソール出力
    println!("{:?}", v);

    // 目的の値
    let x = 3;

    // 番兵を配列の最後尾にセット
    v.push(x);
    // loop時のindex番号用の変数を用意
    let mut i = 0;
    loop {
        // 目的の値が見つかればループを抜ける
        if x == v[i] {break;}
        i += 1;
    }
    // 目的の値が配列中に存在するか判定
    if i < v.len() - 1 {
        println!("xは存在する: index is {}", i);
    } else {
        println!("xは存在しない");
    }
}
