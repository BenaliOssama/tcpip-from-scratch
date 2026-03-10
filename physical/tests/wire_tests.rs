use physical::create_wire;

#[test]
fn test_a_sends_to_b() {
    let (a, b) = create_wire();
    a.send(vec![1, 2, 3]);
    assert_eq!(b.receive(), Some(vec![1, 2, 3]));
}

#[test]
fn test_b_sends_to_a() {
    let (a, b) = create_wire();
    b.send(vec![7, 8, 9]);
    assert_eq!(a.receive(), Some(vec![7, 8, 9]));
}

#[test]
fn test_nothing_sent_returns_none() {
    let (_a, b) = create_wire();
    assert_eq!(b.receive(), None);
}
