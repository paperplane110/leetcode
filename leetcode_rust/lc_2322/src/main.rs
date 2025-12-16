fn main() {
    println!("Hello, world!");
    let mut name = String::from("Mike");
    let me = "Tim";
    let hello = |name: &String| -> String {
        format!("Hello, {}! from {}", name, me)
    };
    println!("{}", hello(&name));
    name += "232";
    println!("{}", hello(&name))
}

fn dfs(
    x: usize, 
    fa: Option<usize>,
    adj: &Vec<Vec<usize>>,
    nums: &Vec<i32>,
    xor: &mut Vec<i32>,
    in_: &mut Vec<i32>,
    out_: &mut Vec<i32>,
    clock: &mut i32
) {
    *clock += 1;
    in_[x] = *clock;
    xor[x] = nums[x];
    for &y in &adj[x] {
        if Some(y) != fa {
            dfs(y, Some(x), adj, nums, xor, in_, out_, clock);
            xor[x] ^= xor[y];
        }
    }
    out_[x] = *clock;
}

fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
    let n = nums.len();
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
    for edge in &edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut xor = vec![0; n];
    let mut in_ = vec![0; n];
    let mut out_ = vec![0; n];
    let mut clock = 0;
    dfs(0, None, &adj, &nums, &mut xor, &mut in_, &mut out_, &mut clock);

    let is_ancestor = |x: usize, y: usize| -> bool {
        in_[x] <= in_[y] && in_[y] <= out_[x]
    };

    let mut ans = i32::MAX;
    for x in 2..n {
        for y in 1..x {
            let (a, b, c) = if is_ancestor(x, y) {
                (xor[y], xor[x] ^ xor[y], xor[0] ^ xor[x])
            } else if is_ancestor(y, x) {
                (xor[x], xor[y] ^ xor[x], xor[0] ^ xor[y])
            } else {
                (xor[x], xor[y], xor[0] ^ xor[x] ^ xor[y])
            };

            let max_val = a.max(b).max(c);
            let min_val = a.min(b).min(c);
            ans = ans.min(max_val - min_val);

            if ans == 0 {
                return ans;
            }
        }
    }
    ans
}