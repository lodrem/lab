#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use macros::Builder;

    #[test]
    fn test_create_builder() {
        #[derive(Builder)]
        pub struct Command {
            executable: String,
            args: Vec<String>,
            env: Vec<String>,
            current_dir: String,
        }
        let builder = Command::builder();

        let _ = builder;
    }

    #[test]
    fn test_field_setter() {
        #[derive(Builder)]
        pub struct Command {
            executable: String,
            args: Vec<String>,
            env: Vec<String>,
            current_dir: String,
        }
        let mut builder = Command::builder();
        builder.executable("cargo".to_owned());
        builder.args(vec!["build".to_owned(), "--release".to_owned()]);
        builder.env(vec![]);
        builder.current_dir("..".to_owned());
    }

    #[test]
    fn test_build() {
        #[derive(Builder)]
        pub struct Command {
            executable: String,
            args: Vec<String>,
            env: Vec<String>,
            current_dir: String,
        }
        let mut builder = Command::builder();
        builder.executable("cargo".to_owned());
        builder.args(vec!["build".to_owned(), "--release".to_owned()]);
        builder.env(vec![]);
        builder.current_dir("..".to_owned());

        let command = builder.build().unwrap();
        assert_eq!(command.executable, "cargo");
    }

    #[test]
    fn test_method_chaining() {
        #[derive(Builder)]
        pub struct Command {
            executable: String,
            args: Vec<String>,
            env: Vec<String>,
            current_dir: String,
        }
        let command = Command::builder()
            .executable("cargo".to_owned())
            .args(vec!["build".to_owned(), "--release".to_owned()])
            .env(vec![])
            .current_dir("..".to_owned())
            .build()
            .unwrap();

        assert_eq!(command.executable, "cargo");
    }

    #[test]
    fn test_optional() {
        #[derive(Builder)]
        pub struct Command {
            executable: String,
            args: Vec<String>,
            env: Vec<String>,
            current_dir: Option<String>,
        }

        let command = Command::builder()
            .executable("cargo".to_owned())
            .args(vec!["build".to_owned(), "--release".to_owned()])
            .env(vec![])
            .build()
            .unwrap();
        assert!(command.current_dir.is_none());

        let command = Command::builder()
            .executable("cargo".to_owned())
            .args(vec!["build".to_owned(), "--release".to_owned()])
            .env(vec![])
            .current_dir("..".to_owned())
            .build()
            .unwrap();
        assert!(command.current_dir.is_some());
    }
}
