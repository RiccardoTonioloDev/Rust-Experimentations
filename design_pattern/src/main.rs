use design_pattern::Post;
use design_pattern::rust_pattern;

fn main() {
    //Primo modo (classica programmazione ad oggetti [non ideale])
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("",post.content());

    post.request_review();
    assert_eq!("",post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today",post.content());

    post.add_text(". Aggiungo altro testo.");
    let copia_content_2 = post.content();

    println!("{}",copia_content_2);

    //Secondo modo (uso dei tipi per definire comportamenti e stati [ideale])
    let mut post = rust_pattern::Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today",post.content());
}
