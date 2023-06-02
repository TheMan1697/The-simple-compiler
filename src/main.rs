// 토큰 타입을 나타내는 열거형
enum Token {
    Keyword(String),       // 키워드 (예: System, out, println)
    OpenParenthesis,       // 여는 괄호 (예: ()
    CloseParenthesis,      // 닫는 괄호 (예: ))
    StringLiteral(String), // 문자열 리터럴 (예: "이리오너라!")
    Semicolon,             // 세미콜론 (예: ;)
}

// Java 코드 문자열을 입력으로 받아 토큰 벡터를 반환하는 함수
fn lexical_analysis(input: &str) -> Vec<Token> {
    // 반환할 토큰 벡터를 초기화합니다.
    let mut tokens = Vec::new();

    // 입력 문자열에 대한 이터레이터를 생성하고 peekable로 변환합니다.
    let mut chars = input.chars().peekable();

    // 입력 문자열의 모든 문자를 순회합니다.
    while let Some(c) = chars.next() {
        // 현재 문자에 따라 적절한 토큰을 생성합니다.
        let token = match c {
            // 공백, 새 줄, 탭 등은 무시합니다.
            ' ' | '\n' | '\r' | '\t' => {
                continue;
            }
            // 여는 괄호, 닫는 괄호, 세미콜론은 해당 토큰으로 변환합니다.
            '(' => {
                Token::OpenParenthesis
            }
            ')' => {
                Token::CloseParenthesis
            }
            ';' => {
                Token::Semicolon
            }
            // 따옴표로 시작하는 경우 문자열 리터럴로 간주합니다.
            '"' => {
                let mut string = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    } else {
                        string.push(c);
                    }
                }
                Token::StringLiteral(string)
            }
            // 그 외의 경우 키워드로 간주합니다.
            _ => {
                let mut keyword = String::new();
                keyword.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() || c == ';' || c == '(' || c == ')' || c == '"' {
                        break;
                    } else {
                        keyword.push(chars.next().unwrap());
                    }
                }
                Token::Keyword(keyword)
            }
        };
        // 생성된 토큰을 벡터에 추가합니다.
        tokens.push(token);
    }

    // 토큰 벡터를 반환합니다.
    tokens
}

// AST 열거형
enum AST {
    PrintStatement(String),
}

// 토큰 벡터를 입력으로 받아 AST 벡터를 반환하는 함수
fn syntax_analysis(tokens: Vec<Token>) -> Vec<AST> {
    // 반환할 AST 벡터를 초기화합니다.
    let mut ast = Vec::new();

    // 토큰 벡터에 대한 이터레이터를 생성하고 peekable로 변환합니다.
    let mut tokens = tokens.into_iter().peekable();

    // 토큰 벡터의 모든 토큰을 순회합니다.
    while let Some(token) = tokens.next() {
        // 토큰이 키워드인 경우에만 처리합니다.
        if let Token::Keyword(keyword) = token {
            // 키워드가 "System.out.println"인 경우에만 처리합니다.
            if keyword == "System.out.println" {
                // 다음 3개 토큰을 가져와 적절한 문장 구조인지 확인합니다.
                if let Some(Token::OpenParenthesis) = tokens.next() {
                    if let Some(Token::StringLiteral(s)) = tokens.next() {
                        if let Some(Token::CloseParenthesis) = tokens.next() {
                            ast.push(AST::PrintStatement(s));
                        }
                    }
                }
            }
        }
    }
    // AST 벡터를 반환합니다.
    ast
}

// 이 함수는 AST 벡터를 받아 JavaScript 코드를 문자열로 반환합니다.
fn generate_code(ast: Vec<AST>) -> String {
    // 반환할 JavaScript 코드 문자열을 초기화합니다.
    let mut js_code = String::new();

    // AST 벡터를 순회하며 각 노드에 대한 JavaScript 코드를 생성합니다.
    for node in ast {
        match node {
            // 노드가 PrintStatement인 경우
            AST::PrintStatement(s) => {
                // 해당 문자열을 출력하는 JavaScript 코드를 생성합니다.
                // 예: `console.log("이리오너라!");`
                js_code.push_str(&format!("console.log(\"{}\");\n", s));
            },
        }
    }

    // 생성된 JavaScript 코드를 반환합니다.
    js_code
}

// std::fs 모듈을 사용합니다.
use std::fs;

// compile 함수를 정의합니다.
// 이 함수는 Java 파일의 경로와 JavaScript 파일의 경로를 입력 받아,
// Java 코드를 JavaScript 코드로 컴파일하고 결과를 파일에 저장합니다.
fn compile(java_file_path: &str, js_file_path: &str) -> std::io::Result<()> {
    // Java 파일을 읽습니다.
    let java_code = fs::read_to_string(java_file_path)?;

    // 읽어온 Java 코드에 대해 어휘 분석을 수행합니다.
    let tokens = lexical_analysis(&java_code);

    // 어휘 분석을 통해 얻은 토큰들에 대해 구문 분석을 수행합니다.
    let ast = syntax_analysis(tokens);

    // 구문 분석을 통해 얻은 추상 구문 트리를 바탕으로 JavaScript 코드를 생성합니다.
    let js_code = generate_code(ast);

    // 생성된 JavaScript 코드를 파일에 씁니다.
    fs::write(js_file_path, &js_code)?;

    // 함수가 성공적으로 완료되었음을 나타내는 Ok(())를 반환합니다.
    Ok(())
}

fn main() {
    match compile("input.java", "output.js") {
        Ok(()) => println!("Compilation successful!"),
        Err(err) => println!("Error during compilation: {}", err),
    }
}
