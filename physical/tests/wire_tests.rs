use physical::{ChaosConfig, create_wire};

#[test]
fn test_perfect_wire_delivers_everything() {
    let (a, b) = create_wire(ChaosConfig::perfect());

    for i in 0..10 {
        a.send(vec![i]);
    }

    let mut received = 0;
    for _ in 0..10 {
        if b.receive().is_some() {
            received += 1;
        }
    }

    assert_eq!(received, 10);
}

#[test]
fn test_noisy_wire_loses_data() {
    let (a, b) = create_wire(ChaosConfig::noisy());

    for i in 0..100 {
        a.send(vec![i]);
    }

    let mut received = 0;
    for _ in 0..100 {
        if b.receive().is_some() {
            received += 1;
        }
    }

    println!("Sent: 100, Received: {}", received);
    assert!(received < 100);
}
