// 線形探索法(逐次探索法)
fn main() {
    // 探索したい元の配列(ベクタ)
    let v = vec![4, 1, 3, 2, 5];
    // 配列ををコンソール出力
    println!("{:?}", v);

    // 目的の値
    let x = 3;

    // 配列の要素を前方から検索していく
    let mut i = 0;
    loop {
        // すべての検索が終了するか検索要素が見つかるとloopを抜ける
        if i > v.len() - 1 || x == v[i] {
            break;
        }
        i += 1;
    }
    // xが存在するか判定する。あるならindex番号も出力
    if i <= v.len() - 1 {
        println!("xは存在する: index is {}", i);
    } else {
        println!("xは存在しない");
    }
}
