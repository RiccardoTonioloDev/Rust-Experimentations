fn main() {
    let mut s = String::from("hello");

    let _word_idx = first_word_idx(&s);

    s.clear(); //ora però il word_idx non è aggiornato (non corrisponde più a verità in quanto ora
    //la parola è vuota).
    
    s.push_str("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11]; //qui stiamo andando a creare delle reference ad una porzione della
    //stringa.
    
    let _word_slice = first_word_slice(&s[..]);
    s.clear();
    //println!("{}",word_slice);
}

fn first_word_idx(s: &String) -> usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    return s.len();
}

fn first_word_slice(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    return &s[..];
}
