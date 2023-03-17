pub fn climb_stairs(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let mut ways = vec![0; n as usize];
    ways[0] = 1;
    ways[1] = 2;

    for i in 2..n as usize {
        ways[i] = ways[i - 1] + ways[i - 2];
    }

    ways[n as usize - 1]
}

#[test]
fn test_climb_stairs() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}
