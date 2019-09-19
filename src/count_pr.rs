
//计算成功概率用的






use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;




pub fn b1(a: f64, r: f64) -> f64 {
        let b = r + 2.0 * a;
        b
}

pub fn hb(x: f64) -> f64 {
    //二元熵函数
        let y = -x * (x.log2()) - (1.0 - x) * ((1.0 - x).log2());
        y
}

pub fn creat_permutation(n: i64, a: f64, r: f64) -> HashMap<i64, HashSet<i64>> {
    let mut map1: HashMap<i64, HashSet<i64>> = HashMap::new();
    let mut rng = rand::thread_rng();
    let b = b1(a, r); //生成a，b的（0，1）的随机数
    let k = (hb(a) + hb(b)) / (hb(a) - b * hb(a / b)); //取k临界值
    let d = k.ceil() as i64;
    for j in 0..d * n {
        let i = rng.gen_range(0, d * n);
        if map1.get(&(j % n)) == None {
            let set: HashSet<i64> = HashSet::new();
            map1.insert(j % n, set);
        }
        map1.get_mut(&(j % n)).unwrap().insert(i % n);
    }
    map1
}
pub fn count_pr(n: i64, b: f64, map: HashMap<i64, HashSet<i64>>) -> f64 {
    let mut count_bad = 0;
    let mut count_good = 0;
    for oi in 0..n {
        for ii in oi + 1..n {
            let mut nset = HashSet::new();
            for v in map.get(&oi).unwrap().iter() {
                nset.insert(v);
            }
            for v in map.get(&ii).unwrap().iter() {
                nset.insert(v);
            }
            let l = nset.len();
            if l >= b as usize {
                count_good += 1;
            } else {
                count_bad += 1;
            }
        }
    }
    let pr = count_good as f64 / (count_good as f64 + count_bad as f64);
    println!("good pr is {:?}", pr);
    pr
}
