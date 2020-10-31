use proptest::prelude::*;
use rdftk_iri::Host;
use rdftk_iri::ValidateStr;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// API Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_is_host() {
    let host = Host::new_domain_name("example.org").unwrap();
    println!("{:?}", host.value());

    assert!(host.is_domain_name());
    assert!(!host.is_ipv4_address());
    assert!(!host.is_ipv6_address());
    assert!(!host.is_ip_future_address());

    assert_eq!(host.to_string(), "example.org".to_string());
}

#[test]
fn test_is_ipv4() {
    let host = Host::new_ipv4_address(Ipv4Addr::LOCALHOST).unwrap();
    println!("{:?}", host.value());

    assert!(!host.is_domain_name());
    assert!(host.is_ipv4_address());
    assert!(!host.is_ipv6_address());
    assert!(!host.is_ip_future_address());

    assert_eq!(host.to_string(), "127.0.0.1".to_string());
}

// [2001:db8:85a3:8d3:1319:8a2e:370:7348]

#[test]
fn test_is_ipv6() {
    let host = Host::new_ipv6_address(Ipv6Addr::LOCALHOST).unwrap();
    println!("{:?}", host.value());

    assert!(!host.is_domain_name());
    assert!(!host.is_ipv4_address());
    assert!(host.is_ipv6_address());
    assert!(!host.is_ip_future_address());

    assert_eq!(host.to_string(), "[::1]".to_string());
}

#[test]
fn test_is_ipv_future() {
    let host = Host::new_ipv_future_address(7, "::1").unwrap();
    println!("{:?}", host.value());

    assert!(!host.is_domain_name());
    assert!(!host.is_ipv4_address());
    assert!(!host.is_ipv6_address());
    assert!(host.is_ip_future_address());

    assert_eq!(host.to_string(), "[v7.::1]".to_string());
}

#[test]
fn test_host_from_str() {
    let host = Host::from_str("example.org").unwrap();
    println!("{:?}", host.value());

    assert!(host.is_domain_name());
    assert!(!host.is_ipv4_address());
    assert!(!host.is_ipv6_address());
    assert!(!host.is_ip_future_address());

    assert_eq!(host.to_string(), "example.org".to_string());
}

#[test]
fn test_ipv4_from_str() {
    let host = Host::from_str("127.0.0.1").unwrap();
    println!("{:?}", host.value());

    assert!(!host.is_domain_name());
    assert!(host.is_ipv4_address());
    assert!(!host.is_ipv6_address());
    assert!(!host.is_ip_future_address());

    assert_eq!(host.to_string(), "127.0.0.1".to_string());
}

// [2001:db8:85a3:8d3:1319:8a2e:370:7348]

#[test]
fn test_ipv6_from_str() {
    let host = Host::from_str("[::1]").unwrap();
    println!("{:?}", host.value());

    assert!(!host.is_domain_name());
    assert!(!host.is_ipv4_address());
    assert!(host.is_ipv6_address());
    assert!(!host.is_ip_future_address());

    assert_eq!(host.to_string(), "[::1]".to_string());
}

#[test]
fn test_ipv_future_from_str() {
    let host = Host::from_str("[v7.::1]").unwrap();
    println!("{:?}", host.value());

    assert!(!host.is_domain_name());
    assert!(!host.is_ipv4_address());
    assert!(!host.is_ipv6_address());
    assert!(host.is_ip_future_address());

    assert_eq!(host.to_string(), "[v7.::1]".to_string());
}

// ------------------------------------------------------------------------------------------------
// Automated Property Tests
// ------------------------------------------------------------------------------------------------

proptest! {
    #[test]
    fn doesnt_crash(s in "\\PC*") {
        let _ = Host::from_str(&s);
    }

    #[test]
    fn valid_ipv4_values(a in 0..255u8, b in 0..255u8, c in 0..255u8, d in 0..255u8) {
        let s = format!("{}.{}.{}.{}", a, b, c, d);
        println!("valid_ipv4_values {:?}", s);
        assert!(Host::is_valid(&s));
        assert!(Host::from_str(&s).is_ok());
    }
}
