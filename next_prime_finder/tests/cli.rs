use assert_cmd::Command;

#[test]
fn test_cli_prime() {
    let mut cmd = Command::cargo_bin("next_prime_finder").unwrap();
    cmd.arg("7")
        .assert()
        .success()
        .stdout("7 is 1 digit long\n7 is prime\n");
}

#[test]
fn test_cli_not_prime() {
    let mut cmd = Command::cargo_bin("next_prime_finder").unwrap();
    cmd.arg("8")
        .assert()
        .success()
        .stdout("8 is 1 digit long\n8 is not prime, next prime is 11\nPrime is 3 more\n");
}

#[test]
fn test_cli_single_digit() {
    let mut cmd = Command::cargo_bin("next_prime_finder").unwrap();
    cmd.arg("2")
        .assert()
        .success()
        .stdout("2 is 1 digit long\n2 is prime\n");
}

#[test]
fn test_cli_multiple_digits() {
    let mut cmd = Command::cargo_bin("next_prime_finder").unwrap();
    cmd.arg("10")
        .assert()
        .success()
        .stdout("10 is 2 digits long\n10 is not prime, next prime is 11\nPrime is 1 more\n");
}

#[test]
fn test_cli_zero() {
    let mut cmd = Command::cargo_bin("next_prime_finder").unwrap();
    cmd.arg("0")
        .assert()
        .success()
        .stdout("0 is 1 digit long\n0 is not prime, next prime is 2\nPrime is 2 more\n");
}

#[test]
fn test_cli_one() {
    let mut cmd = Command::cargo_bin("next_prime_finder").unwrap();
    cmd.arg("1")
        .assert()
        .success()
        .stdout("1 is 1 digit long\n1 is not prime, next prime is 2\nPrime is 1 more\n");
}
