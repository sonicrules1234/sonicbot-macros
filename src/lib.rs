use proc_macro::TokenStream;

#[proc_macro]
pub fn backinsert(item: TokenStream) -> TokenStream {
    let mut code = String::new();
    let mut items: Vec<String> = Vec::new();
    for a in item {
        //println!("{:?}", a);
        //println!("{:?}", a);
        //for token in a.as_.stream {
        //}
        match a {
            proc_macro::TokenTree::Punct(_y) => (),
            proc_macro::TokenTree::Group(z) => {
                //let last = z.stream().into_iter().last().unwrap();
                for token in z.stream() {
                    let x: String = match token {
                        proc_macro::TokenTree::Punct(_y) => "".to_string(),
                        proc_macro::TokenTree::Group(_y) => "".to_string(),
                        proc_macro::TokenTree::Ident(y) => y.to_string(),
                        proc_macro::TokenTree::Literal(y) => y.to_string(),
                    };
                    if x != String::new() {
                        //if x.starts_with("\"") && x.ends_with("\"") {
                        //    items.push(x[1..(x.len() - 1)].to_string());
                        //} else {
                        //if x != last.to_string() {
                            items.push(x);
                        //} else {
                        //    items.push(x[1..(x.len() - 1)].to_string());
                        //}

                        //println!("{}", x[1..(x.len() - 1)].to_string());
                    }
                }
                
            },
            proc_macro::TokenTree::Ident(_y) => (),
            proc_macro::TokenTree::Literal(_y) => (),
        };    
        //}
        

        //let x: String = token.to_string();
        //code.push_str(format!("let item{} = ").as_str())
        
        
    }
    //println!("1");
    let lastitemindex = items.len() - 1;
    //let lastvalue = items.last().unwrap();
    let firstvalue = &items.clone()[0];
    //println!("2");
    for (num, token) in items.clone().into_iter().enumerate() {
        if num < (lastitemindex - 1) && num != 0 {
            code.push_str(format!("let mut item{} = item{}.get({}).unwrap();\n", num, num - 1, token).as_str());
        } else if num == 0 {
            code.push_str(format!("let mut item0 = xobj.get({}).unwrap();\n", token).as_str())
        }
    }
    //println!("3");
    //code.push_str(format!("item{}.insert(\"{}\", {});\n", lastitemindex - 1, items.clone()[lastitemindex - 2], lastvalue).as_str());
    //println!("3.5");
    //println!("{}", code);
    items.reverse();
    /*for (num, token) in items.clone().into_iter().enumerate() {
        let goodnum = lastitemindex - num;
        //println!("Goodnum {}", goodnum);
        if goodnum  2 {
            code.push_str(format!("item{}.insert(\"{}\", item{}.value);\n", num, token, goodnum - 2).as_str());
        }
    }*/
    for num in 0..(lastitemindex + 1) {
        let goodnum = lastitemindex - num;
        //println!("Goodnum {}", goodnum);
        if num == 1 {
            code.push_str(format!("item{}.insert({}, {});\n", goodnum - 1, items.clone()[1], items.clone()[0]).as_str());
        } else if num > 2 {
            code.push_str(format!("item{}.insert({}, item{}.value);\n", goodnum, items.clone()[num - 1], goodnum + 1).as_str());
        }
    }
    //println!("4");
    code.push_str(format!("xobj.insert({}, item0.value);\n", firstvalue, ).as_str());
    //println!("{}", code);
    code.as_str().parse().unwrap()
}

#[proc_macro]
pub fn pluginmacro(_item: TokenStream) -> TokenStream {
    let mut modulelist: Vec<String> = Vec::new();
    let potentialfilesresults = glob::glob("src/plugins/*.rs").unwrap();//.collect::<glob::GlobResult>().unwrap();
    //println!("1");
    let mut code = r#"use std::collections::HashMap;
use crate::parser::IRCMessage;
use sonicobject::SonicObject;
use crate::msgfmts;
"#.to_string();
    for pfr in potentialfilesresults {
        let filename: String = pfr.unwrap().as_path().file_name().unwrap().to_str().unwrap().to_string();
        let modulename = filename.split(".").collect::<Vec<&str>>()[0];
        if modulename != "mod" {
            modulelist.push(modulename.to_string());
        }
    }
    for modulename in modulelist.clone() {
        code.push_str(format!("mod {};\n", modulename).as_str());
    }
    code.push_str(r#"pub struct ModList {
        pub modnames: Vec<String>,
        pub mainfunctions: HashMap<String, Box<dyn Fn(IRCMessage, &mut SonicObject, SonicObject, &mut SonicObject) -> Vec<msgfmts::Message>>>,
        pub permissions: HashMap<String, u8>,
        pub syntaxes: HashMap<String, String>,
        pub helps: HashMap<String, String>,
        pub minargs: HashMap<String, u8>,
    }
    impl ModList {
        pub fn new() -> Self {
            let mut mainfunctions: HashMap<String, Box<dyn Fn(IRCMessage, &mut SonicObject, SonicObject, &mut SonicObject) -> Vec<msgfmts::Message>>> = HashMap::new();
            let mut permissions: HashMap<String, u8> = HashMap::new();
            let mut syntaxes: HashMap<String, String> = HashMap::new();
            let mut helps: HashMap<String, String> = HashMap::new();
            let mut minargs: HashMap<String, u8> = HashMap::new();
            "#);
    for modulename in modulelist {
        code.push_str(format!("mainfunctions.insert(\"{}\".to_string(), Box::new({}::main));\n", modulename, modulename).as_str());
        code.push_str(format!("permissions.insert(\"{}\".to_string(), {}::permissionlevel());\n", modulename, modulename).as_str());
        code.push_str(format!("syntaxes.insert(\"{}\".to_string(), {}::syntax());\n", modulename, modulename).as_str());
        code.push_str(format!("helps.insert(\"{}\".to_string(), {}::help());\n", modulename, modulename).as_str());
        code.push_str(format!("minargs.insert(\"{}\".to_string(), {}::minargs());\n", modulename, modulename).as_str());
    }
    code.push_str(r#"        let modnamesstr = mainfunctions.keys().collect::<Vec<&String>>();
    let mut modnames: Vec<String> = Vec::new();
    for modname in modnamesstr {
        modnames.push(modname.to_string());
    }
    Self {
        modnames: modnames,
        mainfunctions: mainfunctions,
        permissions: permissions,
        syntaxes: syntaxes,
        helps: helps,
        minargs: minargs,
    }}}"#);
    //println!("{}", code.as_str().to_string());
    code.as_str().parse().unwrap()
    //"".parse().unwrap()
}