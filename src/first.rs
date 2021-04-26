pub fn sort(x: &mut[u32], up: bool){ //pubはほかのモジュールから参照可能、[u32]はスライス（一次元配列と考えてよい）
    if x.len() > 1 {
        let mid_point = x.len() /2;
        sort(&mut x[..mid_point], true);
        sort(&mut x[mid_point..], false);
        sub_sort(x, up);
    }
}

fn sub_sort(x: &mut[u32], up:bool){
    if x.len() > 1 {
        compare_and_swap(x, up);
        let mid_point = x.len() /2;
        sub_sort(&mut x[..mid_point], up);
        sub_sort(&mut x[mid_point..], up);
    }
}

fn compare_and_swap(x: &mut[u32], up:bool){
    let mid_point = x.len() / 2;
    for i in 0..mid_point{
        if(x[i] > x[mid_point + i]) == up{
            x.swap(i, mid_point + i);
        }
    }
}

//corgo testを実行したときのみコンパイル
#[cfg(test)]
//firstモジュールの子供としてtestモジュールの記述
mod tests{
    //親モジュール(first)sort関数を使用する
    use super::sort;

    //#[test]のついた関数はcargo testしたときに実行される
    #[test]
    fn sort_u32_ascending(){
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];

        sort(&mut x, true);

        assert_eq!(x, vec![4, 10, 11, 20, 21, 30, 110, 330]);
    }

    #[test]
    fn sort_u32_descending() {
        let mut x = vec![10, 30, 11, 20, 4, 330, 21, 110];
        sort(&mut x, false);
        // xの要素が降順にソートされていることを確認する
        assert_eq!(x, vec![330, 110, 30, 21, 20, 11, 10, 4]);
    }
}
