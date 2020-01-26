// 二分探索法
fn main() {
    // 探索したい元の配列(ベクタ)*昇順ソート済み
    let v = vec![1, 3, 5, 7, 9];
    // 配列ををコンソール出力
    println!("{:?}", v);

    // 目的の値
    let x = 7;
    println!("目的の値xは: {}", x);

    // 二分する際の最小値と最大値それぞれのindex番号用の変数を用意
    let mut low = 0;
    let mut high = v.len() - 1;

    loop {
        // 探索する値が最小値と最大値の間にあるか判定
        if low > high {
            println!("xは存在しない");
            break;
        }
        // 最小値と最大値の中間の値を変数に用意
        let k = (low + high) / 2;
        // 中間の値が目的の値に一致するか判定
        if v[k] == x {
            println!("xは存在する: index is {}", k);
            break;
        }
        // 目的の値が中間の値より大きければその一つ右隣の値を最小値とする
        if v[k] < x {low = k + 1;}
        // 目的の値が中間の値より小さければその一つ左隣の値を最小値とする
        if v[k] > x {
            if k != 0 {
                high = k - 1;
            } else {
                println!("xは存在しない");
                break;
            }
        }
    }
}
