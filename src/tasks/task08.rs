#[test]
fn example() {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};

    let bufreader = BufReader::new(File::open("test_assets/08_messages.txt").unwrap());

    let mut ecb_encrypted_line = "".to_string();
    let mut ecb_encrypted_blocks = Vec::<String>::new();

    'a: for line in bufreader.lines() {
        let line = line.unwrap();

        // encrypted messages are stored in hex format for this challenge. Cut each line into a
        // Vec<String> where each string is 32 hex chars (16 bytes) in length. Then, sort them and
        // see how many repeats we have.
        let mut blocks = Vec::<String>::new();

        // The message must be a multiple of 16 bytes long.
        assert!(line.len() % 32 == 0);

        while line.len() >= (blocks.len() + 1) * 32 {
            blocks.push(line[blocks.len() * 32..(blocks.len() + 1) * 32].into());
        }

        blocks.sort();

        for i in 1..blocks.len() {
            if blocks[i - 1] == blocks[i] {
                println!("{:?}", line);
                ecb_encrypted_line = line;
                ecb_encrypted_blocks = blocks;
                break 'a;
            }
        }
    }

    let expected_blocks = vec![
        "08649af70dc06f4fd5d2d69c744cd283",
        "08649af70dc06f4fd5d2d69c744cd283",
        "08649af70dc06f4fd5d2d69c744cd283",
        "08649af70dc06f4fd5d2d69c744cd283",
        "9475c9dfdbc1d46597949d9c7e82bf5a",
        "97a93eab8d6aecd566489154789a6b03",
        "ab51b29933f2c123c58386b06fba186a",
        "d403180c98c8f6db1f2a3f9c4040deb0",
        "d880619740a8a19b7840a8a31c810a3d",
        "e2dd052f6b641dbf9d11b0348542bb57",
    ];

    let expected_line = "\
        d880619740a8a19b7840a8a31c810a3d\
        08649af70dc06f4fd5d2d69c744cd283\
        e2dd052f6b641dbf9d11b0348542bb57\
        08649af70dc06f4fd5d2d69c744cd283\
        9475c9dfdbc1d46597949d9c7e82bf5a\
        08649af70dc06f4fd5d2d69c744cd283\
        97a93eab8d6aecd566489154789a6b03\
        08649af70dc06f4fd5d2d69c744cd283\
        d403180c98c8f6db1f2a3f9c4040deb0\
        ab51b29933f2c123c58386b06fba186a";

    assert_eq!(ecb_encrypted_line, expected_line);
    assert_eq!(ecb_encrypted_blocks, expected_blocks);
}
