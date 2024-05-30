fn main() {
    // for [1,2,3] the ans = [[1, 2, 3], [2, 1, 3], [2, 3, 1], [2, 1, 3], [2, 3, 1], [2, 1, 3]]
    let n: Vec<i32> = [1,2,3].to_vec();
    let ans = permute(n);
    println!("{:?}", ans);
}

pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ank: Vec<Vec<i32>> = vec![];

    let mut len = nums.len();
    let mut k = nums;
    ank.push(k.to_vec());
    let mut i = 0;
    let mut n = 0;
    if len ==1{
        return ank;
    }
    else if len == 2{
        k.swap(i, i + 1);
        ank.push(k.to_vec());
        return ank;
    }
    loop {

        k.swap(i, i + 1);
        ank.push(k.to_vec());
        if i + 1 == len-1 {
            n+=1;
            i=0;
            if n > len{
                break;
            }
        }
        i+=1;
    }

    ank
}
