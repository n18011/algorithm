// 二分探索法
fn main() {
    // 探索したい元の配列(ベクタ)*昇順ソート済み
    let v = vec![1, 3, 5, 7, 9];
    // 配列ををコンソール出力
    println!("{:?}", v);

    // 目的の値
    let x = 0;

    let mut low = 0;
    let mut high = v.len() - 1;

    loop {
        if low > high {
            println!("xは存在しない");
            break;
        }
        let k = (low + high) / 2;
        println!("k is {}", k);
        if v[k] == x {
            println!("xは存在する: index is {}", k);
            break;
        }
        if v[k] < x {low = k + 1;}
        if v[k] > x {high = k - 1;}
    }
}
