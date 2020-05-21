pub fn conv_to_pig(st : &String) -> String {
    let vowel = vec!['a', 'e', 'i', 'o', 'u'];

    let c = st.chars().next().unwrap();
    if vowel.contains(&c) {
        return format!("{}-hay", st);
    }
    
    else {
        return format!("{}-{}ay", &st[1 .. ], c);
    }

}
