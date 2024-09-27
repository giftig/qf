use super::*;

fn default_args() -> Args {
    Args {
        mode: SearchMode::AllUsage,
        output_style: OutputStyle::Auto,
        lang: Language::Auto,
        first_hit: false,
        list: false,
        term: "OogaBooga".to_string(),
    }
}

#[test]
fn validate_success_simple_args() {
    let args = default_args();
    let res = args.validate().unwrap();

    assert_eq!(res, ());
}

#[test]
fn validate_fail_illegal_clean_imports() {
    let args = Args {
        output_style: OutputStyle::Import,
        ..default_args()
    };

    let res = args.validate().err().unwrap();
    assert_eq!(res, ArgError::IllegalStyleImport);
}

#[test]
fn validate_fail_file_search_bad_output_style() {
    for style in vec![OutputStyle::Coords, OutputStyle::Quickfix] {
        let args = Args {
            mode: SearchMode::File,
            output_style: style,
            ..default_args()
        };

        let res = args.validate().err().unwrap();
        assert_eq!(res, ArgError::IllegalFileOutputMode);
    }

}
