// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

//! Dump AST node types for all supported languages.
//! Run with: cargo run --example dump_ast --features all

fn print_tree(node: &tree_sitter::Node, source: &[u8], indent: usize) {
    let prefix = " ".repeat(indent);
    let text = node.utf8_text(source).unwrap_or("<binary>");
    let short_text: String = text.chars().take(60).collect();
    let field = node.parent().and_then(|p| {
        (0..p.child_count()).find_map(|i| {
            if p.child(i).map(|c| c.id()) == Some(node.id()) {
                p.field_name_for_child(i as u32)
            } else {
                None
            }
        })
    });
    let field_str = field.map(|f| format!(" [field: {}]", f)).unwrap_or_default();
    println!(
        "{}{} ({}){}  | {}",
        prefix,
        node.kind(),
        node.kind_id(),
        field_str,
        short_text.replace('\n', "\\n")
    );
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        print_tree(&child, source, indent + 2);
    }
}

fn parse_and_dump(lang_name: &str, language: tree_sitter::Language, source: &str) {
    println!("\n{}", "=".repeat(80));
    println!("=== {} ===", lang_name);
    println!("{}", "=".repeat(80));
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(&language).unwrap();
    let tree = parser.parse(source, None).unwrap();
    print_tree(&tree.root_node(), source.as_bytes(), 0);
}

fn main() {
    // Python
    #[cfg(feature = "python")]
    parse_and_dump(
        "Python",
        tree_sitter_python::LANGUAGE.into(),
        r#"
def foo(x):
    if x > 0:
        for i in range(x):
            if i > 2 and i < 5 or i == 0:
                pass
    elif x == 0:
        while True:
            break
    else:
        try:
            bar()
        except ValueError:
            pass
    return lambda y: y + 1

class MyClass:
    def method(self):
        pass
"#,
    );

    // JavaScript
    #[cfg(feature = "javascript")]
    parse_and_dump(
        "JavaScript",
        tree_sitter_javascript::LANGUAGE.into(),
        r#"
function foo(x) {
    if (x > 0) {
        for (let i = 0; i < x; i++) {
            if (i > 2 && i < 5 || i === 0) {
                console.log(i);
            }
        }
    } else if (x === 0) {
        while (true) { break; }
    } else {
        try {
            bar();
        } catch (e) {
            console.error(e);
        }
    }
    const f = (y) => y + 1;
    const g = function* () { yield 1; };
    return x > 0 ? x : -x;
}

class MyClass {
    method() { }
    get prop() { return 1; }
}
"#,
    );

    // TypeScript
    #[cfg(feature = "typescript")]
    parse_and_dump(
        "TypeScript",
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        r#"
function foo(x: number): number {
    if (x > 0) {
        for (let i = 0; i < x; i++) {
            if (i > 2 && i < 5 || i === 0) { }
        }
    } else if (x === 0) {
        while (true) { break; }
    } else {
        try { bar(); } catch (e) { }
    }
    const f = (y: number): number => y + 1;
    return x > 0 ? x : -x;
}

class MyClass {
    method(): void { }
}
"#,
    );

    // Java
    #[cfg(feature = "java")]
    parse_and_dump(
        "Java",
        tree_sitter_java::LANGUAGE.into(),
        r#"
class MyClass {
    int foo(int x) {
        if (x > 0) {
            for (int i = 0; i < x; i++) {
                if (i > 2 && i < 5 || i == 0) { }
            }
        } else if (x == 0) {
            while (true) { break; }
        } else {
            try { bar(); } catch (Exception e) { }
        }
        switch (x) {
            case 1: break;
            default: break;
        }
        Runnable r = () -> System.out.println("hi");
        return x > 0 ? x : -x;
    }
}
"#,
    );

    // C#
    #[cfg(feature = "csharp")]
    parse_and_dump(
        "C#",
        tree_sitter_c_sharp::LANGUAGE.into(),
        r#"
class MyClass {
    int Foo(int x) {
        if (x > 0) {
            for (int i = 0; i < x; i++) {
                if (i > 2 && i < 5 || i == 0) { }
            }
            foreach (var item in list) { }
        } else if (x == 0) {
            while (true) { break; }
            do { } while (false);
        } else {
            try { Bar(); } catch (Exception e) { }
        }
        switch (x) {
            case 1: break;
            default: break;
        }
        Action a = () => Console.WriteLine("hi");
        Func<int, int> f = x => x + 1;
        return x > 0 ? x : -x;
    }
}
"#,
    );

    // C++
    #[cfg(feature = "cpp")]
    parse_and_dump(
        "C++",
        tree_sitter_cpp::LANGUAGE.into(),
        r#"
class MyClass {
public:
    int foo(int x) {
        if (x > 0) {
            for (int i = 0; i < x; i++) {
                if (i > 2 && i < 5 || i == 0) { }
            }
        } else if (x == 0) {
            while (true) { break; }
        } else {
            try { bar(); } catch (const std::exception& e) { }
        }
        switch (x) {
            case 1: break;
            default: break;
        }
        auto f = [](int y) { return y + 1; };
        return x > 0 ? x : -x;
    }
};
"#,
    );

    // C
    #[cfg(feature = "c")]
    parse_and_dump(
        "C",
        tree_sitter_c::LANGUAGE.into(),
        r#"
int foo(int x) {
    if (x > 0) {
        for (int i = 0; i < x; i++) {
            if (i > 2 && i < 5 || i == 0) { }
        }
    } else if (x == 0) {
        while (1) { break; }
        do { } while (0);
    } else {
        goto end;
    }
    switch (x) {
        case 1: break;
        default: break;
    }
    return x > 0 ? x : -x;
    end:
    return 0;
}
"#,
    );

    // Go
    #[cfg(feature = "go")]
    parse_and_dump(
        "Go",
        tree_sitter_go::LANGUAGE.into(),
        r#"
package main

func foo(x int) int {
    if x > 0 {
        for i := 0; i < x; i++ {
            if i > 2 && i < 5 || i == 0 { }
        }
    } else if x == 0 {
        for { break }
    } else {
        defer func() { recover() }()
    }
    switch x {
    case 1:
    default:
    }
    f := func(y int) int { return y + 1 }
    _ = f
    return x
}

type MyStruct struct{}

func (m *MyStruct) Method() {}
"#,
    );

    // PHP
    #[cfg(feature = "php")]
    parse_and_dump(
        "PHP",
        tree_sitter_php::LANGUAGE_PHP.into(),
        r#"<?php
function foo($x) {
    if ($x > 0) {
        for ($i = 0; $i < $x; $i++) {
            if ($i > 2 && $i < 5 || $i == 0) { }
        }
        foreach ($list as $item) { }
    } elseif ($x == 0) {
        while (true) { break; }
        do { } while (false);
    } else {
        try { bar(); } catch (Exception $e) { }
    }
    switch ($x) {
        case 1: break;
        default: break;
    }
    $f = function($y) { return $y + 1; };
    $g = fn($y) => $y + 1;
    return $x > 0 ? $x : -$x;
}

class MyClass {
    public function method() { }
}
?>"#,
    );
}
