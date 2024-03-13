fn main (){
    use trees::Tree;
    let mut tree = Tree::new(9);
    let  root = tree.root();
assert!( root.has_no_child() );
    assert_eq!( root.data (),&9);
    let mut root = tree.root_mut();
    *root.data_mut()=0;
    println!("The {}",root);
}