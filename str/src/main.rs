// Convert strings to pig latin. The first consonant of each word is moved to the end of the
// word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details
// about UTF-8 encoding!

fn main() {
    let mut s = String::from("foo");
    let mut x = String::from("apple");
    println!("{}", pig_latin(&mut s));
    println!("{}", pig_latin(&mut x));
}

fn pig_latin(s: &mut String) -> String {
    let vowels = String::from("aeiou");
    let mut first_char = s.chars().nth(0).unwrap();
    if vowels.contains(first_char) {
        String::from(format!("{}-hay", s))
    } else {
        first_char = s.remove(0);
        String::from(format!("{}-{}ay", s, first_char))
    }
}
