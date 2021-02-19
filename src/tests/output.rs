use crate::{build_out_path, opt};

#[test]
fn out() {
    let opt = opt::Opt{ 
        input: "./".to_string(),
        output_dir: Some("src/".to_string()),
        width: 1920,
        height: 1080
    };

    let out_path = build_out_path(&opt);
    assert!(out_path.is_some(), true);
    let out_path = out_path.unwrap();
    assert!(out_path.to_str().is_some(), true);
    let out_path = out_path.to_str().unwrap();
    assert_eq!(out_path, "src/");
}

#[test]
fn out_w_dir_input() {
    let opt = opt::Opt{ 
        input: "src/".to_string(),
        output_dir: None,
        width: 1920,
        height: 1080
    };

    let out_path = build_out_path(&opt);
    assert!(out_path.is_some(), true);
    let out_path = out_path.unwrap();
    assert!(out_path.to_str().is_some(), true);
    let out_path = out_path.to_str().unwrap();
    assert_eq!(out_path, "src/");
}

#[test]
fn out_w_file_input() {
    let opt = opt::Opt{ 
        input: "src/main.rs".to_string(),
        output_dir: None,
        width: 1920,
        height: 1080
    };

    let out_path = build_out_path(&opt);
    assert!(out_path.is_some(), true);
    let out_path = out_path.unwrap();
    assert!(out_path.to_str().is_some(), true);
    let out_path = out_path.to_str().unwrap();
    assert_eq!(out_path, "src");
}

#[test]
fn out_w_root_file_input() {
    let opt = opt::Opt{ 
        input: "README.md".to_string(),
        output_dir: None,
        width: 1920,
        height: 1080
    };

    let out_path = build_out_path(&opt);
    assert!(out_path.is_some(), true);
    let out_path = out_path.unwrap();
    assert!(out_path.to_str().is_some(), true);
    let out_path = out_path.to_str().unwrap();
    assert_eq!(out_path, "./");
}

#[test]
fn out_w_root_dir_input() {
    let opt = opt::Opt{ 
        input: "./".to_string(),
        output_dir: None,
        width: 1920,
        height: 1080
    };

    let out_path = build_out_path(&opt);
    assert!(out_path.is_some(), true);
    let out_path = out_path.unwrap();
    assert!(out_path.to_str().is_some(), true);
    let out_path = out_path.to_str().unwrap();
    assert_eq!(out_path, "./");
}