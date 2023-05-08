use std::collections::HashMap;

/*
    HashMap<K, V> 类型储存了一个键类型 K 对应一个值类型 V 的映射。
    它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中。

    哈希 map 可以用于需要任何类型作为键来寻找数据的情况，而不是像 vector 那样通过索引。
*/

fn test_func1() {
    /*
    可以使用 new 创建一个空的 HashMap，并使用 insert 增加元素。
    */

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    /*
    注意必须首先 use 标准库中集合部分的 HashMap。
    在这三个常用集合中，HashMap 是最不常用的，所以并没有被 prelude 自动引用。
    标准库中对 HashMap 的支持也相对较少，例如，并没有内建的构建宏。
    */

    /* 可以通过 get 方法并提供对应的键来从哈希 map 中获取值。 */
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("name:{}, score:{}", team_name, score);

    /*
    get 方法返回 Option<&V>，如果某个键在哈希 map 中没有对应的值，get 会返回 None。
    程序中通过调用 copied 方法来获取一个 Option<i32> 而不是 Option<&i32>，
        接着调用 unwrap_or 在 score 中没有该键所对应的项时将其设置为零。
    */
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    /*
    对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者。
    */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效，
    // 尝试使用它们看看会出现什么编译错误！
    // println!("field_name:{}, field_value:{}", field_name, field_value); // error

    /*
    如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。
    */

    /*
    当我们想要改变哈希 map 中的数据时，必须决定如何处理一个键已经有值了的情况。
    可以选择完全无视旧值并用新值代替旧值。
    可以选择保留旧值而忽略新值，并只在键 没有 对应值时增加新值。
    或者可以结合新旧两值。让我们看看这分别该如何处理！
    */

    /* 如果我们插入了一个键值对，接着用相同的键插入一个不同的值，与这个键相关联的旧值将被替换。 */
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    /*
    我们经常会检查某个特定的键是否已经存在于哈希 map 中并进行如下操作：
        如果哈希 map 中键已经存在则不做任何操作。如果不存在则连同值一块插入。

    为此哈希 map 有一个特有的 API，叫做 entry，它获取我们想要检查的键作为参数。
    entry 函数的返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值。

    Entry 的 or_insert 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。
    这比编写自己的逻辑要简明的多，另外也与借用检查器结合得更好。
    */
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    /*
    另一个常见的哈希 map 的应用场景是找到一个键对应的值并根据旧的值更新它。
    遍历哈希 map 会以任意顺序进行。
    */
    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }

    /*
    split_whitespace 方法返回一个由空格分隔 text 值子 slice 的迭代器。
    or_insert 方法返回这个键的值的一个可变引用（&mut V）。
    这里我们将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
    这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
    */

    /*
    HashMap 默认使用一种叫做 SipHash 的哈希函数，它可以抵御涉及哈希表（hash table）1 的拒绝服务（Denial of Service, DoS）攻击。
    然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。
    如果性能监测显示此哈希函数非常慢，以致于你无法接受，你可以指定一个不同的 hasher 来切换为其它函数。
    hasher 是一个实现了 BuildHasher trait 的类型。
    */
}

fn main() {
    test_func1();
}
