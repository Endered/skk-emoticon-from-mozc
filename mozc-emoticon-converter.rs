#!/usr/bin/env rust-script

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::collections::{HashMap, HashSet};
    use std::io::BufRead;

    let mut table = HashMap::new();
    let mut comments: HashMap<String, String> = HashMap::new();

    let lines: Vec<String> = std::io::stdin().lock().lines().collect::<Result<_, _>>()?;

    let conversions: HashMap<_, _> = lines
        .iter()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .filter(|xs| xs.len() >= 2 && xs[0] != "")
        .map(|xs| {
            let emoticon = xs[0];
            let words: Vec<_> = xs[1..]
                .iter()
                .flat_map(|x| x.split(' ')) // mozcのtsvファイルが区切り文字にtabとspace両方を使用しているため
                .chain(["かお", "かおもじ"])
                .filter(|word| word.chars().all(|c| !c.is_ascii_alphabetic()))
                .collect();
            (emoticon, words)
        })
        .collect();

    for (emoticon, words) in &conversions {
        for word in words {
            table
                .entry(word.to_string())
                .or_insert(HashSet::new())
                .insert(emoticon.to_string());
        }
        comments.insert(emoticon.to_string(), words[0].to_string());
    }

    let mut table: Vec<_> = table.iter().collect();
    table.sort_by_key(|x| x.0);

    println!(";;"); // libskkは1行目を無視するため
    println!(";; okuri-ari entries.");
    println!(";; okuri-nasi entries.");

    let mut escapes = HashMap::new();
    escapes.insert(';', "\\073".to_string());
    escapes.insert('/', "\\057".to_string());

    for (word, emoticons) in table {
        let mut emoticons: Vec<String> = emoticons
            .iter()
            .map(|emoticon| {
                let replaced = {
                    let mut buf = String::new();

                    let chars = emoticon.chars().collect::<Vec<_>>();
                    for i in 0..(chars.len() - 1) {
                        buf += &escapes
                            .get(&chars[i])
                            .map(|x| x.to_string())
                            .unwrap_or_else(|| chars[i].to_string());
                        if escapes.contains_key(&chars[i]) || escapes.contains_key(&chars[i + 1]) {
                            buf += "\" \"";
                        }
                    }
                    buf += &chars
                        .last()
                        .map(|c| {
                            escapes
                                .get(c)
                                .map(|x| x.to_string())
                                .unwrap_or(c.to_string())
                        })
                        .unwrap_or("".to_string());
                    buf
                };
                let replaced = if &replaced == emoticon {
                    replaced
                } else {
                    format!("(concat \"{}\")", replaced)
                };
                let comment = conversions[&emoticon[..]]
                    .iter()
                    .filter(|x| x != &word)
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{replaced};{comment}")
            })
            .collect();

        emoticons.sort();

        println!(
            "{} /{}/",
            word,
            emoticons.into_iter().collect::<Vec<_>>().join("/")
        );
    }

    Ok(())
}
