#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        if s.len() > 12 {
            return vec![];
        }
        let mut ip = String::new();
        let mut ips = Vec::new();
        Solution::restore_ip_addresses_helper(s.as_bytes(), 0, &mut ip, &mut ips);
        return ips;
    }

    fn is_valid_ip_value(s: &[u8]) -> bool {
        if s.is_empty() || s.len() > 3 {
            return false;
        }
        // leading zeros '0x', '0xx'
        if s.len() > 1 && s[0] == '0' as u8 {
            return false;
        }
        // [0, 255]
        let mut value = 0i32;
        for &c in s {
            value = value * 10 + (c as i32 - '0' as i32);
        }
        return value >= 0 && value <= 255;
    }

    fn restore_ip_addresses_helper(
        s: &[u8],
        ip_index: usize,
        ip: &mut String,
        ips: &mut Vec<String>,
    ) {
        if ip_index == 4 {
            if s.is_empty() {
                ips.push(ip.to_string());
            }
            return;
        }

        for i in 1..std::cmp::min(4, s.len() + 1) {
            // println!(
            //     "{} {} {} {}",
            //     i,
            //     ip_index,
            //     std::str::from_utf8(&s[..i]).unwrap(),
            //     std::str::from_utf8(&s[i..]).unwrap()
            // );
            if Solution::is_valid_ip_value(&s[..i]) {
                let old_len = ip.len();
                if ip_index != 0 {
                    ip.push('.');
                }
                ip.push_str(std::str::from_utf8(&s[..i]).expect("s slice not is utf8"));
                Solution::restore_ip_addresses_helper(&s[i..], ip_index + 1, ip, ips);
                ip.truncate(old_len);
            }
        }
    }
}

#[test]
fn restore_ip_addresses_93_test() {
    println!("{}", Solution::is_valid_ip_value("135".as_bytes()));
    assert_eq!(
        Solution::restore_ip_addresses("25525511135".to_string()),
        vec!["255.255.11.135", "255.255.111.35"]
    );
}
