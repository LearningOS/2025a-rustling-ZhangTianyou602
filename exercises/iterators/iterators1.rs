// iterators1.rs
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    // Step 1: 获取向量的迭代器（mut 标记为可变，因为 next() 会消耗迭代器）
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   

    // Step 2: 第二个元素是 "custard apple"，迭代器返回 Some(&元素)
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    // Step 3: 第四个元素是 "peach"
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    // Step 4: 迭代器遍历完所有元素后返回 None
    assert_eq!(my_iterable_fav_fruits.next(), None);     
}