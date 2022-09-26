use rlt33_statedp::{Post, Post2};

fn main() {
    let mut post = Post::default();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    
    let mut post = Post2::default();
    post.add_text("I ate a salad for dinner today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for dinner today", post.content());
}
