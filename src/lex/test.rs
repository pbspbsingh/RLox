use crate::lex::Lexer;

#[test]
fn test1() {
    let src = r#"
        let a = "My name is Prashant;
        let b = 234;
    "#;
    let err = Lexer::new(src).tokenize().unwrap_err();
    println!("{}", err);
}

#[test]
fn test2() {
    let src = r#"
        let a = 5.2.3;
    "#;
    let err = Lexer::new(src).tokenize().unwrap_err();
    println!("{}", err);
}

#[test]
fn test3() {
    let src = r#"
        let a = 1 == 1 ? 23 : 55;
    "#;
    let err = Lexer::new(src).tokenize().unwrap_err();
    println!("{}", err);
}

#[test]
fn test4() {
    let src = r#"
        let abc1 = (x + y) * z - 'name';
    "#;
    let tokens = Lexer::new(src).tokenize().unwrap();
    for t in tokens {
        println!("{:?}", t);
    }
}